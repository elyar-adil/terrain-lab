/**
 * Per-device settings for the streamed city.
 *
 * A streamed city is the heaviest thing the page draws: every tile brings a full
 * set of textures, grown trees, grass shells and thousands of instances. A phone
 * browser tab is killed at roughly 1-1.5 GB, and a desktop on integrated graphics
 * drops to single-digit frame rates with the full profile, so the budget is set
 * in one place and every city module reads it from here.
 *
 * `?mobile=0|1` (see `platform.ts`) switches the profile for testing.
 */
import { isConstrainedDevice } from "../platform";

export interface CityQuality {
  /** Tiles any part of which is within this of the camera are built, metres. */
  tileLoadM: number;
  /** Most tiles that may be fetched, building or mounted at once. */
  maxTiles: number;
  /** Texture anisotropy; three.js clamps it to what the GPU supports. */
  anisotropy: number;
  /** Multiplier on every LOD distance (leaf cards, props, trim...). */
  lodScale: number;
  /** Draw real grass shells near the camera. */
  grassShells: boolean;
  /** Distances (m) at which grown trees step down a level, and where they stop. */
  treeLodM: [number, number, number];
  treeFarM: number;
  /** Milliseconds of tile building per frame. */
  tileBudgetMs: number;
}

/**
 * Visual quality is the same on every device: performance comes from sharing and
 * streaming work, never from drawing less. Only how much tile-building work is
 * done per frame differs, which changes how fast detail arrives, not how it looks.
 */
const FULL: CityQuality = {
  tileLoadM: 600,
  maxTiles: 9,
  anisotropy: 16,
  lodScale: 1,
  grassShells: true,
  treeLodM: [22, 45, 160],
  treeFarM: 1500,
  tileBudgetMs: 6,
};

const LIGHT: CityQuality = { ...FULL, tileBudgetMs: 3 };

export const cityQuality: CityQuality = isConstrainedDevice ? LIGHT : FULL;
