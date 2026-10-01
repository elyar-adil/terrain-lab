import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import type { CityScene } from "./city/cityScene";
import { Terrain3D } from "./components/Terrain3D";
import type { GenerationResult, SimulationConfig } from "./types";
import "./styles.css";

declare global {
  interface Window {
    __RENDER_READY__?: boolean;
  }
}

interface Fixture {
  config: SimulationConfig;
  result: GenerationResult;
}

// The product renderer intentionally animates at display refresh rate. Under
// SwiftShader, advancing several seconds of virtual time at that rate turns a
// four-frame smoke test into hundreds of expensive software-rendered frames.
// Throttle only the headless harness: simulation clocks still advance in real
// elapsed time, so frame pairs continue to verify water/cloud evolution.
const requestedFps = Number(new URLSearchParams(window.location.search).get("fps"));
const validationFps = Number.isFinite(requestedFps)
  ? Math.min(30, Math.max(1, requestedFps))
  : 6;
const validationFrameMs = 1000 / validationFps;
window.requestAnimationFrame = (callback: FrameRequestCallback) => window.setTimeout(
  () => callback(performance.now()),
  validationFrameMs,
);
window.cancelAnimationFrame = (handle: number) => window.clearTimeout(handle);

function Harness() {
  const [fixture, setFixture] = useState<Fixture | null>(null);
  const [cityScene, setCityScene] = useState<CityScene | null>(null);
  const mode = new URLSearchParams(window.location.search).get("mode") === "satellite"
    ? "satellite"
    : "3d";
  // `focus=-1` frames the whole world; `focus=N` flies to city N the same way
  // the app's "跳转城市" selector does, so screenshots audit the exact view a
  // user lands on.
  const params = new URLSearchParams(window.location.search);
  const focusParam = Number(params.get("focus"));
  const spanOverride = Number(params.get("span"));
  const focus = fixture && Number.isFinite(focusParam) && focusParam !== 0
    ? (() => {
        const spanKm = Number.isFinite(spanOverride) && spanOverride > 0
          ? spanOverride
          : focusParam < 0
          ? fixture.config.worldSizeKm * 0.55
          : focusParam === 1 ? 6 : focusParam <= 4 ? 3.5 : 1.6;
        if (focusParam < 0) {
          // `fx`/`fy` (km) aim at any point; `pitch`/`az` (degrees) set the view
          // direction, 90 being straight down.
          const num = (name: string) => (params.has(name) ? Number(params.get(name)) : undefined);
          return {
            xKm: num("fx") ?? fixture.config.worldSizeKm / 2,
            yKm: num("fy") ?? fixture.config.worldSizeKm / 2,
            spanKm,
            nonce: 1,
            pitchDeg: num("pitch"),
            azimuthDeg: num("az"),
          };
        }
        const city = fixture.result.modernCities?.[focusParam - 1];
        if (!city?.nodes?.length) return null;
        let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
        for (const node of city.nodes) {
          minX = Math.min(minX, node.point.x_km);
          maxX = Math.max(maxX, node.point.x_km);
          minY = Math.min(minY, node.point.y_km);
          maxY = Math.max(maxY, node.point.y_km);
        }
        return {
          xKm: (minX + maxX) / 2,
          yKm: (minY + maxY) / 2,
          spanKm,
          nonce: 1,
        };
      })()
    : null;

  useEffect(() => {
    fetch("/render-fixture.json")
      .then((response) => response.json())
      .then(async (data: Fixture) => {
        // `?city=1` stands in for the desktop shell: it places the first scene of
        // public/city-scenes.json at the world centre, declares a matching
        // settlement so the terrain is levelled under it, and mounts it.
        if (new URLSearchParams(window.location.search).get("city")) {
          const scenes: CityScene[] = await (await fetch("/city-scenes.json")).json();
          const scene = scenes[0];
          const half = data.config.worldSizeKm / 2;
          scene.origin = [half, half];
          const [minX, minZ, maxX, maxZ] = scene.extentM;
          const points = [[minX, minZ], [maxX, maxZ]].map(([x, z]) => ({ point: { x_km: half + x / 1000, y_km: half + z / 1000 } }));
          (data.result as unknown as { modernCities: unknown[] }).modernCities = [{ nodes: points, edges: [], blocks: [], buildings: [], parcels: [], hdRoads: [], sdRoads: [], streets: [], lanes: [], connectors: [], junctions: [], compounds: [], trees: [] }];
          setCityScene(scene);
        }
        setFixture(data);
      });
  }, []);

  useEffect(() => {
    if (!fixture) return;
    const timer = window.setTimeout(() => {
      window.__RENDER_READY__ = true;
      document.documentElement.dataset.renderReady = "true";
    }, 2500);
    return () => window.clearTimeout(timer);
  }, [fixture]);

  return fixture
    ? <Terrain3D result={fixture.result} config={fixture.config} cameraMode={mode} cityFocus={focus} cityScene={cityScene} />
    : null;
}

// StrictMode deliberately mounts effects twice in development. That is useful
// for application diagnostics, but duplicates all WebGL allocations here and
// does not add coverage to a screenshot harness.
createRoot(document.getElementById("render-root")!).render(<Harness />);
