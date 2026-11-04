import { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import type { CityScene } from "../city/cityScene";
import { Terrain3D } from "../components/Terrain3D";
import type { GenerationResult, SimulationConfig } from "../types";
import { loadPayloads } from "./payload";
import "../styles.css";

/**
 * The single-file build of the viewer. The desktop shell supplies a generated
 * world and a city scene over IPC; here both are shipped inside the page (see
 * scripts/build-single.mjs), so nothing is fetched at run time.
 */
interface Fixture {
  config: SimulationConfig;
  result: GenerationResult;
}

type View = "world" | "city";

const barStyle: React.CSSProperties = {
  position: "fixed", top: 12, left: 12, right: 12, display: "flex", gap: 8,
  alignItems: "center", flexWrap: "wrap", pointerEvents: "none",
  font: "13px/1.3 system-ui, sans-serif", color: "#e8efe9",
};
const buttonStyle = (active: boolean): React.CSSProperties => ({
  pointerEvents: "auto", cursor: "pointer", padding: "6px 12px", borderRadius: 6,
  border: "1px solid rgba(255,255,255,0.28)", font: "inherit", color: "#fff",
  background: active ? "rgba(46,110,90,0.92)" : "rgba(11,16,15,0.72)",
});

function Web() {
  const [fixture, setFixture] = useState<Fixture | null>(null);
  const [cityScene, setCityScene] = useState<CityScene | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [mode, setMode] = useState<"3d" | "satellite">("3d");
  const [view, setView] = useState<View>("world");
  const [nonce, setNonce] = useState(1);

  useEffect(() => {
    loadPayloads()
      .then(({ fixture: data, city }) => {
        const scene = city as CityScene;
        const half = data.config.worldSizeKm / 2;
        scene.origin = [half, half];
        const [minX, minZ, maxX, maxZ] = scene.extentM;
        const nodes = [[minX, minZ], [maxX, maxZ]].map(([x, z]) => ({ point: { x_km: half + x / 1000, y_km: half + z / 1000 } }));
        (data.result as unknown as { modernCities: unknown[] }).modernCities = [
          { frame: { origin: { x_km: half, y_km: half }, rotationRadians: 0 }, nodes, edges: [], blocks: [], buildings: [], parcels: [], hdRoads: [], sdRoads: [], streets: [], lanes: [], connectors: [], junctions: [], compounds: [], trees: [] },
        ];
        setCityScene(scene);
        setFixture(data as Fixture);
      })
      .catch((thrown: Error) => setError(String(thrown.message ?? thrown)));
  }, []);

  if (error) return <div style={{ color: "#f88", padding: 24, font: "14px system-ui" }}>Could not load: {error}</div>;
  if (!fixture) return <div style={{ color: "#cfe0d6", padding: 24, font: "14px system-ui" }} id="loading">Unpacking the generated world...</div>;

  const half = fixture.config.worldSizeKm / 2;
  const focus = view === "city"
    ? { xKm: half, yKm: half, spanKm: 0.7, nonce }
    : { xKm: half, yKm: half, spanKm: fixture.config.worldSizeKm * 0.55, nonce };

  return (
    <>
      <div style={{ position: "fixed", inset: 0 }}>
      <Terrain3D result={fixture.result} config={fixture.config} cameraMode={mode} cityFocus={focus} cityScene={cityScene} />
      </div>
      <div style={barStyle}>
        <strong style={{ marginRight: 8 }}>Wind and Water Terrain Lab</strong>
        <button style={buttonStyle(view === "world")} onClick={() => { setView("world"); setNonce((n) => n + 1); }}>World</button>
        <button style={buttonStyle(view === "city")} onClick={() => { setView("city"); setNonce((n) => n + 1); }}>City</button>
        <button style={buttonStyle(mode === "3d")} onClick={() => setMode("3d")}>3D</button>
        <button style={buttonStyle(mode === "satellite")} onClick={() => setMode("satellite")}>Satellite</button>
        <span style={{ opacity: 0.75 }}>Drag to orbit, scroll to zoom.</span>
      </div>
      <div style={{ position: "fixed", bottom: 8, left: 12, right: 12, font: "12px system-ui, sans-serif", color: "rgba(255,255,255,0.8)", textShadow: "0 1px 2px #000", pointerEvents: "none" }}>
        Everything shown is a procedural simulation: terrain, water, vegetation and the city are generated, not photographed or surveyed.
      </div>
    </>
  );
}

createRoot(document.getElementById("web-root")!).render(<Web />);
