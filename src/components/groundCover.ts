import * as THREE from "three";

/**
 * Near-field ground cover for open land: instanced 3D grass tufts, shrubs and
 * wildflowers streamed around the camera target. It exists so that ground never
 * reads as a flat green plane once the camera is close enough to see blades:
 * the terrain shader carries micro-relief and colour variation at every range,
 * and this layer adds real geometry (distance-culled by construction: only a
 * disc around the target is populated, thinning to nothing at its rim).
 *
 * Everything is seeded from world-aligned cells, so panning never reshuffles
 * plants. Cultivated land, settlements and roads are excluded through the
 * payload's vegetation-exclusion mask, water through the water mask, and steep
 * or high ground is left bare.
 */

export interface GroundSampler {
  /** Deterministic 0..1 hash of two integers and a salt. */
  hash01(x: number, y: number, salt: number): number;
  /** Forest density 0..255 at a world position. */
  forest(worldX: number, worldZ: number): number;
  /** Vegetation exclusion 0..255 (cities, roads, fields). */
  excluded(worldX: number, worldZ: number): number;
  /** Water coverage 0..255. */
  water(worldX: number, worldZ: number): number;
  /** Terrain height in scene units above the mesh floor. */
  height(worldX: number, worldZ: number): number;
  /** Fraction 0..1 of the elevation range at this point (snow line, alpine bare rock). */
  elevation01(worldX: number, worldZ: number): number;
}

interface Layer {
  mesh: THREE.InstancedMesh;
  capacity: number;
  count: number;
}

function seeded(seed: number) {
  let state = seed >>> 0;
  return () => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return state / 4294967296;
  };
}

/** A clump of tapered, arched blades: dark at the root, bright and warm at the tip. */
export function tuftGeometry(metresToScene: number): THREE.BufferGeometry {
  const random = seeded(0x7f4a7c15);
  const positions: number[] = [];
  const normals: number[] = [];
  const colours: number[] = [];
  const blades = 9;
  for (let blade = 0; blade < blades; blade += 1) {
    const angle = (blade / blades) * Math.PI * 2 + random() * 0.6;
    const height = (0.20 + random() * 0.24) * metresToScene;
    const width = (0.011 + random() * 0.008) * metresToScene;
    const reach = (0.05 + random() * 0.16) * metresToScene;
    const rootRadius = random() * 0.05 * metresToScene;
    const dx = Math.cos(angle);
    const dz = Math.sin(angle);
    // Blade cross direction (perpendicular to its lean in the ground plane).
    const cx = -dz;
    const cz = dx;
    const rootX = dx * rootRadius;
    const rootZ = dz * rootRadius;
    const shade = 0.85 + random() * 0.3;
    const rows = 3;
    const stations: { x: number; y: number; z: number; w: number; v: number }[] = [];
    for (let row = 0; row <= rows; row += 1) {
      const t = row / rows;
      stations.push({
        x: rootX + dx * reach * t * t,
        y: height * t - height * 0.10 * t * t,
        z: rootZ + dz * reach * t * t,
        w: width * (1 - t * t * 0.92),
        v: (0.38 + 0.62 * t) * shade,
      });
    }
    const push = (s: (typeof stations)[number], side: number) => {
      positions.push(s.x + cx * s.w * side, s.y, s.z + cz * s.w * side);
      // Soft grass shading: normal leans away from the clump and up.
      const n = new THREE.Vector3(dx * 0.35 + cx * side * 0.25, 1, dz * 0.35 + cz * side * 0.25).normalize();
      normals.push(n.x, n.y, n.z);
      colours.push(s.v * 0.9, s.v, s.v * (0.62 + 0.2 * (1 - s.v)));
    };
    for (let row = 0; row < rows; row += 1) {
      const a = stations[row];
      const b = stations[row + 1];
      if (row === rows - 1) {
        // Last quad collapses to the tip triangle.
        push(a, -1); push(a, 1); push(b, 0);
      } else {
        push(a, -1); push(a, 1); push(b, 1);
        push(a, -1); push(b, 1); push(b, -1);
      }
    }
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geometry.setAttribute("normal", new THREE.Float32BufferAttribute(normals, 3));
  geometry.setAttribute("color", new THREE.Float32BufferAttribute(colours, 3));
  return geometry;
}

/** A leafy shrub: a noisy, low-poly heap with a dark underside and lit top. */
export function shrubGeometry(metresToScene: number): THREE.BufferGeometry {
  const geometry = new THREE.SphereGeometry(0.5 * metresToScene, 11, 8);
  const position = geometry.attributes.position as THREE.BufferAttribute;
  const colours = new Float32Array(position.count * 3);
  for (let index = 0; index < position.count; index += 1) {
    const x = position.getX(index) / metresToScene;
    const y = position.getY(index) / metresToScene;
    const z = position.getZ(index) / metresToScene;
    // Lumpy foliage: layered sines in the vertex's own direction, so the shrub
    // is a heap of leafy bunches rather than a smooth ball.
    const lump = 0.86 + 0.20 * Math.sin(x * 19 + z * 7) * Math.cos(z * 17 - y * 9) + 0.10 * Math.sin(y * 31 + x * 23);
    position.setXYZ(
      index,
      x * lump * 1.15 * metresToScene,
      Math.max(-0.05, y * lump * 0.78 + 0.30) * metresToScene,
      z * lump * 1.15 * metresToScene,
    );
    const up = THREE.MathUtils.clamp(y + 0.5, 0, 1);
    const speckle = 0.5 + 0.5 * Math.sin(x * 57 + y * 43) * Math.sin(z * 61 - y * 37);
    const value = 0.42 + 0.50 * up + 0.30 * speckle;
    colours[index * 3] = value * 0.82;
    colours[index * 3 + 1] = value;
    colours[index * 3 + 2] = value * 0.55;
  }
  geometry.setAttribute("color", new THREE.BufferAttribute(colours, 3));
  geometry.computeVertexNormals();
  return geometry;
}

/** A wildflower: thin stem and a flat five-petal head; the instance colour is the petal colour. */
export function flowerGeometry(metresToScene: number): THREE.BufferGeometry {
  const positions: number[] = [];
  const colours: number[] = [];
  const stem = 0.24 * metresToScene;
  const w = 0.006 * metresToScene;
  // Stem: two crossed slim quads.
  for (const [ax, az] of [[1, 0], [0, 1]]) {
    positions.push(-ax * w, 0, -az * w, ax * w, 0, az * w, ax * w * 0.6, stem, az * w * 0.6);
    positions.push(-ax * w, 0, -az * w, ax * w * 0.6, stem, az * w * 0.6, -ax * w * 0.6, stem, -az * w * 0.6);
    for (let i = 0; i < 6; i += 1) colours.push(0.16, 0.42, 0.06);
  }
  // Head: fan of five petals, tilted slightly so it faces up and out.
  const petal = 0.034 * metresToScene;
  for (let p = 0; p < 5; p += 1) {
    const a0 = (p / 5) * Math.PI * 2;
    const a1 = ((p + 0.5) / 5) * Math.PI * 2;
    const a2 = ((p + 1) / 5) * Math.PI * 2;
    positions.push(0, stem + 0.004 * metresToScene, 0);
    positions.push(Math.cos(a0) * petal * 0.6, stem + 0.010 * metresToScene, Math.sin(a0) * petal * 0.6);
    positions.push(Math.cos(a1) * petal, stem + 0.012 * metresToScene, Math.sin(a1) * petal);
    positions.push(0, stem + 0.004 * metresToScene, 0);
    positions.push(Math.cos(a1) * petal, stem + 0.012 * metresToScene, Math.sin(a1) * petal);
    positions.push(Math.cos(a2) * petal * 0.6, stem + 0.010 * metresToScene, Math.sin(a2) * petal * 0.6);
    for (let i = 0; i < 6; i += 1) colours.push(i % 3 === 0 ? 0.55 : 1, i % 3 === 0 ? 0.42 : 1, i % 3 === 0 ? 0.1 : 1);
  }
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
  geometry.setAttribute("color", new THREE.Float32BufferAttribute(colours, 3));
  geometry.computeVertexNormals();
  // Petals and stems light from above whichever way the triangle winds.
  const normal = geometry.attributes.normal as THREE.BufferAttribute;
  for (let i = 0; i < normal.count; i += 1) normal.setXYZ(i, 0, 1, 0);
  return geometry;
}

export interface GroundCover {
  group: THREE.Group;
  /** 0..1 overall fade, from the caller's zoom-dependent policy. */
  setFade(fade: number): void;
  update(viewSpanScene: number, groundFootprintScale: number, targetX: number, targetZ: number): void;
  dispose(): void;
}

export function createGroundCover(metresToScene: number, sampler: GroundSampler): GroundCover {
  const group = new THREE.Group();
  const materials: THREE.MeshStandardMaterial[] = [];
  const geometries: THREE.BufferGeometry[] = [];
  const makeLayer = (geometry: THREE.BufferGeometry, capacity: number, roughness: number): Layer => {
    const material = new THREE.MeshStandardMaterial({ vertexColors: true, roughness, metalness: 0, side: THREE.DoubleSide, transparent: false });
    materials.push(material);
    geometries.push(geometry);
    const mesh = new THREE.InstancedMesh(geometry, material, capacity);
    mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    mesh.count = 0;
    mesh.frustumCulled = false;
    mesh.castShadow = false;
    mesh.receiveShadow = true;
    group.add(mesh);
    return { mesh, capacity, count: 0 };
  };
  const tufts = makeLayer(tuftGeometry(metresToScene), 40_000, 1.0);
  const shrubs = makeLayer(shrubGeometry(metresToScene), 4_000, 0.95);
  const flowers = makeLayer(flowerGeometry(metresToScene), 9_000, 0.8);
  const layers = [tufts, shrubs, flowers];
  group.visible = false;

  let centreX = Number.POSITIVE_INFINITY;
  let centreZ = Number.POSITIVE_INFINITY;
  let builtRadius = 0;
  let builtSpacing = 0;
  let fade = 0;

  const matrix = new THREE.Matrix4();
  const quaternion = new THREE.Quaternion();
  const euler = new THREE.Euler();
  const position = new THREE.Vector3();
  const scale = new THREE.Vector3();
  const colour = new THREE.Color();
  const yAxis = new THREE.Vector3(0, 1, 0);

  // Flower palette: real meadow flowers, not confetti.
  const petals = [0xf4f0e6, 0xf6d84a, 0xd9558f, 0x8a6bd0, 0xf08a3c];

  const push = (layer: Layer, m: THREE.Matrix4, c: THREE.Color) => {
    if (layer.count >= layer.capacity) return false;
    layer.mesh.setMatrixAt(layer.count, m);
    layer.mesh.setColorAt(layer.count, c);
    layer.count += 1;
    return true;
  };

  const update = (viewSpanScene: number, groundFootprintScale: number, targetX: number, targetZ: number) => {
    const spanMetres = (viewSpanScene / metresToScene) * groundFootprintScale;
    // Population disc: as wide as the eye can resolve blades, no wider.
    const wantedRadius = THREE.MathUtils.clamp(spanMetres * 0.62, 10, 130);
    const spacing = wantedRadius <= 22 ? 0.42 : wantedRadius <= 50 ? 0.75 : wantedRadius <= 90 ? 1.25 : 1.9;
    const moved = Math.hypot(targetX - centreX, targetZ - centreZ) / metresToScene;
    const radiusChange = Math.abs(wantedRadius - builtRadius) / Math.max(1, builtRadius);
    if (moved < spacing * 4 && radiusChange < 0.12 && builtSpacing === spacing) return;
    centreX = targetX;
    centreZ = targetZ;
    builtRadius = wantedRadius;
    builtSpacing = spacing;
    for (const layer of layers) layer.count = 0;

    const centreMetresX = (centreX + 1.6) / metresToScene;
    const centreMetresZ = (centreZ + 1.6) / metresToScene;
    const minCellX = Math.floor((centreMetresX - wantedRadius) / spacing);
    const maxCellX = Math.ceil((centreMetresX + wantedRadius) / spacing);
    const minCellZ = Math.floor((centreMetresZ - wantedRadius) / spacing);
    const maxCellZ = Math.ceil((centreMetresZ + wantedRadius) / spacing);
    const h = sampler.hash01;
    const probeMetres = Math.max(1.5, spacing);
    const probe = probeMetres * metresToScene;
    // Cell coordinates for the per-patch decisions (meadow dryness, thickets).
    const patchMetres = 34;

    for (let cellZ = minCellZ; cellZ <= maxCellZ; cellZ += 1) {
      for (let cellX = minCellX; cellX <= maxCellX; cellX += 1) {
        const metresX = (cellX + 0.08 + h(cellX, cellZ, 101) * 0.84) * spacing;
        const metresZ = (cellZ + 0.08 + h(cellX, cellZ, 102) * 0.84) * spacing;
        const dx = metresX - centreMetresX;
        const dz = metresZ - centreMetresZ;
        const distance = Math.hypot(dx, dz);
        if (distance > wantedRadius) continue;
        const worldX = -1.6 + metresX * metresToScene;
        const worldZ = -1.6 + metresZ * metresToScene;
        if (worldX <= -1.6 || worldX >= 1.6 || worldZ <= -1.6 || worldZ >= 1.6) continue;
        if (sampler.excluded(worldX, worldZ) > 48) continue;
        if (sampler.water(worldX, worldZ) > 24) continue;
        const elevation = sampler.elevation01(worldX, worldZ);
        if (elevation > 0.80) continue;
        const height = sampler.height(worldX, worldZ);
        const slope = Math.hypot(
          sampler.height(worldX + probe, worldZ) - sampler.height(worldX - probe, worldZ),
          sampler.height(worldX, worldZ + probe) - sampler.height(worldX, worldZ - probe),
        ) / (2 * probe);
        if (slope > 0.85) continue;
        const forest = sampler.forest(worldX, worldZ) / 255;

        // Patches: dry meadow, lush hollows, thickets and bare soil, decided per
        // ~34 m cell so a field has regions instead of uniform stipple.
        const patchX = Math.floor(metresX / patchMetres);
        const patchZ = Math.floor(metresZ / patchMetres);
        const dryness = h(patchX, patchZ, 111);
        const bare = h(patchX, patchZ, 112) > 0.90;
        const thicket = h(patchX, patchZ, 113) > 0.86;
        // Distance thinning: nothing pops at the rim, everything fades out in the last quarter.
        const rim = 1 - THREE.MathUtils.smoothstep(distance, wantedRadius * 0.72, wantedRadius);
        if (rim <= 0.02) continue;
        // Steeper ground and forest floor are sparser; bare patches nearly empty.
        const cover = (bare ? 0.10 : 1.0) * (1 - THREE.MathUtils.smoothstep(slope, 0.35, 0.85)) * (1 - forest * 0.45);
        if (h(cellX, cellZ, 103) > cover * rim) continue;

        const yaw = h(cellX, cellZ, 104) * Math.PI * 2;
        const roll = (h(cellX, cellZ, 105) - 0.5) * 0.22;
        quaternion.setFromEuler(euler.set(roll, yaw, roll * 0.7));
        const size = (0.70 + h(cellX, cellZ, 106) * 0.85) * (0.6 + 0.4 * rim) * (1.12 - dryness * 0.32);
        position.set(worldX, height, worldZ);
        scale.set(size * (0.85 + h(cellX, cellZ, 107) * 0.5), size, size * (0.85 + h(cellX, cellZ, 108) * 0.5));
        matrix.compose(position, quaternion, scale);
        // Lush green through dry straw, with per-plant lightness noise.
        colour.setHSL(
          0.245 - dryness * 0.125 + (h(cellX, cellZ, 109) - 0.5) * 0.03,
          0.34 + (1 - dryness) * 0.18,
          0.25 + h(cellX, cellZ, 110) * 0.14 + dryness * 0.05,
        );
        push(tufts, matrix, colour);

        // Flowers scatter in the greener meadow patches only.
        if (!bare && dryness < 0.55 && forest < 0.5 && h(cellX, cellZ, 120) > 0.93 && rim > 0.4) {
          const flowerYaw = h(cellX, cellZ, 121) * Math.PI * 2;
          quaternion.setFromAxisAngle(yAxis, flowerYaw);
          position.set(worldX + (h(cellX, cellZ, 122) - 0.5) * spacing, height, worldZ + (h(cellX, cellZ, 123) - 0.5) * spacing);
          const flowerSize = 0.8 + h(cellX, cellZ, 124) * 0.7;
          scale.set(flowerSize, flowerSize, flowerSize);
          matrix.compose(position, quaternion, scale);
          // A patch of one colour: the flower colour is decided per patch.
          colour.setHex(petals[Math.floor(h(patchX, patchZ, 125) * petals.length) % petals.length]);
          push(flowers, matrix, colour);
        }

        // Shrubs: singles in scrub, many in thickets, some at forest edges.
        const shrubChance = (thicket ? 0.16 : 0.006) + forest * 0.03;
        if (!bare && h(cellX, cellZ, 130) < shrubChance && rim > 0.55 && slope < 0.6) {
          const shrubSize = (0.7 + h(cellX, cellZ, 131) * 1.5) * (thicket ? 1.1 : 0.9);
          quaternion.setFromAxisAngle(yAxis, h(cellX, cellZ, 132) * Math.PI * 2);
          position.set(worldX, height - 0.04 * metresToScene, worldZ);
          scale.set(shrubSize * (0.9 + h(cellX, cellZ, 133) * 0.4), shrubSize * (0.75 + h(cellX, cellZ, 134) * 0.5), shrubSize * (0.9 + h(cellX, cellZ, 135) * 0.4));
          matrix.compose(position, quaternion, scale);
          colour.setHSL(0.27 - dryness * 0.09 + (h(cellX, cellZ, 136) - 0.5) * 0.05, 0.40, 0.13 + h(cellX, cellZ, 137) * 0.09);
          push(shrubs, matrix, colour);
        }
      }
    }
    for (const layer of layers) {
      layer.mesh.count = layer.count;
      layer.mesh.instanceMatrix.needsUpdate = true;
      if (layer.mesh.instanceColor) layer.mesh.instanceColor.needsUpdate = true;
    }
  };

  return {
    group,
    setFade(value: number) {
      fade = value;
      group.visible = fade > 0.001;
      // Blades shrink toward the fade's end rather than blending (no sorting cost):
      // the population disc is rebuilt smaller as the camera pulls back.
    },
    update(viewSpanScene, groundFootprintScale, targetX, targetZ) {
      if (fade <= 0.001) return;
      update(viewSpanScene, groundFootprintScale, targetX, targetZ);
    },
    dispose() {
      for (const geometry of geometries) geometry.dispose();
      for (const material of materials) material.dispose();
      group.removeFromParent();
    },
  };
}
