import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import type { GenerationResult, SimulationConfig } from "../types";

function decodeHeights(base64: string): Float32Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return new Float32Array(bytes.buffer);
}

function decodeBytes(base64: string): Uint8Array {
  const binary = atob(base64);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

const cloudVertexShader = `
  varying vec2 vUv;
  void main() {
    vUv = uv;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;

const cloudFragmentShader = `
  precision highp float;
  varying vec2 vUv;
  uniform float uTime;
  uniform float uCoverage;
  uniform vec2 uWind;
  uniform vec3 uSunColor;
  uniform float uShadowMode;

  float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.55;
    mat2 rotation = mat2(0.82, -0.57, 0.57, 0.82);
    for (int i = 0; i < 6; i++) {
      value += noise(p) * amplitude;
      p = rotation * p * 2.03 + 9.17;
      amplitude *= 0.5;
    }
    return value;
  }
  void main() {
    vec2 drift = uWind * uTime;
    vec2 p = vUv * 4.2 + drift;
    float broad = fbm(p * 0.72);
    float detail = fbm(p * 2.1 + broad * 1.7);
    float field = broad * 0.72 + detail * 0.28;
    float threshold = mix(0.91, 0.43, uCoverage);
    float density = smoothstep(threshold, threshold + 0.16, field);
    density *= smoothstep(0.0, 0.10, vUv.x) * smoothstep(0.0, 0.10, vUv.y);
    density *= smoothstep(0.0, 0.10, 1.0 - vUv.x) * smoothstep(0.0, 0.10, 1.0 - vUv.y);
    float silver = smoothstep(threshold + 0.02, threshold + 0.19, fbm(p + vec2(-0.11, 0.08)));
    vec3 shadow = vec3(0.43, 0.49, 0.50);
    vec3 light = mix(vec3(0.88, 0.91, 0.90), uSunColor, 0.22);
    vec3 color = mix(shadow, light, 0.55 + silver * 0.45);
    if (uShadowMode > 0.5) {
      gl_FragColor = vec4(vec3(0.015, 0.025, 0.022), density * 0.34);
    } else {
      gl_FragColor = vec4(color, density * 0.76);
    }
  }
`;

const waterVertexShader = `
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform float uMetresToScene;
  uniform vec2 uWind;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;

  float vertexHash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float vertexNoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(vertexHash(i), vertexHash(i + vec2(1.0, 0.0)), f.x),
               mix(vertexHash(i + vec2(0.0, 1.0)), vertexHash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float vertexFbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.58;
    mat2 rotation = mat2(0.81, -0.59, 0.59, 0.81);
    for (int i = 0; i < 3; i++) {
      value += vertexNoise(p) * amplitude;
      p = rotation * p * 2.03 + vec2(5.7, -3.2);
      amplitude *= 0.47;
    }
    return value;
  }

  void main() {
    vUv = uv;
    vec4 hydrology = texture2D(uHydrologyMap, uv);
    float water = hydrology.r;
    float waterKind = hydrology.g;
    float flowStrength = hydrology.b;
    vec2 flowSample = texture2D(uFlowMap, uv).rg * 2.0 - 1.0;
    vec3 displaced = position;
    float ocean = smoothstep(0.72, 0.96, waterKind);
    float lake = smoothstep(0.26, 0.48, waterKind) * (1.0 - smoothstep(0.62, 0.82, waterKind));
    float river = 1.0 - smoothstep(0.18, 0.38, waterKind);
    vec2 wind = normalize(uWind + vec2(0.0001));
    vec2 flow = normalize(flowSample + vec2(0.0001));
    float oceanWave = (vertexFbm(displaced.xy * 6.5 + wind * uTime * 0.32) - 0.50) * 7.5;
    float lakeWave = (vertexFbm(displaced.xy * 13.0 + wind * uTime * 0.20 + vec2(9.0, 4.0)) - 0.50) * 0.42;
    vec2 channel = vec2(dot(displaced.xy, flow), dot(displaced.xy, vec2(-flow.y, flow.x)));
    float riverWave = (vertexFbm(channel * vec2(18.0, 38.0) + vec2(-uTime * (0.8 + flowStrength), 0.0)) - 0.50) * 0.16;
    displaced.z += water * uMetresToScene * (ocean * oceanWave + lake * lakeWave + river * riverWave);
    vec4 world = modelMatrix * vec4(displaced, 1.0);
    vWorldPosition = world.xyz;
    gl_Position = projectionMatrix * viewMatrix * world;
  }
`;

const waterFragmentShader = `
  precision highp float;
  varying vec2 vUv;
  varying vec3 vWorldPosition;
  uniform float uTime;
  uniform vec2 uWind;
  uniform vec3 uSunDirection;
  uniform sampler2D uHydrologyMap;
  uniform sampler2D uFlowMap;

  float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
  }
  float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.54;
    mat2 rotation = mat2(0.80, -0.60, 0.60, 0.80);
    for (int i = 0; i < 4; i++) {
      value += noise(p) * amplitude;
      p = rotation * p * 2.07 + vec2(7.3, -4.1);
      amplitude *= 0.48;
    }
    return value;
  }
  float warpedSurface(vec2 p) {
    vec2 warp = vec2(fbm(p * 0.37 + vec2(11.2, -3.7)), fbm(p * 0.37 + vec2(-8.4, 6.1)));
    vec2 domain = p + (warp - 0.5) * 2.4;
    return fbm(domain) * 0.72 + fbm(domain * 2.73 + vec2(19.0, -13.0)) * 0.28;
  }
  float waterSurface(vec2 uv, float kind, vec2 wind, vec2 flow, float strength) {
    if (kind > 0.72) {
      return warpedSurface(uv * 22.0 + wind * uTime * 0.085);
    }
    if (kind > 0.24) {
      return warpedSurface(uv * 31.0 + wind * uTime * 0.045 + vec2(17.0, 9.0));
    }
    vec2 across = vec2(-flow.y, flow.x);
    vec2 channel = vec2(dot(uv, flow), dot(uv, across));
    return warpedSurface(channel * vec2(48.0, 96.0) + vec2(-uTime * (0.22 + strength * 0.34), 0.0));
  }

  void main() {
    vec4 hydrology = texture2D(uHydrologyMap, vUv);
    float water = hydrology.r;
    float waterKind = hydrology.g;
    float flowStrength = hydrology.b;
    vec2 flowSample = texture2D(uFlowMap, vUv).rg * 2.0 - 1.0;
    if (water < 0.16) discard;
    float ocean = smoothstep(0.72, 0.96, waterKind);
    float lake = smoothstep(0.26, 0.48, waterKind) * (1.0 - smoothstep(0.62, 0.82, waterKind));
    float river = 1.0 - smoothstep(0.18, 0.38, waterKind);
    vec2 wind = normalize(uWind + vec2(0.0001));
    vec2 flow = normalize(flowSample + vec2(0.0001));
    float footprint = max(length(fwidth(vUv)) * 1.5, 0.0012);
    float centre = waterSurface(vUv, waterKind, wind, flow, flowStrength);
    float sampleX = waterSurface(vUv + vec2(footprint, 0.0), waterKind, wind, flow, flowStrength);
    float sampleY = waterSurface(vUv + vec2(0.0, footprint), waterKind, wind, flow, flowStrength);
    float normalStrength = ocean * 0.00165 + lake * 0.0012 + river * (0.0009 + flowStrength * 0.0011);
    vec2 gradient = vec2(sampleX - centre, sampleY - centre) / footprint * normalStrength;
    vec3 normal = normalize(vec3(-gradient.x, 1.0, -gradient.y));
    vec3 viewDirection = normalize(cameraPosition - vWorldPosition);
    float fresnel = pow(1.0 - max(dot(normal, viewDirection), 0.0), 4.0);
    vec3 reflectedSun = reflect(-normalize(uSunDirection), normal);
    float specular = pow(max(dot(reflectedSun, viewDirection), 0.0), 38.0);
    float glint = pow(max(dot(reflectedSun, viewDirection), 0.0), 120.0);
    vec3 base = vec3(0.018, 0.105, 0.135) * ocean
      + vec3(0.035, 0.145, 0.155) * lake
      + vec3(0.075, 0.175, 0.155) * river;
    base = mix(base, vec3(0.39, 0.53, 0.56), fresnel * 0.56);
    base += vec3(1.0, 0.91, 0.72) * specular * 0.10 + vec3(1.0) * glint * 0.13;
    float edgeFoam = smoothstep(0.16, 0.42, water) * (1.0 - smoothstep(0.42, 0.72, water));
    float streamFoam = river * flowStrength * smoothstep(0.73, 0.92, centre);
    base = mix(base, vec3(0.72, 0.79, 0.75), edgeFoam * ocean * 0.38 + streamFoam * 0.14);
    float alpha = mix(0.52, 0.82, ocean) * smoothstep(0.12, 0.58, water);
    gl_FragColor = vec4(base, alpha);
  }
`;

export function Terrain3D({ result, config, cameraMode }: { result: GenerationResult; config: SimulationConfig; cameraMode: "3d" | "satellite" }) {
  const hostRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x9aa9a4);
    const baseFogDensity = 0.085 + config.haze * 0.006;
    scene.fog = new THREE.FogExp2(0x9aa9a4, baseFogDensity);

    const camera = cameraMode === "satellite"
      ? new THREE.OrthographicCamera(-1.7, 1.7, 1.7, -1.7, 0.000005, 30)
      : new THREE.PerspectiveCamera(42, 1, 0.000005, 30);
    if (cameraMode === "satellite") {
      camera.position.set(0, 5.2, 0.001);
      camera.up.set(0, 0, -1);
    } else {
      camera.position.set(3.15, 1.65, 3.15);
    }

    const renderer = new THREE.WebGLRenderer({ antialias: true, powerPreference: "high-performance" });
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.08;
    host.appendChild(renderer.domElement);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.065;
    controls.zoomToCursor = true;
    controls.screenSpacePanning = true;
    controls.target.set(0, 0.055, 0);
    if (cameraMode === "satellite") {
      controls.enableRotate = false;
      controls.enablePan = true;
      controls.minZoom = 0.75;
      controls.maxZoom = 16_384;
    } else {
      // Replaced below with a real-metre value once the world scale is known.
      controls.minDistance = 0.00008;
      controls.maxDistance = 7.5;
      controls.maxPolarAngle = Math.PI * 0.495;
    }

    const heights = decodeHeights(result.heightDataBase64);
    const geometry = new THREE.PlaneGeometry(3.2, 3.2, result.meshSize - 1, result.meshSize - 1);
    const positions = geometry.attributes.position as THREE.BufferAttribute;
    const metresToScene = 3.2 / (result.worldSizeKm * 1000);
    const sceneToMetres = 1 / metresToScene;
    const terrainHalfExtent = 1.6;
    const terrainSampleMargin = 3.2 / Math.max(2, result.meshSize - 1) * 0.5;
    const minimumEyeClearance = 1.7 * metresToScene;
    const targetClearance = 0.35 * metresToScene;
    if (camera instanceof THREE.PerspectiveCamera) {
      // OrbitControls distances are scene units; deriving them from metres keeps
      // navigation identical for a 20 km tile and a 160 km tile.
      controls.minDistance = 2.2 * metresToScene;
      controls.maxDistance = Math.max(7.5, result.worldSizeKm * 1000 * metresToScene * 2.4);
    }
    for (let index = 0; index < heights.length; index += 1) {
      const relativeElevation = Math.max(0, heights[index] - result.stats.minElevation);
      positions.setZ(index, relativeElevation * metresToScene);
    }
    positions.needsUpdate = true;
    geometry.computeVertexNormals();

    const texture = new THREE.TextureLoader().load(result.previewDataUrl);
    texture.colorSpace = THREE.SRGBColorSpace;
    texture.anisotropy = renderer.capabilities.getMaxAnisotropy();
    const terrainMaterial = new THREE.MeshStandardMaterial({ map: texture, roughness: 0.96, metalness: 0.0 });
    const terrain = new THREE.Mesh(geometry, terrainMaterial);
    terrain.rotation.x = -Math.PI / 2;
    scene.add(terrain);

    const forestValues = decodeBytes(result.forestDataBase64);
    const vegetationExclusion = decodeBytes(result.vegetationExclusionDataBase64);
    const treeGeometry = new THREE.ConeGeometry(3.8 * metresToScene, 18.0 * metresToScene, 5, 1);
    const treeMaterial = new THREE.MeshStandardMaterial({ color: 0x1f4b2c, roughness: 0.94 });
    const treeMatrices: THREE.Matrix4[] = [];
    const treeColors: THREE.Color[] = [];
    const hash01 = (x: number, y: number, salt: number) => {
      const value = Math.sin(x * 127.1 + y * 311.7 + salt * 74.7 + config.seed * 0.013) * 43758.5453;
      return value - Math.floor(value);
    };
    for (let y = 1; y + 1 < result.meshSize; y += 1) {
      for (let x = 1; x + 1 < result.meshSize; x += 1) {
        const index = y * result.meshSize + x;
        const density = forestValues[index] / 255;
        if (vegetationExclusion[index] > 32 || density < 0.38 || hash01(x, y, 1) > density * 0.62 || treeMatrices.length >= 45_000) continue;
        const jitterX = (hash01(x, y, 2) - 0.5) * 0.86;
        const jitterY = (hash01(x, y, 3) - 0.5) * 0.86;
        const worldX = -1.6 + (x + jitterX) / (result.meshSize - 1) * 3.2;
        const worldZ = -1.6 + (y + jitterY) / (result.meshSize - 1) * 3.2;
        const treeHeightScale = 0.68 + hash01(x, y, 4) * 0.72;
        const matrix = new THREE.Matrix4();
        matrix.compose(
          new THREE.Vector3(
            worldX,
            Math.max(0, heights[index] - result.stats.minElevation) * metresToScene + 9.0 * metresToScene * treeHeightScale,
            worldZ,
          ),
          new THREE.Quaternion().setFromAxisAngle(new THREE.Vector3(0, 1, 0), hash01(x, y, 5) * Math.PI * 2),
          new THREE.Vector3(0.75 + hash01(x, y, 6) * 0.55, treeHeightScale, 0.75 + hash01(x, y, 7) * 0.55),
        );
        treeMatrices.push(matrix);
        treeColors.push(new THREE.Color().setHSL(0.30 + hash01(x, y, 8) * 0.045, 0.36, 0.15 + hash01(x, y, 9) * 0.09));
      }
    }
    const trees = new THREE.InstancedMesh(treeGeometry, treeMaterial, treeMatrices.length);
    for (let index = 0; index < treeMatrices.length; index += 1) {
      trees.setMatrixAt(index, treeMatrices[index]);
      trees.setColorAt(index, treeColors[index]);
    }
    trees.instanceMatrix.needsUpdate = true;
    if (trees.instanceColor) trees.instanceColor.needsUpdate = true;
    scene.add(trees);

    // A whole-world tree mesh cannot retain real tree density over hundreds of
    // kilometres. These two instance layers act as a small vegetation clipmap:
    // the texture supplies the canopy at satellite scale, the mesh above marks
    // regional stands, and this local layer streams individual trees around the
    // camera target. World-aligned cells keep trees stable while panning.
    const detailTreeCapacity = 32_000;
    const detailCrownGeometry = new THREE.ConeGeometry(4.6 * metresToScene, 13.5 * metresToScene, 7, 2);
    const detailTrunkGeometry = new THREE.CylinderGeometry(0.48 * metresToScene, 0.68 * metresToScene, 7.0 * metresToScene, 6);
    const detailCrownMaterial = new THREE.MeshStandardMaterial({ color: 0x285c31, roughness: 0.95 });
    const detailTrunkMaterial = new THREE.MeshStandardMaterial({ color: 0x59432d, roughness: 1.0 });
    const detailCrowns = new THREE.InstancedMesh(detailCrownGeometry, detailCrownMaterial, detailTreeCapacity);
    const detailTrunks = new THREE.InstancedMesh(detailTrunkGeometry, detailTrunkMaterial, detailTreeCapacity);
    detailCrowns.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    detailTrunks.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    detailCrowns.count = 0;
    detailTrunks.count = 0;
    scene.add(detailTrunks, detailCrowns);

    const sampleGrid = (values: ArrayLike<number>, worldX: number, worldZ: number) => {
      const gridX = THREE.MathUtils.clamp((worldX + 1.6) / 3.2 * (result.meshSize - 1), 0, result.meshSize - 1);
      const gridY = THREE.MathUtils.clamp((worldZ + 1.6) / 3.2 * (result.meshSize - 1), 0, result.meshSize - 1);
      const x0 = Math.floor(gridX);
      const y0 = Math.floor(gridY);
      const x1 = Math.min(result.meshSize - 1, x0 + 1);
      const y1 = Math.min(result.meshSize - 1, y0 + 1);
      const tx = gridX - x0;
      const ty = gridY - y0;
      const top = THREE.MathUtils.lerp(values[y0 * result.meshSize + x0], values[y0 * result.meshSize + x1], tx);
      const bottom = THREE.MathUtils.lerp(values[y1 * result.meshSize + x0], values[y1 * result.meshSize + x1], tx);
      return THREE.MathUtils.lerp(top, bottom, ty);
    };

    // Crop detail is streamed around the camera target. The satellite texture
    // remains the far LOD; these meshes only exist where individual rows and
    // plants are large enough to contribute pixels.
    const cultivatedValues = decodeBytes(result.cultivatedDataBase64);
    const cropDataBase64 = (result as GenerationResult & { cropDataBase64?: string }).cropDataBase64;
    const cropValues = cropDataBase64 ? decodeBytes(cropDataBase64) : undefined;
    const sampleGridNearest = (values: ArrayLike<number>, worldX: number, worldZ: number) => {
      const gridX = THREE.MathUtils.clamp(Math.round((worldX + 1.6) / 3.2 * (result.meshSize - 1)), 0, result.meshSize - 1);
      const gridY = THREE.MathUtils.clamp(Math.round((worldZ + 1.6) / 3.2 * (result.meshSize - 1)), 0, result.meshSize - 1);
      return values[gridY * result.meshSize + gridX];
    };
    const cropAt = (worldX: number, worldZ: number) => {
      if (cropValues?.length === cultivatedValues.length) return sampleGridNearest(cropValues, worldX, worldZ);
      if (sampleGrid(cultivatedValues, worldX, worldZ) < 38) return 0;
      const metresX = (worldX + 1.6) / metresToScene;
      const metresZ = (worldZ + 1.6) / metresToScene;
      const parcelX = Math.floor(metresX / 180);
      const parcelZ = Math.floor(metresZ / 180);
      return hash01(parcelX, parcelZ, 71) < 0.58 ? 1 : 2;
    };

    const cropRowCapacity = 14_000;
    const cropRowGeometry = new THREE.BoxGeometry(1, 1, 1);
    const cropRowMaterial = new THREE.MeshStandardMaterial({ color: 0xa7a844, roughness: 1.0 });
    const cropRows = new THREE.InstancedMesh(cropRowGeometry, cropRowMaterial, cropRowCapacity);
    cropRows.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
    cropRows.count = 0;
    cropRows.frustumCulled = false;
    scene.add(cropRows);

    const wheatCapacity = 30_000;
    const cornCapacity = 14_000;
    const grassCapacity = 24_000;
    const kernelCapacity = 6_000;
    const wheatStemGeometry = new THREE.CylinderGeometry(0.008 * metresToScene, 0.012 * metresToScene, 0.82 * metresToScene, 4);
    wheatStemGeometry.translate(0, 0.41 * metresToScene, 0);
    const wheatHeadGeometry = new THREE.CylinderGeometry(0.018 * metresToScene, 0.028 * metresToScene, 0.15 * metresToScene, 5);
    wheatHeadGeometry.translate(0, 0.88 * metresToScene, 0);
    const cornStemGeometry = new THREE.CylinderGeometry(0.018 * metresToScene, 0.028 * metresToScene, 2.05 * metresToScene, 5);
    cornStemGeometry.translate(0, 1.025 * metresToScene, 0);
    // Two crossed tapered triangles read as leaves without multiplying the
    // draw count per plant.
    const cornLeafGeometry = new THREE.BufferGeometry();
    cornLeafGeometry.setAttribute("position", new THREE.Float32BufferAttribute([
      0, 0.65, 0, 0.34, 0.98, 0.035, 0, 0.88, 0,
      0, 0.90, 0, -0.31, 1.28, -0.025, 0, 1.13, 0,
      0, 1.16, 0, 0.035, 1.52, 0.32, 0, 1.38, 0,
      0, 1.38, 0, -0.02, 1.73, -0.27, 0, 1.57, 0,
    ].map((value) => value * metresToScene), 3));
    cornLeafGeometry.computeVertexNormals();
    const cornCobGeometry = new THREE.CylinderGeometry(0.036 * metresToScene, 0.052 * metresToScene, 0.20 * metresToScene, 8);
    cornCobGeometry.translate(0.065 * metresToScene, 1.20 * metresToScene, 0);
    const kernelGeometry = new THREE.SphereGeometry(0.012 * metresToScene, 4, 3);
    const grassGeometry = new THREE.BufferGeometry();
    grassGeometry.setAttribute("position", new THREE.Float32BufferAttribute([
      -0.018, 0, 0, 0.018, 0, 0, 0, 0.42, 0,
      0, 0, -0.018, 0, 0, 0.018, 0, 0.42, 0,
    ].map((value) => value * metresToScene), 3));
    grassGeometry.computeVertexNormals();
    const wheatMaterial = new THREE.MeshStandardMaterial({ color: 0xb8a448, roughness: 0.96 });
    const wheatHeadMaterial = new THREE.MeshStandardMaterial({ color: 0xc8ad54, roughness: 0.91 });
    const cornMaterial = new THREE.MeshStandardMaterial({ color: 0x44762c, roughness: 0.96 });
    const cornLeafMaterial = new THREE.MeshStandardMaterial({ color: 0x3d762c, roughness: 0.98, side: THREE.DoubleSide });
    const cornCobMaterial = new THREE.MeshStandardMaterial({ color: 0xc6a529, roughness: 0.88 });
    const kernelMaterial = new THREE.MeshStandardMaterial({ color: 0xe5c340, roughness: 0.82 });
    const grassMaterial = new THREE.MeshStandardMaterial({ color: 0x698a3c, roughness: 1.0, side: THREE.DoubleSide });
    const wheatStems = new THREE.InstancedMesh(wheatStemGeometry, wheatMaterial, wheatCapacity);
    const wheatHeads = new THREE.InstancedMesh(wheatHeadGeometry, wheatHeadMaterial, wheatCapacity);
    const cornStems = new THREE.InstancedMesh(cornStemGeometry, cornMaterial, cornCapacity);
    const cornLeaves = new THREE.InstancedMesh(cornLeafGeometry, cornLeafMaterial, cornCapacity);
    const cornCobs = new THREE.InstancedMesh(cornCobGeometry, cornCobMaterial, cornCapacity);
    const cornKernels = new THREE.InstancedMesh(kernelGeometry, kernelMaterial, kernelCapacity);
    const grassBlades = new THREE.InstancedMesh(grassGeometry, grassMaterial, grassCapacity);
    const cropPlantMeshes = [grassBlades, wheatStems, wheatHeads, cornStems, cornLeaves, cornCobs, cornKernels];
    for (const mesh of cropPlantMeshes) {
      mesh.instanceMatrix.setUsage(THREE.DynamicDrawUsage);
      mesh.count = 0;
      mesh.frustumCulled = false;
      scene.add(mesh);
    }

    let cropDetailCentreX = Number.POSITIVE_INFINITY;
    let cropDetailCentreZ = Number.POSITIVE_INFINITY;
    let cropDetailRadiusMetres = 0;
    let cropDetailSpacingMetres = 0;
    const cropMatrix = new THREE.Matrix4();
    const cropQuaternion = new THREE.Quaternion();
    const cropPosition = new THREE.Vector3();
    const cropScale = new THREE.Vector3();
    const cropAxis = new THREE.Vector3(0, 1, 0);
    const updateCropDetail = (viewSpanScene: number, groundFootprintScale: number) => {
      const spanMetres = viewSpanScene / metresToScene * groundFootprintScale;
      const showPlants = spanMetres < 520;
      const wantedRadius = showPlants
        ? THREE.MathUtils.clamp(spanMetres * 0.62, 12, 230)
        : THREE.MathUtils.clamp(spanMetres * 0.58, 100, 1_150);
      const spacingMetres = showPlants
        ? (wantedRadius <= 24 ? 0.28 : wantedRadius <= 55 ? 0.55 : wantedRadius <= 120 ? 1.15 : 2.1)
        : (wantedRadius <= 320 ? 3.2 : wantedRadius <= 650 ? 5.5 : 9.0);
      const moveMetres = Math.hypot(controls.target.x - cropDetailCentreX, controls.target.z - cropDetailCentreZ) / metresToScene;
      const radiusChange = Math.abs(wantedRadius - cropDetailRadiusMetres) / Math.max(1, cropDetailRadiusMetres);
      if (moveMetres < spacingMetres * 3 && radiusChange < 0.12 && cropDetailSpacingMetres === spacingMetres) return;
      cropDetailCentreX = controls.target.x;
      cropDetailCentreZ = controls.target.z;
      cropDetailRadiusMetres = wantedRadius;
      cropDetailSpacingMetres = spacingMetres;
      const centreMetresX = (cropDetailCentreX + 1.6) / metresToScene;
      const centreMetresZ = (cropDetailCentreZ + 1.6) / metresToScene;
      let rowCount = 0;
      let grassCount = 0;
      let wheatCount = 0;
      let cornCount = 0;
      let kernelCount = 0;
      const minCellX = Math.floor((centreMetresX - wantedRadius) / spacingMetres);
      const maxCellX = Math.ceil((centreMetresX + wantedRadius) / spacingMetres);
      const minCellZ = Math.floor((centreMetresZ - wantedRadius) / spacingMetres);
      const maxCellZ = Math.ceil((centreMetresZ + wantedRadius) / spacingMetres);
      for (let cellZ = minCellZ; cellZ <= maxCellZ; cellZ += 1) {
        for (let cellX = minCellX; cellX <= maxCellX; cellX += 1) {
          const metresX = (cellX + 0.12 + hash01(cellX, cellZ, 72) * 0.76) * spacingMetres;
          const metresZ = (cellZ + 0.12 + hash01(cellX, cellZ, 73) * 0.76) * spacingMetres;
          const dx = metresX - centreMetresX;
          const dz = metresZ - centreMetresZ;
          if (dx * dx + dz * dz > wantedRadius * wantedRadius) continue;
          const worldX = -1.6 + metresX * metresToScene;
          const worldZ = -1.6 + metresZ * metresToScene;
          if (worldX <= -1.6 || worldX >= 1.6 || worldZ <= -1.6 || worldZ >= 1.6) continue;
          const crop = cropAt(worldX, worldZ);
          if (crop < 1 || crop > 3) continue;
          if (sampleGrid(vegetationExclusion, worldX, worldZ) > 48) continue;
          const terrainHeight = Math.max(0, sampleGrid(heights, worldX, worldZ) - result.stats.minElevation) * metresToScene;
          const parcelX = Math.floor(metresX / 180);
          const parcelZ = Math.floor(metresZ / 180);
          const rowAngle = hash01(parcelX, parcelZ, 74) * Math.PI;
          cropQuaternion.setFromAxisAngle(cropAxis, rowAngle + (hash01(cellX, cellZ, 75) - 0.5) * 0.035);

          if (!showPlants) {
            if (rowCount >= cropRowCapacity) continue;
            const segmentLength = spacingMetres * (2.2 + hash01(cellX, cellZ, 76) * 1.4);
            cropPosition.set(worldX, terrainHeight + (crop === 1 ? 0.34 : crop === 2 ? 0.72 : 0.18) * metresToScene, worldZ);
            cropScale.set(segmentLength * metresToScene, (crop === 1 ? 0.55 : crop === 2 ? 1.25 : 0.30) * metresToScene, 0.24 * metresToScene);
            cropMatrix.compose(cropPosition, cropQuaternion, cropScale);
            cropRows.setMatrixAt(rowCount, cropMatrix);
            cropRows.setColorAt(rowCount, new THREE.Color(crop === 1 ? 0xa9a143 : crop === 2 ? 0x4f7a2e : 0x71883c));
            rowCount += 1;
            continue;
          }

          const heightScale = 0.86 + hash01(cellX, cellZ, 77) * 0.26;
          const lean = (hash01(cellX, cellZ, 78) - 0.5) * 0.08;
          cropQuaternion.setFromEuler(new THREE.Euler(lean, rowAngle + hash01(cellX, cellZ, 79) * 0.12, lean * 0.6));
          cropPosition.set(worldX, terrainHeight, worldZ);
          cropScale.set(1, heightScale, 1);
          cropMatrix.compose(cropPosition, cropQuaternion, cropScale);
          if (crop === 3 && grassCount < grassCapacity) {
            grassBlades.setMatrixAt(grassCount, cropMatrix);
            grassBlades.setColorAt(grassCount, new THREE.Color().setHSL(
              0.22 + hash01(cellX, cellZ, 80) * 0.06,
              0.38,
              0.28 + hash01(cellX, cellZ, 81) * 0.12,
            ));
            grassCount += 1;
          } else if (crop === 1 && wheatCount < wheatCapacity) {
            wheatStems.setMatrixAt(wheatCount, cropMatrix);
            wheatHeads.setMatrixAt(wheatCount, cropMatrix);
            wheatCount += 1;
          } else if (crop === 2 && cornCount < cornCapacity) {
            cornStems.setMatrixAt(cornCount, cropMatrix);
            cornLeaves.setMatrixAt(cornCount, cropMatrix);
            cornCobs.setMatrixAt(cornCount, cropMatrix);
            // Individual kernels are reserved for the nearest fourteen metres;
            // farther away the faceted cob is the correct pixel-budget LOD.
            if (dx * dx + dz * dz < 14 * 14) {
              for (let kernel = 0; kernel < 12 && kernelCount < kernelCapacity; kernel += 1) {
                const ring = kernel % 6;
                const tier = Math.floor(kernel / 6);
                const angle = rowAngle + ring / 6 * Math.PI * 2;
                cropPosition.set(
                  worldX + Math.cos(angle) * 0.054 * metresToScene,
                  terrainHeight + (1.145 + tier * 0.07) * metresToScene * heightScale,
                  worldZ + Math.sin(angle) * 0.054 * metresToScene,
                );
                cropScale.set(0.78, 1.18, 0.78);
                cropMatrix.compose(cropPosition, cropQuaternion.identity(), cropScale);
                cornKernels.setMatrixAt(kernelCount, cropMatrix);
                kernelCount += 1;
              }
            }
            cornCount += 1;
          }
        }
      }
      cropRows.count = rowCount;
      grassBlades.count = grassCount;
      wheatStems.count = wheatCount;
      wheatHeads.count = wheatCount;
      cornStems.count = cornCount;
      cornLeaves.count = cornCount;
      cornCobs.count = cornCount;
      cornKernels.count = kernelCount;
      for (const mesh of [cropRows, ...cropPlantMeshes]) {
        mesh.instanceMatrix.needsUpdate = true;
        if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
      }
    };
    const terrainHeightAt = (worldX: number, worldZ: number) => (
      Math.max(0, sampleGrid(heights, worldX, worldZ) - result.stats.minElevation) * metresToScene
    );

    // City generators provide real kilometre footprints and metre heights.
    // Build those exact polygons rather than scattering illustrative boxes.
    // Each style is merged into one draw call; geometry remains 1:1 because
    // horizontal coordinates and extrusion depth use the same conversion.
    const cityGroup = new THREE.Group();
    const buildingGeometries: THREE.BufferGeometry[] = [];
    const mergedBuildingGeometries: THREE.BufferGeometry[] = [];
    const buildingMaterials = {
      parisian: new THREE.MeshStandardMaterial({ color: 0xb8aa91, roughness: 0.91 }),
      barcelonaEixample: new THREE.MeshStandardMaterial({ color: 0xb98769, roughness: 0.94 }),
      manhattan: new THREE.MeshStandardMaterial({ color: 0x9da3a2, roughness: 0.87 }),
    };
    const geometriesByStyle: Record<keyof typeof buildingMaterials, THREE.BufferGeometry[]> = {
      parisian: [],
      barcelonaEixample: [],
      manhattan: [],
    };
    const kilometreToScene = 1000 * metresToScene;
    const cityHalfExtentKm = result.worldSizeKm * 0.5;
    let buildingCount = 0;
    for (const city of result.cities ?? []) {
      const style = city.style;
      if (!(style in geometriesByStyle)) continue;
      for (const building of city.buildings) {
        if (buildingCount >= 8_000 || building.footprint.length < 3) break;
        const footprint = building.footprint.map((point) => ({
          x: (point.x_km - cityHalfExtentKm) * kilometreToScene,
          z: (point.y_km - cityHalfExtentKm) * kilometreToScene,
        }));
        const shape = new THREE.Shape();
        shape.moveTo(footprint[0].x, -footprint[0].z);
        for (const point of footprint.slice(1)) shape.lineTo(point.x, -point.z);
        shape.closePath();
        if (building.courtyard && building.courtyard.length >= 3) {
          const hole = new THREE.Path();
          const courtyard = building.courtyard.map((point) => ({
            x: (point.x_km - cityHalfExtentKm) * kilometreToScene,
            z: (point.y_km - cityHalfExtentKm) * kilometreToScene,
          }));
          hole.moveTo(courtyard[0].x, -courtyard[0].z);
          for (const point of courtyard.slice(1)) hole.lineTo(point.x, -point.z);
          hole.closePath();
          shape.holes.push(hole);
        }
        const centreX = footprint.reduce((sum, point) => sum + point.x, 0) / footprint.length;
        const centreZ = footprint.reduce((sum, point) => sum + point.z, 0) / footprint.length;
        const buildingGeometry = new THREE.ExtrudeGeometry(shape, {
          depth: building.height_metres * metresToScene,
          bevelEnabled: false,
          curveSegments: 1,
          steps: 1,
        });
        buildingGeometry.rotateX(-Math.PI / 2);
        buildingGeometry.translate(0, terrainHeightAt(centreX, centreZ) + 0.08 * metresToScene, 0);
        buildingGeometry.computeVertexNormals();
        buildingGeometries.push(buildingGeometry);
        geometriesByStyle[style].push(buildingGeometry);
        buildingCount += 1;
      }
    }
    for (const style of Object.keys(geometriesByStyle) as Array<keyof typeof buildingMaterials>) {
      const parts = geometriesByStyle[style];
      if (parts.length === 0) continue;
      const merged = mergeGeometries(parts, false);
      if (!merged) continue;
      merged.computeBoundingSphere();
      mergedBuildingGeometries.push(merged);
      cityGroup.add(new THREE.Mesh(merged, buildingMaterials[style]));
    }
    cityGroup.visible = false;
    scene.add(cityGroup);

    // Roads remain vectors until this near-field LOD. Their physical widths
    // are converted with the same metres-to-scene factor as elevation, so a
    // 7.2 m collector is 7.2 m wide from both an overhead and street camera.
    type RoadSample = { x: number; y: number; z: number; nx: number; nz: number; distanceMetres: number };
    const roadSurfaceGroup = new THREE.Group();
    const roadMarkingGroup = new THREE.Group();
    const asphaltMaterial = new THREE.MeshStandardMaterial({
      color: 0x4d4e4c,
      roughness: 0.93,
      polygonOffset: true,
      polygonOffsetFactor: -2,
      polygonOffsetUnits: -2,
    });
    const dirtRoadMaterial = new THREE.MeshStandardMaterial({
      color: 0x796447,
      roughness: 1.0,
      polygonOffset: true,
      polygonOffsetFactor: -2,
      polygonOffsetUnits: -2,
    });
    const whiteMarkingMaterial = new THREE.MeshBasicMaterial({
      color: 0xe7e4d8,
      polygonOffset: true,
      polygonOffsetFactor: -4,
      polygonOffsetUnits: -4,
    });
    const yellowMarkingMaterial = new THREE.MeshBasicMaterial({
      color: 0xd8ad32,
      polygonOffset: true,
      polygonOffsetFactor: -4,
      polygonOffsetUnits: -4,
    });
    const roadGeometries: THREE.BufferGeometry[] = [];
    const roadHalfExtentKm = result.worldSizeKm * 0.5;

    const prepareRoadSamples = (pathKm: [number, number][]) => {
      if (pathKm.length < 2) return [] as RoadSample[];
      let refined = pathKm.map(([x, z]) => [x, z] as [number, number]);
      for (let pass = 0; pass < 2; pass += 1) {
        const next: [number, number][] = [refined[0]];
        for (let index = 0; index + 1 < refined.length; index += 1) {
          const a = refined[index];
          const b = refined[index + 1];
          next.push([a[0] * 0.75 + b[0] * 0.25, a[1] * 0.75 + b[1] * 0.25]);
          next.push([a[0] * 0.25 + b[0] * 0.75, a[1] * 0.25 + b[1] * 0.75]);
        }
        next.push(refined[refined.length - 1]);
        refined = next;
      }
      const points: Array<{ x: number; y: number; z: number; distanceMetres: number }> = [];
      let distanceMetres = 0;
      for (let index = 0; index + 1 < refined.length; index += 1) {
        const a = refined[index];
        const b = refined[index + 1];
        const lengthMetres = Math.hypot(b[0] - a[0], b[1] - a[1]) * 1000;
        const steps = Math.max(1, Math.ceil(lengthMetres / 80));
        for (let step = index === 0 ? 0 : 1; step <= steps; step += 1) {
          const t = step / steps;
          const kmX = THREE.MathUtils.lerp(a[0], b[0], t);
          const kmZ = THREE.MathUtils.lerp(a[1], b[1], t);
          const x = (kmX - roadHalfExtentKm) * 1000 * metresToScene;
          const z = (kmZ - roadHalfExtentKm) * 1000 * metresToScene;
          if (points.length > 0) {
            const previous = points[points.length - 1];
            distanceMetres += Math.hypot(x - previous.x, z - previous.z) * sceneToMetres;
          }
          points.push({ x, z, y: terrainHeightAt(x, z) + 0.06 * metresToScene, distanceMetres });
        }
      }
      return points.map((point, index) => {
        const previous = points[Math.max(0, index - 1)];
        const next = points[Math.min(points.length - 1, index + 1)];
        const length = Math.max(1.0e-9, Math.hypot(next.x - previous.x, next.z - previous.z));
        return { ...point, nx: -(next.z - previous.z) / length, nz: (next.x - previous.x) / length };
      });
    };

    const solidRibbon = (samples: RoadSample[], widthMetres: number, offsetMetres = 0) => {
      const positions: number[] = [];
      const indices: number[] = [];
      const halfWidth = widthMetres * metresToScene * 0.5;
      const offset = offsetMetres * metresToScene;
      for (const sample of samples) {
        const centreX = sample.x + sample.nx * offset;
        const centreZ = sample.z + sample.nz * offset;
        positions.push(
          centreX + sample.nx * halfWidth, sample.y, centreZ + sample.nz * halfWidth,
          centreX - sample.nx * halfWidth, sample.y, centreZ - sample.nz * halfWidth,
        );
      }
      for (let index = 0; index + 1 < samples.length; index += 1) {
        const vertex = index * 2;
        indices.push(vertex, vertex + 2, vertex + 1, vertex + 1, vertex + 2, vertex + 3);
      }
      const ribbon = new THREE.BufferGeometry();
      ribbon.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
      ribbon.setIndex(indices);
      ribbon.computeVertexNormals();
      roadGeometries.push(ribbon);
      return ribbon;
    };

    const dashedRibbon = (samples: RoadSample[], widthMetres: number, offsetMetres: number) => {
      const positions: number[] = [];
      const indices: number[] = [];
      const halfWidth = widthMetres * metresToScene * 0.5;
      for (let index = 0; index + 1 < samples.length; index += 1) {
        const a = samples[index];
        const b = samples[index + 1];
        const segmentLength = b.distanceMetres - a.distanceMetres;
        let cursor = a.distanceMetres;
        while (cursor < b.distanceMetres) {
          const cycle = cursor % 9;
          const dashStart = cycle < 3 ? cursor : cursor + (9 - cycle);
          const dashEnd = Math.min(b.distanceMetres, dashStart + 3);
          if (dashStart >= b.distanceMetres || dashEnd <= dashStart) break;
          const t0 = (dashStart - a.distanceMetres) / Math.max(segmentLength, 1.0e-6);
          const t1 = (dashEnd - a.distanceMetres) / Math.max(segmentLength, 1.0e-6);
          const base = positions.length / 3;
          for (const t of [t0, t1]) {
            const nx = THREE.MathUtils.lerp(a.nx, b.nx, t);
            const nz = THREE.MathUtils.lerp(a.nz, b.nz, t);
            const normalLength = Math.max(1.0e-9, Math.hypot(nx, nz));
            const unitX = nx / normalLength;
            const unitZ = nz / normalLength;
            const centreX = THREE.MathUtils.lerp(a.x, b.x, t) + unitX * offsetMetres * metresToScene;
            const centreZ = THREE.MathUtils.lerp(a.z, b.z, t) + unitZ * offsetMetres * metresToScene;
            const y = THREE.MathUtils.lerp(a.y, b.y, t) + 0.018 * metresToScene;
            positions.push(
              centreX + unitX * halfWidth, y, centreZ + unitZ * halfWidth,
              centreX - unitX * halfWidth, y, centreZ - unitZ * halfWidth,
            );
          }
          indices.push(base, base + 2, base + 1, base + 1, base + 2, base + 3);
          cursor = dashEnd + 6;
        }
      }
      const ribbon = new THREE.BufferGeometry();
      ribbon.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
      ribbon.setIndex(indices);
      ribbon.computeVertexNormals();
      roadGeometries.push(ribbon);
      return ribbon;
    };

    for (const road of result.roads ?? []) {
      const samples = prepareRoadSamples(road.pathKm);
      if (samples.length < 2) continue;
      const surface = new THREE.Mesh(
        solidRibbon(samples, road.profile.carriagewayWidthMetres),
        road.profile.paved ? asphaltMaterial : dirtRoadMaterial,
      );
      surface.renderOrder = 3;
      roadSurfaceGroup.add(surface);
      const addMarking = (geometry: THREE.BufferGeometry, material: THREE.Material) => {
        const marking = new THREE.Mesh(geometry, material);
        marking.renderOrder = 4;
        roadMarkingGroup.add(marking);
      };
      if (road.class === "motorway") {
        addMarking(dashedRibbon(samples, 0.15, -road.profile.carriagewayWidthMetres * 0.25), whiteMarkingMaterial);
        addMarking(dashedRibbon(samples, 0.15, road.profile.carriagewayWidthMetres * 0.25), whiteMarkingMaterial);
        addMarking(solidRibbon(samples, 0.14, -0.16), yellowMarkingMaterial);
        addMarking(solidRibbon(samples, 0.14, 0.16), yellowMarkingMaterial);
      } else if (road.class === "arterial") {
        addMarking(solidRibbon(samples, 0.12, -0.14), yellowMarkingMaterial);
        addMarking(solidRibbon(samples, 0.12, 0.14), yellowMarkingMaterial);
      } else if (road.class === "collector") {
        addMarking(dashedRibbon(samples, 0.12, 0), yellowMarkingMaterial);
      }
    }
    roadSurfaceGroup.visible = false;
    roadMarkingGroup.visible = false;
    scene.add(roadSurfaceGroup, roadMarkingGroup);

    // Start focused on the actual surface. The previous fixed target represented
    // 1.4 km at the default world size and made close zoom orbit empty air.
    const initialTargetHeight = terrainHeightAt(controls.target.x, controls.target.z);
    controls.target.y = initialTargetHeight + targetClearance;
    if (camera instanceof THREE.PerspectiveCamera) {
      camera.position.y = Math.max(camera.position.y, controls.target.y + minimumEyeClearance);
    }
    controls.update();

    let detailCentreX = Number.POSITIVE_INFINITY;
    let detailCentreZ = Number.POSITIVE_INFINITY;
    let detailRadiusMetres = 0;
    const localMatrix = new THREE.Matrix4();
    const localQuaternion = new THREE.Quaternion();
    const localScale = new THREE.Vector3();
    const localPosition = new THREE.Vector3();
    const localColour = new THREE.Color();
    const treeRotationAxis = new THREE.Vector3(0, 1, 0);
    // Quantised spacings form world-aligned LOD rings. A continuously changing
    // spacing makes every tree jump whenever the camera zooms; these levels
    // retain the same positions throughout each useful scale band.
    const detailSpacingForRadius = (radiusMetres: number) => {
      if (radiusMetres <= 420) return 8;
      if (radiusMetres <= 900) return 12;
      if (radiusMetres <= 1_800) return 20;
      if (radiusMetres <= 3_600) return 36;
      // 8 km / 80 m keeps the complete circular patch below the 32k
      // allocation even in a fully forested region (no directional truncation).
      return 80;
    };
    const updateDetailForest = (viewSpanScene: number, groundFootprintScale: number) => {
      const wantedRadiusMetres = THREE.MathUtils.clamp(
        viewSpanScene / metresToScene * 0.72 * groundFootprintScale,
        180,
        8_000,
      );
      const spacingMetres = detailSpacingForRadius(wantedRadiusMetres);
      const moveMetres = Math.hypot(controls.target.x - detailCentreX, controls.target.z - detailCentreZ) / metresToScene;
      const radiusChange = Math.abs(wantedRadiusMetres - detailRadiusMetres) / Math.max(1, detailRadiusMetres);
      if (moveMetres < spacingMetres * 5 && radiusChange < 0.14) return;

      detailCentreX = controls.target.x;
      detailCentreZ = controls.target.z;
      detailRadiusMetres = wantedRadiusMetres;
      const centreMetresX = (detailCentreX + 1.6) / metresToScene;
      const centreMetresZ = (detailCentreZ + 1.6) / metresToScene;
      const minCellX = Math.floor((centreMetresX - wantedRadiusMetres) / spacingMetres);
      const maxCellX = Math.ceil((centreMetresX + wantedRadiusMetres) / spacingMetres);
      const minCellZ = Math.floor((centreMetresZ - wantedRadiusMetres) / spacingMetres);
      const maxCellZ = Math.ceil((centreMetresZ + wantedRadiusMetres) / spacingMetres);
      let count = 0;

      for (let cellZ = minCellZ; cellZ <= maxCellZ && count < detailTreeCapacity; cellZ += 1) {
        for (let cellX = minCellX; cellX <= maxCellX && count < detailTreeCapacity; cellX += 1) {
          const jitterX = hash01(cellX, cellZ, 31);
          const jitterZ = hash01(cellX, cellZ, 32);
          const metresX = (cellX + 0.08 + jitterX * 0.84) * spacingMetres;
          const metresZ = (cellZ + 0.08 + jitterZ * 0.84) * spacingMetres;
          const dx = metresX - centreMetresX;
          const dz = metresZ - centreMetresZ;
          if (dx * dx + dz * dz > wantedRadiusMetres * wantedRadiusMetres) continue;
          const worldX = -1.6 + metresX * metresToScene;
          const worldZ = -1.6 + metresZ * metresToScene;
          if (worldX <= -1.6 || worldX >= 1.6 || worldZ <= -1.6 || worldZ >= 1.6) continue;
          const density = sampleGrid(forestValues, worldX, worldZ) / 255;
          const excluded = sampleGrid(vegetationExclusion, worldX, worldZ) / 255;
          if (excluded > 0.12) continue;
          // Eight-metre near-field cells yield plausible closed forest density
          // while the coarser LODs deliberately thin into individual crowns.
          const probability = THREE.MathUtils.clamp((density - 0.18) * 1.45, 0, 0.96);
          if (hash01(cellX, cellZ, 33) > probability) continue;

          const terrainHeight = Math.max(0, sampleGrid(heights, worldX, worldZ) - result.stats.minElevation) * metresToScene;
          const heightScale = 0.68 + hash01(cellX, cellZ, 34) * 0.76;
          const widthScale = 0.72 + hash01(cellX, cellZ, 35) * 0.58;
          localQuaternion.setFromAxisAngle(treeRotationAxis, hash01(cellX, cellZ, 36) * Math.PI * 2);

          localPosition.set(worldX, terrainHeight + 3.5 * metresToScene * heightScale, worldZ);
          localScale.set(0.78 + widthScale * 0.22, heightScale, 0.78 + widthScale * 0.22);
          localMatrix.compose(localPosition, localQuaternion, localScale);
          detailTrunks.setMatrixAt(count, localMatrix);

          localPosition.y = terrainHeight + (7.0 + 6.75) * metresToScene * heightScale;
          localScale.set(widthScale, heightScale, widthScale);
          localMatrix.compose(localPosition, localQuaternion, localScale);
          detailCrowns.setMatrixAt(count, localMatrix);
          localColour.setHSL(0.295 + hash01(cellX, cellZ, 37) * 0.055, 0.38, 0.17 + hash01(cellX, cellZ, 38) * 0.10);
          detailCrowns.setColorAt(count, localColour);
          count += 1;
        }
      }
      detailCrowns.count = count;
      detailTrunks.count = count;
      detailCrowns.instanceMatrix.needsUpdate = true;
      detailTrunks.instanceMatrix.needsUpdate = true;
      if (detailCrowns.instanceColor) detailCrowns.instanceColor.needsUpdate = true;
      // Dynamic instance matrices need fresh bounds before Three.js can cull
      // the streamed patch. Without this, all 32k slots are submitted forever.
      detailCrowns.computeBoundingSphere();
      detailTrunks.computeBoundingSphere();
    };

    const ambient = new THREE.HemisphereLight(0xd9e7e2, 0x283129, 1.7);
    scene.add(ambient);
    const azimuth = THREE.MathUtils.degToRad(config.sunAzimuth);
    const elevation = THREE.MathUtils.degToRad(config.sunElevation);
    const sun = new THREE.DirectionalLight(0xfff0d0, 3.4);
    sun.position.set(Math.sin(azimuth) * Math.cos(elevation) * 5, Math.sin(elevation) * 5, Math.cos(azimuth) * Math.cos(elevation) * 5);
    scene.add(sun);

    const windAngle = THREE.MathUtils.degToRad(config.windDirection);
    const waterHeights = decodeHeights(result.waterHeightDataBase64);
    const waterMask = decodeBytes(result.waterMaskBase64);
    const waterKind = decodeBytes(result.waterKindBase64);
    const flowDirection = new Int8Array(decodeBytes(result.flowDirectionBase64).buffer);
    const flowStrength = decodeBytes(result.flowStrengthBase64);
    const waterGeometry = new THREE.PlaneGeometry(3.2, 3.2, result.meshSize - 1, result.meshSize - 1);
    const waterPositions = waterGeometry.attributes.position as THREE.BufferAttribute;
    for (let index = 0; index < waterHeights.length; index += 1) {
      const surfaceElevation = Math.max(0, waterHeights[index] - result.stats.minElevation);
      waterPositions.setZ(index, surfaceElevation * metresToScene + metresToScene * 0.18);
    }
    waterPositions.needsUpdate = true;
    const hydrologyPixels = new Uint8Array(waterMask.length * 4);
    const flowPixels = new Uint8Array(waterMask.length * 4);
    for (let index = 0; index < waterMask.length; index += 1) {
      hydrologyPixels[index * 4] = waterMask[index];
      hydrologyPixels[index * 4 + 1] = waterKind[index];
      hydrologyPixels[index * 4 + 2] = flowStrength[index];
      hydrologyPixels[index * 4 + 3] = 255;
      flowPixels[index * 4] = Math.round((flowDirection[index * 2] / 127 * 0.5 + 0.5) * 255);
      flowPixels[index * 4 + 1] = Math.round((flowDirection[index * 2 + 1] / 127 * 0.5 + 0.5) * 255);
      flowPixels[index * 4 + 2] = 128;
      flowPixels[index * 4 + 3] = 255;
    }
    const hydrologyTexture = new THREE.DataTexture(hydrologyPixels, result.waterDataSize, result.waterDataSize, THREE.RGBAFormat);
    hydrologyTexture.minFilter = THREE.LinearFilter;
    hydrologyTexture.magFilter = THREE.LinearFilter;
    hydrologyTexture.wrapS = THREE.ClampToEdgeWrapping;
    hydrologyTexture.wrapT = THREE.ClampToEdgeWrapping;
    hydrologyTexture.flipY = true;
    hydrologyTexture.needsUpdate = true;
    const flowTexture = new THREE.DataTexture(flowPixels, result.waterDataSize, result.waterDataSize, THREE.RGBAFormat);
    flowTexture.minFilter = THREE.LinearFilter;
    flowTexture.magFilter = THREE.LinearFilter;
    flowTexture.wrapS = THREE.ClampToEdgeWrapping;
    flowTexture.wrapT = THREE.ClampToEdgeWrapping;
    flowTexture.flipY = true;
    flowTexture.needsUpdate = true;
    const waterUniforms = {
      uTime: { value: 0 },
      uMetresToScene: { value: metresToScene },
      uWind: { value: new THREE.Vector2(Math.cos(windAngle), Math.sin(windAngle)) },
      uSunDirection: { value: sun.position.clone().normalize() },
      uHydrologyMap: { value: hydrologyTexture },
      uFlowMap: { value: flowTexture },
    };
    const waterMaterial = new THREE.ShaderMaterial({
      vertexShader: waterVertexShader,
      fragmentShader: waterFragmentShader,
      uniforms: waterUniforms,
      transparent: true,
      depthWrite: false,
      polygonOffset: true,
      polygonOffsetFactor: -1,
      polygonOffsetUnits: -2,
      side: THREE.DoubleSide,
    });
    const water = new THREE.Mesh(waterGeometry, waterMaterial);
    water.rotation.x = -Math.PI / 2;
    water.renderOrder = 2;
    scene.add(water);

    const weatherWind = new THREE.Vector2(Math.cos(windAngle), Math.sin(windAngle));
    const cloudUniforms = {
      uTime: { value: 0 },
      uCoverage: { value: config.cloudCoverage / 100 },
      uWind: { value: weatherWind.clone().multiplyScalar(config.cloudSpeed * 0.0006) },
      uSunColor: { value: new THREE.Color(1.0, 0.91, 0.76) },
      uShadowMode: { value: 0 },
    };
    const cloudMaterial = new THREE.ShaderMaterial({
      vertexShader: cloudVertexShader,
      fragmentShader: cloudFragmentShader,
      uniforms: cloudUniforms,
      transparent: true,
      depthWrite: false,
      side: THREE.DoubleSide,
    });
    const cloudGeometry = new THREE.PlaneGeometry(4.4, 4.4, 1, 1);
    const clouds = new THREE.Mesh(cloudGeometry, cloudMaterial);
    clouds.rotation.x = -Math.PI / 2;
    const reliefMetres = result.stats.maxElevation - result.stats.minElevation;
    clouds.position.y = (reliefMetres + 1800) * metresToScene;
    scene.add(clouds);

    const shadowUniforms = {
      uTime: { value: 0 },
      uCoverage: { value: config.cloudCoverage / 100 },
      uWind: { value: weatherWind.clone().multiplyScalar(config.cloudSpeed * 0.0006) },
      uSunColor: { value: new THREE.Color(1, 1, 1) },
      uShadowMode: { value: 1 },
    };
    const shadowMaterial = new THREE.ShaderMaterial({
      vertexShader: cloudVertexShader,
      fragmentShader: cloudFragmentShader,
      uniforms: shadowUniforms,
      transparent: true,
      depthWrite: false,
      side: THREE.DoubleSide,
      blending: THREE.MultiplyBlending,
    });
    const shadowGeometry = new THREE.PlaneGeometry(3.28, 3.28, 1, 1);
    const cloudShadows = new THREE.Mesh(shadowGeometry, shadowMaterial);
    cloudShadows.rotation.x = -Math.PI / 2;
    cloudShadows.position.y = (reliefMetres + 35) * metresToScene;
    cloudShadows.renderOrder = 1;
    cloudShadows.visible = cameraMode === "satellite";
    scene.add(cloudShadows);

    const resize = () => {
      const width = Math.max(1, host.clientWidth);
      const height = Math.max(1, host.clientHeight);
      const aspect = width / height;
      if (camera instanceof THREE.PerspectiveCamera) {
        camera.aspect = aspect;
      } else {
        const halfHeight = 1.7;
        camera.left = -halfHeight * aspect;
        camera.right = halfHeight * aspect;
        camera.top = halfHeight;
        camera.bottom = -halfHeight;
      }
      camera.updateProjectionMatrix();
      renderer.setSize(width, height, false);
    };
    const observer = new ResizeObserver(resize);
    observer.observe(host);
    resize();

    const clock = new THREE.Clock();
    const cameraDirection = new THREE.Vector3();
    let worldTime = 0;
    let frame = 0;
    const animate = () => {
      const delta = clock.getDelta();
      worldTime += delta;
      const weatherFront = Math.sin(worldTime * 0.035 + config.seed * 0.0017);
      const gust = 0.82 + Math.sin(worldTime * 0.21 + config.seed * 0.013) * 0.18;
      const liveCoverage = THREE.MathUtils.clamp(config.cloudCoverage / 100 + weatherFront * 0.11, 0, 1);
      cloudUniforms.uTime.value += delta;
      cloudUniforms.uCoverage.value = liveCoverage;
      cloudUniforms.uWind.value.copy(weatherWind).multiplyScalar(config.cloudSpeed * 0.0006 * gust);
      shadowUniforms.uTime.value = cloudUniforms.uTime.value;
      shadowUniforms.uCoverage.value = liveCoverage;
      shadowUniforms.uWind.value.copy(cloudUniforms.uWind.value);
      waterUniforms.uTime.value += delta;
      const liveAzimuth = azimuth + worldTime * 0.0015;
      const liveElevation = elevation + Math.sin(worldTime * 0.012) * 0.035;
      sun.position.set(
        Math.sin(liveAzimuth) * Math.cos(liveElevation) * 5,
        Math.sin(liveElevation) * 5,
        Math.cos(liveAzimuth) * Math.cos(liveElevation) * 5,
      );
      sun.intensity = 3.4 * (1.0 - liveCoverage * 0.28);
      waterUniforms.uSunDirection.value.copy(sun.position).normalize();
      (scene.fog as THREE.FogExp2).density = baseFogDensity * (1.0 + Math.max(0, weatherFront) * 0.32);
      controls.update();
      if (camera instanceof THREE.PerspectiveCamera) {
        // Keep the orbit focus attached to the terrain while panning, and keep
        // the eye above the sampled surface. This is collision against the same
        // height field as the rendered mesh, not an arbitrary altitude floor.
        controls.target.x = THREE.MathUtils.clamp(
          controls.target.x,
          -terrainHalfExtent + terrainSampleMargin,
          terrainHalfExtent - terrainSampleMargin,
        );
        controls.target.z = THREE.MathUtils.clamp(
          controls.target.z,
          -terrainHalfExtent + terrainSampleMargin,
          terrainHalfExtent - terrainSampleMargin,
        );
        controls.target.y = terrainHeightAt(controls.target.x, controls.target.z) + targetClearance;
        const eyeTerrainHeight = terrainHeightAt(camera.position.x, camera.position.z);
        if (camera.position.y < eyeTerrainHeight + minimumEyeClearance) {
          camera.position.y = eyeTerrainHeight + minimumEyeClearance;
        }
        const distance = camera.position.distanceTo(controls.target);
        const wantedNear = THREE.MathUtils.clamp(
          distance / 900,
          0.05 * metresToScene,
          40 * metresToScene,
        );
        const reliefScene = Math.max(0, result.stats.maxElevation - result.stats.minElevation) * metresToScene;
        const wantedFar = THREE.MathUtils.clamp(
          distance * 8 + reliefScene * 2,
          2_000 * metresToScene,
          30,
        );
        if (
          Math.abs(camera.near - wantedNear) / wantedNear > 0.12
          || Math.abs(camera.far - wantedFar) / wantedFar > 0.12
        ) {
          camera.near = wantedNear;
          camera.far = wantedFar;
          camera.updateProjectionMatrix();
        }
      }
      const viewSpanScene = camera instanceof THREE.OrthographicCamera
        ? 3.4 / camera.zoom
        : camera.position.distanceTo(controls.target) * Math.tan(THREE.MathUtils.degToRad(camera.fov * 0.5)) * 2;
      camera.getWorldDirection(cameraDirection);
      const groundFootprintScale = camera instanceof THREE.PerspectiveCamera
        ? Math.min(3, 1 / Math.max(0.34, Math.abs(cameraDirection.y)))
        : 1;
      const viewSpanMetres = viewSpanScene * sceneToMetres;
      const showDetailTrees = viewSpanMetres < 5_000;
      detailCrowns.visible = showDetailTrees;
      detailTrunks.visible = showDetailTrees;
      // Do not draw the sparse regional markers over the detailed trees: two
      // unrelated representations occupying one stand cause doubled crowns.
      trees.visible = viewSpanMetres >= 5_000 && viewSpanMetres < 18_000;
      if (showDetailTrees) updateDetailForest(viewSpanScene, groundFootprintScale);
      const cropSpanMetres = viewSpanScene / metresToScene * groundFootprintScale;
      const showCropRows = cropSpanMetres >= 520 && cropSpanMetres < 2_200;
      const showCropPlants = cropSpanMetres < 520;
      cropRows.visible = showCropRows;
      for (const mesh of cropPlantMeshes) mesh.visible = showCropPlants;
      if (showCropRows || showCropPlants) updateCropDetail(viewSpanScene, groundFootprintScale);
      // The baked satellite surface remains the far LOD. Vector ribbons only
      // take over when their real metre widths can occupy useful screen pixels.
      roadSurfaceGroup.visible = viewSpanMetres < 15_000;
      roadMarkingGroup.visible = viewSpanMetres < 3_200;
      cityGroup.visible = viewSpanMetres < 20_000;
      renderer.render(scene, camera);
      frame = requestAnimationFrame(animate);
    };
    animate();

    return () => {
      cancelAnimationFrame(frame);
      observer.disconnect();
      controls.dispose();
      texture.dispose();
      geometry.dispose();
      terrainMaterial.dispose();
      treeGeometry.dispose();
      treeMaterial.dispose();
      detailCrownGeometry.dispose();
      detailTrunkGeometry.dispose();
      detailCrownMaterial.dispose();
      detailTrunkMaterial.dispose();
      cropRowGeometry.dispose();
      cropRowMaterial.dispose();
      wheatStemGeometry.dispose();
      wheatHeadGeometry.dispose();
      cornStemGeometry.dispose();
      cornLeafGeometry.dispose();
      cornCobGeometry.dispose();
      kernelGeometry.dispose();
      grassGeometry.dispose();
      wheatMaterial.dispose();
      wheatHeadMaterial.dispose();
      cornMaterial.dispose();
      cornLeafMaterial.dispose();
      cornCobMaterial.dispose();
      kernelMaterial.dispose();
      grassMaterial.dispose();
      for (const buildingGeometry of buildingGeometries) buildingGeometry.dispose();
      for (const mergedGeometry of mergedBuildingGeometries) mergedGeometry.dispose();
      for (const material of Object.values(buildingMaterials)) material.dispose();
      for (const roadGeometry of roadGeometries) roadGeometry.dispose();
      asphaltMaterial.dispose();
      dirtRoadMaterial.dispose();
      whiteMarkingMaterial.dispose();
      yellowMarkingMaterial.dispose();
      waterGeometry.dispose();
      waterMaterial.dispose();
      hydrologyTexture.dispose();
      flowTexture.dispose();
      cloudGeometry.dispose();
      cloudMaterial.dispose();
      shadowGeometry.dispose();
      shadowMaterial.dispose();
      renderer.dispose();
      renderer.domElement.remove();
    };
  }, [result, cameraMode, config.cloudCoverage, config.cloudSpeed, config.haze, config.sunAzimuth, config.sunElevation, config.windDirection]);

  return <div ref={hostRef} className="terrain-3d" />;
}
