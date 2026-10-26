/**
 * A city as square tiles, streamed around the camera.
 *
 * A city's plan is cut into tiles of `CITY_TILE_M` metres in the plan's own frame,
 * the same cut the desktop shell makes when it builds one tile's detail. Each tile
 * is built on request, kept while the camera is near, and dropped beyond it. The
 * far form of the city (`cityMass`) is cut by the same grid, so a tile's mass is
 * hidden exactly while its detail is mounted and the two never share a surface.
 */

import * as THREE from "three";

import { type CityLayer, type CityLayerBuild, beginCityLayer } from "./cityLayer";
import type { CityScene } from "./cityScene";
import { offerFoliageMask } from "./post";

/** Side of one tile, city-local metres. */
export const CITY_TILE_M = 500;
/** Tiles any part of which is within this of the camera are built and kept. */
export const TILE_LOAD_M = 350;
/** Beyond the load range by this much a tile is dropped, so the edge never flickers. */
const TILE_UNLOAD_SLACK_M = 300;
/** Requests in flight at once. */
const MAX_FETCHES = 2;

export interface PlanFrame {
  origin: { x_km: number; y_km: number };
  rotationRadians: number;
}

/**
 * The column and row of the tile a plan point falls in. This is the Rust
 * `ModernCity::frame.to_local` followed by the floor of the tile size, so the two
 * sides agree on every element.
 */
export function tileOfPoint(frame: PlanFrame, point: { x_km: number; y_km: number }): [number, number] {
  const sin = Math.sin(frame.rotationRadians);
  const cos = Math.cos(frame.rotationRadians);
  const dx = (point.x_km - frame.origin.x_km) * 1000;
  const dy = (point.y_km - frame.origin.y_km) * 1000;
  const x = dx * cos + dy * sin;
  const z = -dx * sin + dy * cos;
  return [Math.floor(x / CITY_TILE_M), Math.floor(z / CITY_TILE_M)];
}

export function tileKey(column: number, row: number): string {
  return `${column},${row}`;
}

type TileState = "idle" | "fetching" | "ready" | "building" | "mounted" | "empty" | "failed";

interface Tile {
  column: number;
  row: number;
  state: TileState;
  scene: CityScene | null;
  build: CityLayerBuild | null;
  layer: CityLayer | null;
}

/**
 * Metres from the camera to the nearest point of a tile, zero inside it. Measured to
 * the tile's centre instead, the tile the camera stands in was out of a range shorter
 * than half a tile's diagonal, and the camera stood on bare far form.
 */
function tileDistance(column: number, row: number, camera: THREE.Vector3): number {
  const dx = Math.max(column * CITY_TILE_M - camera.x, 0, camera.x - (column + 1) * CITY_TILE_M);
  const dz = Math.max(row * CITY_TILE_M - camera.z, 0, camera.z - (row + 1) * CITY_TILE_M);
  return Math.hypot(dx, dz);
}

/** Set by the host under `?tilelog`: the streamer then reports how long each step takes. */
export const tileDebug = { log: false };

export type TileLoader = (index: number, column: number, row: number) => Promise<CityScene>;

/**
 * The detail of one city, as tiles around a camera. `group` is city-local: the host
 * scales, rotates and places it exactly as it places a whole-city layer.
 */
export class CityTileStreamer {
  readonly group = new THREE.Group();
  private readonly tiles = new Map<string, Tile>();
  private fetching = 0;
  private disposed = false;

  constructor(
    private readonly index: number,
    private readonly load: TileLoader,
    /** Called when a tile's detail is mounted or dropped, so its far form can be hidden. */
    private readonly onDetail: (column: number, row: number, mounted: boolean) => void,
    /** How far from the camera tiles are built, metres. */
    private readonly loadM: number = TILE_LOAD_M,
  ) {}

  /**
   * Advance the streamer. `camera` is in city-local metres. Builds at most
   * `budgetMs` of tile geometry this call; fetches start as slots free up.
   */
  update(camera: THREE.Vector3, budgetMs: number): void {
    if (this.disposed) return;
    const started = performance.now();
    const reach = Math.ceil((this.loadM + CITY_TILE_M) / CITY_TILE_M);
    const centreColumn = Math.floor(camera.x / CITY_TILE_M);
    const centreRow = Math.floor(camera.z / CITY_TILE_M);
    for (let row = centreRow - reach; row <= centreRow + reach; row += 1) {
      for (let column = centreColumn - reach; column <= centreColumn + reach; column += 1) {
        if (tileDistance(column, row, camera) > this.loadM) continue;
        const key = tileKey(column, row);
        if (!this.tiles.has(key)) {
          this.tiles.set(key, { column, row, state: "idle", scene: null, build: null, layer: null });
        }
      }
    }
    for (const tile of this.tiles.values()) {
      const distance = tileDistance(tile.column, tile.row, camera);
      if (distance > this.loadM + TILE_UNLOAD_SLACK_M) {
        this.drop(tile);
        continue;
      }
      if (tile.state === "idle" && distance <= this.loadM && this.fetching < MAX_FETCHES) {
        this.fetch(tile);
      }
      if (tile.state === "ready") {
        const t0 = performance.now();
        tile.build = beginCityLayer(tile.scene as CityScene);
        if (tileDebug.log) console.log(`[tiles] begin ${tile.column},${tile.row} ${(performance.now() - t0).toFixed(0)}ms`);
        tile.scene = null;
        tile.state = "building";
      }
      if (tile.state === "building" && tile.build) {
        const remaining = budgetMs - (performance.now() - started);
        if (remaining > 0) {
          const t1 = performance.now();
          tile.build.step(remaining);
          if (tileDebug.log) console.log(`[tiles] step ${tile.column},${tile.row} ${(performance.now() - t1).toFixed(1)}ms progress ${tile.build.progress.toFixed(2)}`);
        }
        if (tile.build.done) {
          this.mount(tile, tile.build.layer);
        }
      }
    }
  }

  /** Animate every mounted tile to the host's clock, as the whole-city layer is. */
  tick(seconds: number, distanceM: number, camera: THREE.Camera): void {
    for (const tile of this.tiles.values()) {
      tile.layer?.update(seconds, distanceM, camera);
    }
  }

  /** How many tiles are in each state, for the console while the streamer is being checked. */
  summary(): Record<string, number> {
    const counts: Record<string, number> = {};
    for (const tile of this.tiles.values()) counts[tile.state] = (counts[tile.state] ?? 0) + 1;
    return counts;
  }

  dispose(): void {
    this.disposed = true;
    for (const tile of this.tiles.values()) this.drop(tile);
    this.tiles.clear();
  }

  private fetch(tile: Tile): void {
    tile.state = "fetching";
    this.fetching += 1;
    this.load(this.index, tile.column, tile.row)
      .then((scene) => {
        if (this.disposed) return;
        // A tile with no buildings, roads or trees has nothing to build.
        const empty = scene.meshes.length === 0 && scene.instances.every((list) => list.count === 0);
        if (!empty) offerFoliageMask(scene.textures);
        tile.scene = empty ? null : scene;
        tile.state = empty ? "empty" : "ready";
      })
      .catch(() => {
        if (!this.disposed) tile.state = "failed";
      })
      .finally(() => {
        this.fetching -= 1;
      });
  }

  private mount(tile: Tile, layer: CityLayer | null): void {
    if (!layer) {
      tile.state = "empty";
      tile.build = null;
      return;
    }
    tile.layer = layer;
    tile.build = null;
    tile.state = "mounted";
    this.group.add(layer.group);
    this.onDetail(tile.column, tile.row, true);
  }

  private drop(tile: Tile): void {
    if (tile.build) {
      tile.build.cancel();
      tile.build = null;
    }
    if (tile.layer) {
      this.group.remove(tile.layer.group);
      tile.layer.dispose();
      tile.layer = null;
      this.onDetail(tile.column, tile.row, false);
    }
    if (tile.state === "mounted" || tile.state === "building" || tile.state === "ready") {
      tile.state = "idle";
    }
    tile.scene = null;
  }
}
