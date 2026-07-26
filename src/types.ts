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
  urbanDataBase64: string;
  cultivatedDataBase64: string;
  /** 0 = uncultivated, 1 = wheat, 2 = maize, 3 = other rotation crop. */
  cropDataBase64: string;
  roadDataBase64: string;
  roads: RenderRoad[];
  cities: UrbanModel[];
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
  class: "boulevard" | "avenue" | "street" | "service";
  width_metres: number;
}
export interface UrbanBlock { boundary: UrbanPoint[]; courtyard: UrbanPoint[] | null }
export interface UrbanBuilding {
  footprint: UrbanPoint[];
  courtyard: UrbanPoint[] | null;
  height_metres: number;
  roof: "mansard" | "terracotta" | "flat" | "setbackTower";
}
export interface UrbanModel {
  style: "parisian" | "barcelonaEixample" | "manhattan";
  streets: UrbanStreet[];
  blocks: UrbanBlock[];
  buildings: UrbanBuilding[];
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
