import { useEffect, useMemo, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/plugin-dialog";
import {
  exportTerrain,
  fetchCityScene,
  generateTerrain,
  loadProject,
  saveProject,
} from "./api";
import type {
  GenerationProgress,
  GenerationResult,
  SimulationConfig,
  Landform,
  TerrainPreset,
} from "./types";
import type { CityScene } from "./city/cityScene";
import { Terrain3D } from "./components/Terrain3D";
import { CityViewer } from "./city/CityViewer";

const PRESETS: Record<TerrainPreset, Pick<SimulationConfig, "rainfall" | "evaporation" | "windSpeed" | "windDirection"> & { name: string; description: string }> = {
  arid: { name: "干旱高山", description: "裸岩、冲积扇与干涸河谷", rainfall: 425, evaporation: 975, windSpeed: 11, windDirection: 35 },
  temperate: { name: "温带湿润", description: "森林、活跃河网与冲积平原", rainfall: 1275, evaporation: 600, windSpeed: 7, windDirection: 225 },
  glacial: { name: "高寒冰川", description: "冰斗、雪线与冰川侵蚀谷", rainfall: 750, evaporation: 250, windSpeed: 13, windDirection: 285 },
};

const LANDFORMS: Record<Landform, { name: string; description: string }> = {
  mountainRange: { name: "山脉", description: "造山带与深切河谷" },
  hills: { name: "丘陵", description: "低缓山丘与谷地" },
  plains: { name: "平原", description: "低起伏与大型河网" },
  plateau: { name: "高原", description: "台地、陡崖与峡谷" },
  coastal: { name: "海岸", description: "大陆边缘与海湾" },
  archipelago: { name: "群岛", description: "岛链、海峡与浅海" },
};

const DEFAULT_CONFIG: SimulationConfig = {
  seed: 284735,
  preset: "temperate",
  landform: "mountainRange",
  gridSize: 512,
  worldSizeKm: 80,
  rainfall: PRESETS.temperate.rainfall,
  evaporation: PRESETS.temperate.evaporation,
  windSpeed: PRESETS.temperate.windSpeed,
  windDirection: PRESETS.temperate.windDirection,
  sunAzimuth: 235,
  sunElevation: 42,
  haze: 2.5,
  cloudCoverage: 35,
  cloudSpeed: 24,
};

type AnalysisLayer = keyof GenerationResult["analysisPreviews"];
type ViewMode = "satellite" | "3d" | "analysis" | "city";

/** Camera framings for the dedicated metre-scale city view (CityViewer presets). */
const CITY_VIEW_PRESETS: Array<{ id: string; label: string }> = [
  { id: "street", label: "街道" },
  { id: "junction", label: "路口" },
  { id: "tower", label: "塔仰视" },
  { id: "aerial", label: "鸟瞰" },
  { id: "skyline", label: "天际线" },
];

const ANALYSIS_LAYERS: Record<AnalysisLayer, string> = {
  discharge: "汇流量",
  lake: "湖泊",
  wetland: "湿地",
  floodplain: "洪泛区",
  basin: "流域",
  riverOrder: "河流级序",
  geology: "岩性分区",
  soilDepth: "土壤深度",
  landCover: "地表覆盖",
  travelCost: "通行成本",
  hazard: "灾害风险",
  settlementSuitability: "聚落适宜度",
  agriculturalSuitability: "农业适宜度",
  urbanLand: "城市占地",
  cultivatedLand: "耕作带",
  infrastructure: "聚落与道路",
};

function Slider({ label, value, min, max, step = 1, unit = "", onChange }: {
  label: string;
  value: number;
  min: number;
  max: number;
  step?: number;
  unit?: string;
  onChange: (value: number) => void;
}) {
  return (
    <label className="control">
      <span><b>{label}</b><output>{value}{unit}</output></span>
      <input type="range" value={value} min={min} max={max} step={step} onChange={(event) => onChange(Number(event.target.value))} />
    </label>
  );
}

function App() {
  const [config, setConfig] = useState(DEFAULT_CONFIG);
  const [result, setResult] = useState<GenerationResult | null>(null);
  const [busy, setBusy] = useState(false);
  const [status, setStatus] = useState("等待生成");
  const [progress, setProgress] = useState(0);
  const [exportSize, setExportSize] = useState(4096);
  const [error, setError] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("3d");
  const [analysisLayer, setAnalysisLayer] = useState<AnalysisLayer>("discharge");
  const [cityFocus, setCityFocus] = useState<{ xKm: number; yKm: number; spanKm: number; nonce: number } | null>(null);
  const [citySceneIndex, setCitySceneIndex] = useState<number | null>(null);
  const [cityPreset, setCityPreset] = useState<string>("street");
  const viewportRef = useRef<HTMLElement>(null);

  useEffect(() => {
    let dispose: (() => void) | undefined;
    listen<GenerationProgress>("terrain-progress", (event) => {
      setStatus(event.payload.stage);
      setProgress(event.payload.progress);
    }).then((fn) => { dispose = fn; });
    return () => dispose?.();
  }, []);

  useEffect(() => {
    const switchView = (event: KeyboardEvent) => {
      if (!result || event.ctrlKey || event.altKey || event.metaKey) return;
      const target = event.target as HTMLElement | null;
      if (target?.matches("input, select, textarea")) return;
      if (event.key.toLowerCase() === "s") setViewMode("satellite");
      if (event.key === "3") setViewMode("3d");
    };
    window.addEventListener("keydown", switchView);
    return () => window.removeEventListener("keydown", switchView);
  }, [result]);

  const metresPerPixel = useMemo(
    () => ((config.worldSizeKm * 1000) / config.gridSize).toFixed(1),
    [config.gridSize, config.worldSizeKm],
  );

  // Settlement-centred viewpoints: the payload carries each city's graph in
  // world kilometres, so the centre is the bounding-box mid of its nodes and
  // a jump flies the camera straight there at a street-to-district span.
  const cityOptions = useMemo(() => {
    if (!result?.modernCities?.length) return [];
    const labelled = result.modernCities.map((city, index) => {
      let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
      for (const node of city.nodes) {
        minX = Math.min(minX, node.point.x_km); maxX = Math.max(maxX, node.point.x_km);
        minY = Math.min(minY, node.point.y_km); maxY = Math.max(maxY, node.point.y_km);
      }
      const xKm = (minX + maxX) / 2;
      const yKm = (minY + maxY) / 2;
      const label = index === 0 ? "中心城（县城）"
        : index < 4 ? `镇区 ${index}`
        : `村庄 ${index - 3}`;
      return { label, xKm, yKm, spanKm: index === 0 ? 6 : index < 4 ? 3.5 : 1.6 };
    });
    return [
      { label: "全域视角", xKm: config.worldSizeKm / 2, yKm: config.worldSizeKm / 2, spanKm: config.worldSizeKm * 0.55 },
      ...labelled,
    ];
  }, [result, config.worldSizeKm]);

  /**
   * The metre-scale city view fetches its scene on demand.
   *
   * A scene is roughly 160 MB of base64 vertex buffers — 2.4 M vertices for a
   * 1.6 km city, before base64 — so it is no longer part of the `generate`
   * response. Handing the webview three of those unconditionally killed it on
   * arrival, which is what "click generate and it crashes" was.
   *
   * `cityOptions[i + 1]` (i >= 0, skipping the 全域视角 entry) is exactly
   * `city_scene(i)`, because the scenes are built in the same settlement pass as
   * the city list and in the same order.
   */
  const [activeCityScene, setActiveCityScene] = useState<CityScene | null>(null);
  const [citySceneLoading, setCitySceneLoading] = useState(false);

  useEffect(() => {
    if (viewMode !== "city" || citySceneIndex === null) {
      setActiveCityScene(null);
      return;
    }
    let cancelled = false;
    setCitySceneLoading(true);
    fetchCityScene(citySceneIndex)
      .then((scene) => {
        if (!cancelled) setActiveCityScene(scene);
      })
      .catch((error: Error) => {
        if (!cancelled) setError(error.message);
      })
      .finally(() => {
        if (!cancelled) setCitySceneLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [viewMode, citySceneIndex]);

  const jumpToCityOption = (option: (typeof cityOptions)[number], optionIndex: number) => {
    setCityFocus({ xKm: option.xKm, yKm: option.yKm, spanKm: option.spanKm, nonce: Date.now() });
    const sceneIndex = optionIndex - 1;
    if (sceneIndex >= 0) {
      setCitySceneIndex(sceneIndex);
      setViewMode("city");
    }
  };

  const update = <K extends keyof SimulationConfig>(key: K, value: SimulationConfig[K]) => {
    setConfig((current) => ({ ...current, [key]: value }));
  };

  const choosePreset = (preset: TerrainPreset) => {
    const values = PRESETS[preset];
    setConfig((current) => ({
      ...current,
      preset,
      rainfall: values.rainfall,
      evaporation: values.evaporation,
      windSpeed: values.windSpeed,
      windDirection: values.windDirection,
    }));
  };

  const chooseLandform = (landform: Landform) => update("landform", landform);

  const randomizeSeed = () => update("seed", Math.floor(Math.random() * 2_147_483_647));

  const runGeneration = async () => {
    setBusy(true);
    setError(null);
    setProgress(0.02);
    setStatus("初始化原生计算核心");
    try {
      const next = await generateTerrain(config);
      setResult(next);
      setViewMode("3d");
      setCitySceneIndex(null);
      setProgress(1);
      setStatus(`完成 · ${(next.elapsedMs / 1000).toFixed(2)} 秒`);
    } catch (reason) {
      setError(String(reason));
      setStatus("生成失败");
    } finally {
      setBusy(false);
    }
  };

  const runExport = async () => {
    const outputPath = await save({
      title: "导出卫星正射影像",
      defaultPath: `wind-water-${config.preset}-${config.seed}-${exportSize}.png`,
      filters: [{ name: "PNG image", extensions: ["png"] }],
    });
    if (!outputPath) return;
    setBusy(true);
    setError(null);
    setStatus(`渲染 ${exportSize} × ${exportSize} 影像`);
    setProgress(0.02);
    try {
      await exportTerrain(config, outputPath, exportSize);
      setProgress(1);
      setStatus(`已导出 · ${outputPath}`);
    } catch (reason) {
      setError(String(reason));
      setStatus("导出失败");
    } finally {
      setBusy(false);
    }
  };

  const saveCurrentProject = async () => {
    const outputPath = await save({
      title: "保存地貌项目",
      defaultPath: `wind-water-${config.seed}.fwl`,
      filters: [{ name: "Wind & Water project", extensions: ["fwl"] }],
    });
    if (!outputPath) return;
    try {
      await saveProject({ schemaVersion: 1, name: `Terrain ${config.seed}`, config }, outputPath);
      setStatus(`项目已保存 · ${outputPath}`);
      setError(null);
    } catch (reason) {
      setError(String(reason));
    }
  };

  const openExistingProject = async () => {
    const inputPath = await open({
      title: "打开地貌项目",
      multiple: false,
      directory: false,
      filters: [{ name: "Wind & Water project", extensions: ["fwl"] }],
    });
    if (!inputPath) return;
    try {
      const project = await loadProject(inputPath);
      setConfig(project.config);
      setResult(null);
      setStatus(`已打开 · ${project.name}`);
      setError(null);
    } catch (reason) {
      setError(String(reason));
    }
  };

  const toggleFullscreen = async () => {
    if (!viewportRef.current) return;
    if (document.fullscreenElement) await document.exitFullscreen();
    else await viewportRef.current.requestFullscreen();
  };

  return (
    <main className="app-shell">
      <header className="topbar">
        <div className="brand">
          <span className="brand-mark">W/W</span>
          <div><h1>风与水</h1><p>无限地貌实验室 · Desktop Alpha</p></div>
        </div>
        <div className="top-actions">
          <span className="engine-badge"><i /> RUST NATIVE</span>
          <button className="ghost" onClick={openExistingProject} disabled={busy}>打开</button>
          <button className="ghost" onClick={saveCurrentProject} disabled={busy}>保存</button>
          <button className="ghost" onClick={randomizeSeed} disabled={busy}>随机种子</button>
          <button className="primary" onClick={runGeneration} disabled={busy}>{busy ? "计算中…" : "生成地貌"}</button>
        </div>
      </header>

      <section className="workspace">
        <aside className="sidebar left-panel">
          <section>
            <div className="section-title"><span>地貌骨架</span><em>01</em></div>
            <div className="landform-grid">
              {(Object.keys(LANDFORMS) as Landform[]).map((id) => (
                <button key={id} className={`landform-card ${config.landform === id ? "active" : ""}`} onClick={() => chooseLandform(id)} disabled={busy}>
                  <strong>{LANDFORMS[id].name}</strong><small>{LANDFORMS[id].description}</small>
                </button>
              ))}
            </div>
          </section>

          <section>
            <div className="section-title"><span>气候环境</span><em>02</em></div>
            <div className="preset-grid">
              {(Object.keys(PRESETS) as TerrainPreset[]).map((id) => (
                <button key={id} className={`preset-card ${config.preset === id ? "active" : ""}`} onClick={() => choosePreset(id)} disabled={busy}>
                  <strong>{PRESETS[id].name}</strong><small>{PRESETS[id].description}</small>
                </button>
              ))}
            </div>
          </section>

          <section>
            <div className="section-title"><span>计算域</span><em>03</em></div>
            <label className="select-control"><span>模拟网格</span><select value={config.gridSize} onChange={(e) => update("gridSize", Number(e.target.value))} disabled={busy}>
              <option value={512}>512² · 快速</option><option value={1024}>1024² · 精细</option><option value={2048}>2048² · 专业</option>
            </select></label>
            <Slider label="区域尺度" value={config.worldSizeKm} min={20} max={160} step={10} unit=" km" onChange={(v) => update("worldSizeKm", v)} />
            <div className="domain-note"><span>物理分辨率</span><b>{metresPerPixel} m/cell</b></div>
            <label className="seed-input"><span>随机种子</span><input type="number" value={config.seed} onChange={(e) => update("seed", Number(e.target.value))} disabled={busy} /></label>
          </section>

          <section>
            <div className="section-title"><span>水文与气候</span><em>04</em></div>
            <Slider label="年降水" value={config.rainfall} min={100} max={2000} step={25} unit=" mm" onChange={(v) => update("rainfall", v)} />
            <Slider label="蒸散量" value={config.evaporation} min={0} max={1600} step={25} unit=" mm" onChange={(v) => update("evaporation", v)} />
            <Slider label="风速" value={config.windSpeed} min={0} max={30} step={0.5} unit=" m/s" onChange={(v) => update("windSpeed", v)} />
            <Slider label="风向" value={config.windDirection} min={0} max={359} unit="°" onChange={(v) => update("windDirection", v)} />
          </section>
        </aside>

        <section className="viewport" ref={viewportRef}>
          {result ? (viewMode === "city" ? <div style={{ position: "absolute", inset: 0 }}>{activeCityScene ? <CityViewer scene={activeCityScene} preset={cityPreset} /> : <div className="empty-state"><span>CITY VIEW</span><h2>{citySceneLoading ? "正在生成城市…" : "请选择一座城市"}</h2></div>}</div> : viewMode === "analysis" ? <img className="terrain-preview" src={result.analysisPreviews[analysisLayer]} alt={`${ANALYSIS_LAYERS[analysisLayer]}分析图层`} /> : <Terrain3D result={result} config={config} cameraMode={viewMode === "city" ? "3d" : viewMode} cityFocus={cityFocus} />) : <div className="empty-state"><div className="contour-art" /><span>80 × 80 KM SYNTHETIC EARTH</span><h2>创造一片不存在，却足够真实的土地</h2><p>原生 Rust 地貌演化 · 河网推演 · 多尺度卫星辐射合成</p><button className="primary large" onClick={runGeneration}>生成第一片地貌</button></div>}
          {result && <div className="view-switcher">{viewMode === "city" ? (<><select value={cityPreset} onChange={(event) => setCityPreset(event.target.value)}>{CITY_VIEW_PRESETS.map((preset) => <option key={preset.id} value={preset.id}>{preset.label}</option>)}</select><button onClick={() => setViewMode("3d")}>返回地形</button></>) : (<><button className={viewMode === "3d" ? "active" : ""} onClick={() => setViewMode("3d")}>3D 地形</button><button className={viewMode === "satellite" ? "active" : ""} onClick={() => setViewMode("satellite")}>动态卫星</button><select className={viewMode === "analysis" ? "active" : ""} value={analysisLayer} onChange={(event) => { setAnalysisLayer(event.target.value as AnalysisLayer); setViewMode("analysis"); }}><option disabled>分析图层</option>{(Object.keys(ANALYSIS_LAYERS) as AnalysisLayer[]).map((layer) => <option key={layer} value={layer}>{ANALYSIS_LAYERS[layer]}</option>)}</select><select value="" className="city-jump" onChange={(event) => { const index = Number(event.target.value); const option = cityOptions[index]; if (option) jumpToCityOption(option, index); }}><option disabled value="">跳转城市</option>{cityOptions.map((option, index) => <option key={option.label} value={index}>{option.label}</option>)}</select></>)}<button onClick={toggleFullscreen}>全屏</button></div>}
          <div className="viewport-meta"><span>{viewMode === "city" ? "CITY VIEW · 米制尺度" : viewMode === "3d" ? "3D TERRAIN · VERTICAL 1:1 · LIVE WEATHER" : viewMode === "analysis" ? `SEMANTIC LAYER · ${ANALYSIS_LAYERS[analysisLayer]}` : "LIVE ORTHOGRAPHIC · SAME WORLD · SAME TIME"}</span><span>{viewMode === "city" && activeCityScene ? `${citySceneIndex !== null ? cityOptions[citySceneIndex + 1]?.label ?? "城市" : "城市"} · ${Math.round(Math.max(activeCityScene.extentM[2] - activeCityScene.extentM[0], activeCityScene.extentM[3] - activeCityScene.extentM[1]))} M` : `${config.worldSizeKm} KM / ${config.gridSize} PX`}</span></div>
          {busy && <div className="progress-overlay"><div><span>{status}</span><b>{Math.round(progress * 100)}%</b></div><progress value={progress} max={1} /></div>}
        </section>

        <aside className="sidebar right-panel">
          <section>
            <div className="section-title"><span>光照与大气</span><em>04</em></div>
            <Slider label="太阳方位" value={config.sunAzimuth} min={0} max={359} unit="°" onChange={(v) => update("sunAzimuth", v)} />
            <Slider label="太阳高度" value={config.sunElevation} min={5} max={80} unit="°" onChange={(v) => update("sunElevation", v)} />
            <Slider label="大气霾度" value={config.haze} min={0} max={10} step={0.5} onChange={(v) => update("haze", v)} />
            <Slider label="云层覆盖" value={config.cloudCoverage} min={0} max={100} unit="%" onChange={(v) => update("cloudCoverage", v)} />
            <Slider label="云层速度" value={config.cloudSpeed} min={0} max={90} onChange={(v) => update("cloudSpeed", v)} />
          </section>

          <section>
            <div className="section-title"><span>区域诊断</span><em>05</em></div>
            {result ? <div className="stats-grid">
              <div><span>高程范围</span><b>{result.stats.minElevation.toFixed(0)}–{result.stats.maxElevation.toFixed(0)} m</b></div>
              <div><span>平均高程</span><b>{result.stats.meanElevation.toFixed(0)} m</b></div>
              <div><span>平均坡度</span><b>{result.stats.meanSlope.toFixed(1)}°</b></div>
              <div><span>森林覆盖</span><b>{(result.stats.forestCoverage * 100).toFixed(1)}%</b></div>
              <div><span>水体覆盖</span><b>{(result.stats.waterCoverage * 100).toFixed(1)}%</b></div>
              <div><span>积雪覆盖</span><b>{(result.stats.snowCoverage * 100).toFixed(1)}%</b></div>
              <div><span>候选聚落</span><b>{result.infrastructureSummary.settlements}</b></div>
              <div><span>道路连接</span><b>{result.infrastructureSummary.roads}</b></div>
              <div><span>桥梁 / 隧道</span><b>{result.infrastructureSummary.bridges} / {result.infrastructureSummary.tunnels}</b></div>
            </div> : <p className="muted-copy">生成后显示地形、水文与地表覆盖统计。</p>}
          </section>

          <section className="export-section">
            <div className="section-title"><span>卫星影像导出</span><em>06</em></div>
            <label className="select-control"><span>输出尺寸</span><select value={exportSize} onChange={(e) => setExportSize(Number(e.target.value))} disabled={busy}>
              <option value={2048}>2048 × 2048</option><option value={4096}>4096 × 4096</option><option value={8192}>8192 × 8192</option>
            </select></label>
            <button className="export-button" onClick={runExport} disabled={busy || !result}><span>导出 PNG</span><small>正射自然色 · 无水印</small></button>
            <p className="fine-print">采用原生多线程重采样与亚像素材质细节，高分辨率导出不会阻塞界面。</p>
          </section>
        </aside>
      </section>

      <footer className="statusbar"><span className={error ? "error" : ""}>{error ?? status}</span><span>CORE 0.1 · LOCAL ONLY · NO TELEMETRY</span></footer>
    </main>
  );
}

export default App;
