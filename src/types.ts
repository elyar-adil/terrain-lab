import type { CityScene } from "./city/cityScene";

export type TerrainPreset = "arid" | "temperate" | "glacial";
export type Landform = "mountainRange" | "hills" | "plains" | "plateau" | "coastal" | "archipelago";

export interface SimulationConfig {
  seed: number;
  preset: TerrainPreset;
  landform: Landform;
  gridSize: number;
  worldSizeKm: number;
  rainfall: number;
  evaporation: number;
  windSpeed: number;
  windDirection: number;
  sunAzimuth: number;
  sunElevation: number;
  haze: number;
  cloudCoverage: number;
  cloudSpeed: number;
}

export interface TerrainStats {
  minElevation: number;
  maxElevation: number;
  meanElevation: number;
  meanSlope: number;
  waterCoverage: number;
  snowCoverage: number;
  forestCoverage: number;
}

export interface GenerationResult {
  previewDataUrl: string;
  width: number;
  height: number;
  worldSizeKm: number;
  elapsedMs: number;
  stats: TerrainStats;
  meshSize: number;
  waterDataSize: number;
  heightDataBase64: string;
  forestDataBase64: string;
  vegetationExclusionDataBase64: string;
  /** Settlement footprints; heights and the exclusion mask are already levelled/cleared under them (Rust). */
  /** Far-LOD tree prototypes from Rust (species x lod), typed arrays as base64. */
  farTrees?: FarTreePayload[];
  citySites?: { xKm: number; yKm: number; radiusM: number }[];
  urbanDataBase64: string;
  cultivatedDataBase64: string;
  /** 0 = uncultivated, 1 = wheat, 2 = maize, 3 = other rotation crop. */
  cropDataBase64: string;
  roadDataBase64: string;
  roads: RenderRoad[];
  cities: UrbanModel[];
  /** High-detail Chinese city graph emitted by the Rust urban generator. */
  modernCities?: ModernCityPayload[];
  /** Render-ready metre-scale city scenes, index-parallel to `modernCities`. */
  /** Shared L-System tree prototypes (per species × variant × LOD). */
  vegetationPrototypes?: VegetationPrototypePayload[];
  /** CPU-baked weathered material textures shared by every surface. */
  materialTextures?: BakedMaterialTexture[];
  waterHeightDataBase64: string;
  waterMaskBase64: string;
  waterKindBase64: string;
  flowDirectionBase64: string;
  flowStrengthBase64: string;
  analysisPreviews: {
    discharge: string;
    lake: string;
    wetland: string;
    floodplain: string;
    basin: string;
    riverOrder: string;
    geology: string;
    soilDepth: string;
    landCover: string;
    travelCost: string;
    hazard: string;
    settlementSuitability: string;
    agriculturalSuitability: string;
    urbanLand: string;
    cultivatedLand: string;
    infrastructure: string;
  };
  infrastructureSummary: {
    settlements: number;
    roads: number;
    bridges: number;
    tunnels: number;
  };
}

export type RoadClass = "motorway" | "arterial" | "collector" | "local" | "rural";
export interface RoadProfile {
  carriagewayWidthMetres: number;
  rightOfWayWidthMetres: number;
  lanes: number;
  paved: boolean;
}
export interface RenderRoad {
  id: number;
  class: RoadClass;
  profile: RoadProfile;
  lengthKm: number;
  pathKm: [number, number][];
}

export interface UrbanPoint { x_km: number; y_km: number }
export interface UrbanStreet {
  from: UrbanPoint;
  to: UrbanPoint;
  class: "boulevard" | "avenue" | "street" | "service" | string;
  width_metres: number;
  widthMetres?: number;
}
export interface UrbanBlock { boundary: UrbanPoint[]; courtyard: UrbanPoint[] | null }
export interface UrbanBuilding {
  id?: number | string;
  parcelId?: number | string;
  footprint: UrbanPoint[];
  courtyard: UrbanPoint[] | null;
  height_metres: number;
  heightMetres?: number;
  floors?: number;
  variant?: number;
  style?: string;
  pitched?: boolean;
  tierRing?: UrbanPoint[];
  podiumFloors?: number;
  facade?: string;
  podiumHeightMetres?: number;
  windowBays?: number;
  balconyBays?: number;
  entranceCount?: number;
  roof: "mansard" | "terracotta" | "flat" | "setbackTower";
}

/** Optional high-detail city payload emitted by newer Rust generators. */
export interface UrbanNode extends UrbanPoint {
  id?: number;
  kind?: string;
}
export interface UrbanRoad {
  id?: number;
  class?: string;
  from?: UrbanPoint;
  to?: UrbanPoint;
  pathKm?: [number, number][];
  path_km?: [number, number][];
  widthMetres?: number;
  width_metres?: number;
  lanesForward?: number;
  lanesBackward?: number;
  medianMetres?: number;
  layer?: number;
  structure?: boolean;
}
export interface UrbanLane {
  id?: string;
  edgeId?: number | string;
  direction?: 1 | -1 | number;
  index?: number;
  indexFromCurb?: number;
  useType?: string;
  widthMetres?: number;
  offsetMetres?: number;
  allowedMovements?: string[];
  path?: UrbanPoint[];
  markings?: Array<{ kind?: string; arrow?: string; offsetMetres?: number; lengthMetres?: number; dashGapMetres?: number }>;
  movements?: string[];
}
export interface UrbanConnector {
  id?: string;
  fromLane?: string;
  toLane?: string;
  movement?: "left" | "through" | "right" | "uturn" | string;
  path?: UrbanPoint[];
  centreline?: UrbanPoint[];
  widthMetres?: number;
}
export interface UrbanJunction {
  id?: string;
  centre?: UrbanPoint;
  ring?: UrbanPoint[];
  radiusMetres?: number;
  crossing?: boolean;
  roundabout?: boolean;
  node?: number;
  roadIds?: number[];
  kind?: string;
  crosswalks?: UrbanPoint[][];
  signalHeads?: Array<{ id?: string; roadId?: number; position?: UrbanPoint; heightMetres?: number; aspects?: string[] }>;
  connectors?: UrbanConnector[];
}
export interface UrbanCompound {
  id?: string;
  blockId?: number | string;
  seed?: number;
  type?: string;
  wallRing?: UrbanPoint[];
  loopRing?: UrbanPoint[];
  gate?: { x_km: number; y_km: number; width: number; angle: number };
  paths?: UrbanPoint[][];
  roadWidthMetres?: number;
  courts?: Array<{ x_km: number; y_km: number; rx?: number; rz?: number; angle?: number; kind?: string }>;
  parking?: Array<{ x_km: number; y_km: number; angle?: number }>;
  boundary?: UrbanPoint[];
  gatePoints?: UrbanPoint[];
  fenceHeightMetres?: number;
  plantedRatio?: number;
}
export interface UrbanTree {
  id?: number;
  x_km: number;
  y_km: number;
  size?: number;
  seed?: number;
  point?: UrbanPoint;
  species?: string;
  /** Which L-System prototype variant of the species this tree instances. */
  variant?: number;
  heightMetres?: number;
  crownRadiusMetres?: number;
  trunkRadiusMetres?: number;
}
export interface UrbanParcel {
  boundary: UrbanPoint[];
  landUse?: string;
  land_use?: string;
  heightMetres?: number;
  height_metres?: number;
}
export interface UrbanRiver {
  centerline?: UrbanPoint[];
  centerline_km?: [number, number][];
  path?: UrbanPoint[];
  path_km?: [number, number][];
  widthMetres?: number;
  width_metres?: number;
}

/** One L-System tree prototype: flattened segments and foliage blobs. */
export interface FarTreePayload {
  species: string;
  /** 0 = mid detail, 1 = far. */
  lod: number;
  heightMetres: number;
  crownRadiusMetres: number;
  trunkRadiusMetres: number;
  /** base64 f32 LE xyz per vertex, unit height. */
  positions: string;
  normals: string;
  /** base64 f32 LE linear rgb per vertex. */
  colors: string;
  /** base64 u16 LE triangle indices. */
  indices: string;
}

export interface VegetationPrototypePayload {
  species: string;
  lod: "near" | "far" | string;
  /** Flattened per segment: startX,startY,startZ,endX,endY,endZ,radiusStart,radiusEnd. */
  segments: number[];
  /** Flattened per blob: x,y,z,radius,density. */
  foliage: number[];
  heightMetres: number;
  crownRadiusMetres: number;
}

/** A CPU-baked, weathered material texture from the Rust procedural kit. */
export interface BakedMaterialTexture {
  name: string;
  width: number;
  height: number;
  /** base64 RGBA pixels. */
  data: string;
}

export interface ModernCityPayload {  style: "chineseModern";
  seed: number;
  nodes: Array<{ id: number; point: UrbanPoint }>;
  sdRoads: Array<{ id: number; from: number; to: number; class: string; bridge: boolean }>;
  hdRoads: Array<{
    id: number;
    sdRoad: number;
    class: string;
    widthMetres: number;
    centreline: UrbanPoint[];
    bridge: boolean;
    lanesForward?: number;
    lanesBackward?: number;
    medianMetres?: number;
    layer?: number;
    structure?: boolean;
    lanes?: UrbanLane[];
    connectors?: UrbanConnector[];
  }>;
  lanes?: UrbanLane[];
  connectors?: UrbanConnector[];
  junctions?: UrbanJunction[];
  blocks: UrbanBlock[];
  parcels: Array<{ id: number; blockId: number; ring: UrbanPoint[]; useType: string; compound: boolean }>;
  buildings: Array<{
    id: number;
    parcelId: number;
    footprint: UrbanPoint[];
    heightMetres: number;
    floors: number;
    useType: string;
    roof: "mansard" | "terracotta" | "flat" | "setbackTower";
    variant?: number;
    style?: string;
    pitched?: boolean;
    tierRing?: UrbanPoint[];
    podiumFloors?: number;
    facade?: string;
    podiumHeightMetres?: number;
    windowBays?: number;
    balconyBays?: number;
    entranceCount?: number;
  }>;
  river: UrbanPoint[] | null;
  riverWidthMetres?: number;
  compounds?: UrbanCompound[];
  trees?: UrbanTree[];
}
export interface UrbanModel {
  /**
   * City style emitted by the Rust urban generator.  `modernChinese` is the
   * default city layer; the legacy styles remain accepted so existing project
   * files continue to render.
   */
  style: "modernChinese" | "chineseModern" | "parisian" | "barcelonaEixample" | "manhattan";
  streets?: UrbanStreet[];
  blocks?: UrbanBlock[];
  buildings?: UrbanBuilding[];
  nodes?: UrbanNode[];
  sdRoads?: UrbanRoad[];
  hdRoads?: UrbanRoad[];
  lanes?: UrbanLane[];
  connectors?: UrbanConnector[];
  junctions?: UrbanJunction[];
  parcels?: UrbanParcel[];
  river?: UrbanRiver;
  compounds?: UrbanCompound[];
  trees?: UrbanTree[];
}

export interface GenerationProgress {
  stage: string;
  progress: number;
}

export interface ProjectDocument {
  schemaVersion: 1;
  name: string;
  config: SimulationConfig;
}
