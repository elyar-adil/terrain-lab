/**
 * The land beyond the edge of the generated tile.
 *
 * The world is one square of terrain, and drawn alone it is a slab floating in the
 * sky: the first thing that says "model" rather than "place". Nothing is known
 * about what lies outside, so this does not invent geography; it continues the
 * ground the way distance shows it. Next to the edge the relief and colour are the
 * tile's own, mirrored and blurred, so the join is continuous in height and tone.
 * Further out both settle into the tile's average country, broken by broad noise,
 * and the atmosphere (atmosphere.ts) takes it into the horizon.
 */
import * as THREE from "three";

const HALF = 1.6;
// Rings of the skirt as a scale of the tile's outline: tight where the relief is
// still followed, then stretching to the camera's far plane.
const RINGS: number[] = [];
// Out to 0.45 tile widths the tile's own relief is continued, at about the
// tile's own sampling so its slopes shade as slopes and not as streaks.
for (let i = 0; i <= 44; i += 1) RINGS.push(1 + 0.9 * Math.pow(i / 44, 1.6));
RINGS.push(2.1, 2.4, 2.9, 3.6, 4.8, 7, 11, 19);

/**
 * Replace the outermost cells of a square grid with the ring `depth` cells in.
 *
 * The generator's outermost row is its drainage outlet: every edge cell is sea,
 * tens to hundreds of metres below its inland neighbour. Drawn as it is, the
 * tile is a block with a moat cut round it. The outlet is a boundary condition,
 * not country, so the render extends the last real ring to the edge instead.
 */
export function extendRim<T extends { [index: number]: number; length: number }>(values: T, size: number, channels = 1, depth = 2): T {
  if (size < depth * 4 || values.length < size * size * channels) return values;
  const inward = (i: number) => Math.min(size - 1 - depth, Math.max(depth, i));
  for (let y = 0; y < size; y += 1) {
    const interiorRow = y >= depth && y < size - depth;
    for (let x = 0; x < size; x += 1) {
      if (interiorRow && x >= depth && x < size - depth) {
        x = size - depth - 1; // skip the interior of this row
        continue;
      }
      const from = (inward(y) * size + inward(x)) * channels;
      const to = (y * size + x) * channels;
      for (let c = 0; c < channels; c += 1) values[to + c] = values[from + c];
    }
  }
  return values;
}

export interface HorizonLandOptions {
  /** Heights in metres, row-major, row 0 at z = -1.6. */
  heights: Float32Array;
  meshSize: number;
  minElevation: number;
  metresToScene: number;
  /** The tile's baked colour image (the terrain material's map). */
  map: THREE.Texture;
  /** Water surface heights in metres (same grid as `heights`) and the water mask (its own square grid, 0-255). */
  waterHeights: Float32Array;
  waterMask: Uint8Array;
  waterMaskSize: number;
}

export function createHorizonLand(options: HorizonLandOptions): THREE.Mesh {
  const { heights, meshSize: n, minElevation, metresToScene, map, waterHeights, waterMask, waterMaskSize } = options;
  const last = n - 1;
  const heightAt = (ix: number, iz: number) => Math.max(0, heights[iz * n + ix] - minElevation) * metresToScene;
  const bilinear = (worldX: number, worldZ: number) => {
    const gx = THREE.MathUtils.clamp((worldX + HALF) / (2 * HALF) * last, 0, last);
    const gz = THREE.MathUtils.clamp((worldZ + HALF) / (2 * HALF) * last, 0, last);
    const x0 = Math.floor(gx), z0 = Math.floor(gz);
    const x1 = Math.min(last, x0 + 1), z1 = Math.min(last, z0 + 1);
    const tx = gx - x0, tz = gz - z0;
    return THREE.MathUtils.lerp(
      THREE.MathUtils.lerp(heightAt(x0, z0), heightAt(x1, z0), tx),
      THREE.MathUtils.lerp(heightAt(x0, z1), heightAt(x1, z1), tx),
      tz,
    );
  };
  const mirror = (v: number) => {
    // Reflect about the nearest edge (valid out to one tile beyond it).
    if (v < -HALF) return -2 * HALF - v;
    if (v > HALF) return 2 * HALF - v;
    return v;
  };

  // The tile's outline, vertex for vertex, so the join has no cracks.
  const outline: [number, number, number][] = [];
  const cells: [number, number][] = [];
  const coordinate = (i: number) => -HALF + (2 * HALF * i) / last;
  const corner = (ix: number, iz: number) => {
    outline.push([coordinate(ix), coordinate(iz), heightAt(ix, iz)]);
    cells.push([ix, iz]);
  };
  for (let i = 0; i < last; i += 1) corner(i, 0);
  for (let i = 0; i < last; i += 1) corner(last, i);
  for (let i = last; i > 0; i -= 1) corner(i, last);
  for (let i = last; i > 0; i -= 1) corner(0, i);
  const count = outline.length;

  // Where the tile ends in open water the sea goes on to the horizon; where it
  // ends on land, land does. Decided per outline vertex and eased along the
  // outline so a coast leaves the tile as a coast.
  const wet = new Float32Array(count);
  const seaLevels: number[] = [];
  for (let i = 0; i < count; i += 1) {
    const [ix, iz] = cells[i];
    const mx = Math.round((ix / last) * (waterMaskSize - 1));
    const mz = Math.round((iz / last) * (waterMaskSize - 1));
    if (waterMask[mz * waterMaskSize + mx] > 96) {
      wet[i] = 1;
      seaLevels.push(Math.max(0, waterHeights[iz * n + ix] - minElevation) * metresToScene);
    }
  }
  seaLevels.sort((a, b) => a - b);
  const seaLevel = seaLevels.length ? seaLevels[Math.floor(seaLevels.length / 2)] : 0;
  // Two widths of that easing: a tight one, exact where the coast meets the edge,
  // and a broad one inside which the shoreline is free to wander further out.
  const eased = (reach: number) => {
    const prefix = new Float32Array(count * 3 + 1);
    for (let i = 0; i < count * 3; i += 1) prefix[i + 1] = prefix[i] + wet[i % count];
    const out = new Float32Array(count);
    for (let i = 0; i < count; i += 1) out[i] = (prefix[count + i + reach + 1] - prefix[count + i - reach]) / (2 * reach + 1);
    return out;
  };
  const sea = eased(Math.max(2, Math.round(n / 60)));
  const seaBroad = eased(Math.max(4, Math.round(n / 5)));
  // The level the far country lies at: the low side of the dry edge.
  const dry = outline.filter((_, i) => wet[i] === 0).map((point) => point[2]).sort((a, b) => a - b);
  const base = dry.length ? dry[Math.floor(dry.length * 0.2)] : seaLevel;

  const positions = new Float32Array(count * RINGS.length * 3);
  const uvs = new Float32Array(count * RINGS.length * 2);
  const seas = new Float32Array(count * RINGS.length * 2);
  for (let ring = 0; ring < RINGS.length; ring += 1) {
    const scale = RINGS[ring];
    for (let i = 0; i < count; i += 1) {
      const [ex, ez, eh] = outline[i];
      const x = ex * scale, z = ez * scale;
      let y = eh;
      if (ring > 0) {
        const outside = (scale - 1) * 0.5; // tile widths beyond the edge
        const keep = 1 - THREE.MathUtils.smoothstep(outside, 0.0, 0.42);
        // The tile's own relief reflected in the edge, sinking into level country.
        y = base + (bilinear(mirror(x), mirror(z)) - base) * keep * keep;
        y = THREE.MathUtils.lerp(y, seaLevel, Math.max(sea[i], THREE.MathUtils.smoothstep(seaBroad[i], 0.05, 0.3)));
      }
      seas[(ring * count + i) * 2] = sea[i];
      seas[(ring * count + i) * 2 + 1] = seaBroad[i];
      const o = (ring * count + i) * 3;
      positions[o] = x;
      positions[o + 1] = y;
      positions[o + 2] = z;
      const u = (ring * count + i) * 2;
      uvs[u] = (x + HALF) / (2 * HALF);
      uvs[u + 1] = 1 - (z + HALF) / (2 * HALF);
    }
  }
  const indices = new Uint32Array(count * (RINGS.length - 1) * 6);
  let cursor = 0;
  for (let ring = 0; ring < RINGS.length - 1; ring += 1) {
    for (let i = 0; i < count; i += 1) {
      const a = ring * count + i;
      const b = ring * count + ((i + 1) % count);
      const c = a + count;
      const d = b + count;
      indices[cursor++] = a; indices[cursor++] = b; indices[cursor++] = c;
      indices[cursor++] = b; indices[cursor++] = d; indices[cursor++] = c;
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute("uv", new THREE.BufferAttribute(uvs, 2));
  geometry.setAttribute("aSea", new THREE.BufferAttribute(seas, 2));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));
  geometry.computeVertexNormals();

  // The image repeats mirrored, so colour is continuous across the edge too.
  map.wrapS = map.wrapT = THREE.MirroredRepeatWrapping;
  map.needsUpdate = true;
  const material = new THREE.MeshStandardMaterial({ map, roughness: 1, metalness: 0 });
  material.onBeforeCompile = (shader) => {
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nattribute vec2 aSea;\nvarying vec2 vLandP;\nvarying vec3 vLandW;\nvarying vec2 vSea;")
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvLandP = position.xz;\nvLandW = position;\nvSea = aSea;");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", /* glsl */ `#include <common>
varying vec2 vLandP;
varying vec3 vLandW;
varying vec2 vSea;
float gSeaMask = 0.0;
float landHash(vec2 p) { p = fract(p * vec2(123.34, 456.21)); p += dot(p, p + 45.32); return fract(p.x * p.y); }
float landNoise(vec2 p) {
  vec2 i = floor(p), f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(landHash(i), landHash(i + vec2(1.0, 0.0)), f.x), mix(landHash(i + vec2(0.0, 1.0)), landHash(i + vec2(1.0, 1.0)), f.x), f.y);
}
float landFbm(vec2 p) {
  float v = 0.0, a = 0.5;
  mat2 r = mat2(0.80, -0.60, 0.60, 0.80);
  for (int i = 0; i < 5; i++) { v += a * landNoise(p); p = r * p * 2.03 + 11.7; a *= 0.5; }
  return v;
}`)
      .replace("#include <map_fragment>", /* glsl */ `
{
  vec2 t = (vLandP + ${HALF.toFixed(1)}) / ${(2 * HALF).toFixed(1)};
  vec2 beyond = max(max(-t, t - 1.0), 0.0);
  float outside = length(beyond); // tile widths past the edge
  // Close to the edge: the tile's own image, losing detail with distance (the
  // mirror must not read as a reflection). Far: its regional average.
  // Mirrored by hand and held off the image's own border (the generator's outlet
  // row and the wet margin shaded around it). Only its broad tones are used: the
  // terrain repaints its fields and natural ground itself, so the image's detail
  // would not match what the tile shows.
  vec2 mirrored = 0.03 + 0.94 * (1.0 - abs(1.0 - mod(vMapUv, 2.0)));
  vec3 nearColour = texture2D(map, mirrored, 4.5 + 3.5 * smoothstep(0.0, 0.10, outside)).rgb;
  vec3 region = mix(texture2D(map, mirrored, 8.0).rgb, texture2D(map, vec2(0.5), 12.0).rgb, 0.55);
  float broad = landFbm(vLandP * 1.9) * 0.7 + landFbm(vLandP * 7.3 + 4.0) * 0.3;
  region *= 0.72 + 0.56 * broad;
  vec3 land = mix(nearColour, region, smoothstep(0.04, 0.30, outside));
  // The terrain builds its natural ground darker and greyer than the baked image
  // (rock, soil and grass at their measured reflectance); match it.
  land = mix(land, vec3(dot(land, vec3(0.3, 0.55, 0.15))), 0.30) * 0.62;
  // The shoreline between a wet and a dry stretch of the edge wanders instead of
  // leaving the tile along a ruled line.
  float shore = (landFbm(vLandP * 1.7 + 7.0) - 0.5) * 1.5 * smoothstep(0.0, 0.25, vSea.y) * smoothstep(1.0, 0.75, vSea.y);
  gSeaMask = smoothstep(0.47, 0.53, mix(vSea.x, vSea.y + shore, smoothstep(0.0, 0.08, outside)));
  diffuseColor.rgb *= land * (1.0 - gSeaMask);
}`)
      .replace("#include <emissivemap_fragment>", /* glsl */ `#include <emissivemap_fragment>
#if defined( USE_FOG ) && ! defined( FOG_EXP2 )
if (gSeaMask > 0.0) {
  // Open sea, the same water as the tile's (Terrain3D's water shader): a dark
  // body under the mirrored sky, and the sun's glitter path.
  vec3 seaView = normalize(cameraPosition - vLandW);
  float seaFacing = max(seaView.y, 0.0);
  float seaFresnel = 0.02 + 0.98 * pow(1.0 - seaFacing, 5.0);
  vec3 seaMirror = vec3(-seaView.x, abs(seaView.y) + 0.02, -seaView.z);
  vec3 seaSky = atmSky(normalize(seaMirror), max(cameraPosition.y * fogNear, 2.0), fogFar);
  float seaSun = max(dot(reflect(-ATM_SUN, vec3(0.0, 1.0, 0.0)), seaView), 0.0);
  float seaGlitter = pow(seaSun, 90.0) * 0.35 + pow(seaSun, 900.0) * 6.0;
  vec3 sea = vec3(0.009, 0.056, 0.077) * (1.0 - seaFresnel) + seaSky * seaFresnel * 0.75
    + atmSunTransmittance(fogFar) * seaGlitter * (0.3 + 0.7 * seaFresnel);
  totalEmissiveRadiance += sea * gSeaMask;
}
#endif`);
  };
  const mesh = new THREE.Mesh(geometry, material);
  mesh.name = "horizon-land";
  mesh.frustumCulled = false;
  mesh.matrixAutoUpdate = false;
  // A flat decal-like ground: contact occlusion has nothing to find on it.
  mesh.userData.noOcclusion = true;
  return mesh;
}
