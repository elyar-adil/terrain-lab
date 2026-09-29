import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { EffectComposer } from "three/examples/jsm/postprocessing/EffectComposer.js";
import { RenderPass } from "three/examples/jsm/postprocessing/RenderPass.js";
import { OutputPass } from "three/examples/jsm/postprocessing/OutputPass.js";
import { GTAOPass } from "three/examples/jsm/postprocessing/GTAOPass.js";
import { UnrealBloomPass } from "three/examples/jsm/postprocessing/UnrealBloomPass.js";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import type { GenerationResult, SimulationConfig, UrbanBuilding, UrbanLane, UrbanPoint, UrbanRoad, VegetationPrototypePayload } from "../types";

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

// ---- 路口工坊天空移植:物理感穹顶 + 太阳圆盘 + FBM 漂移积云 ----
const SUN_DIR = new THREE.Vector3(-0.38, 0.74, -0.46).normalize();
const SKY_FOG = 0xc4d4e4;
const skyVertShader = `
  varying vec3 vDir;
  void main() {
    vDir = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;
const skyFragShader = `
  varying vec3 vDir;
  uniform vec3 uSunDir;
  uniform float uTime;

  float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
  float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float v = 0.0, a = 0.5;
    for (int i = 0; i < 6; i++) { v += a * vnoise(p); p = p * 2.07 + vec2(19.7, 7.3); a *= 0.5; }
    return v;
  }

  void main() {
    vec3 d = normalize(vDir);
    float y = d.y;
    vec3 zenith = vec3(0.070, 0.225, 0.565);
    vec3 horizon = vec3(0.46, 0.60, 0.76);
    vec3 col = mix(horizon, zenith, pow(clamp(y, 0.0, 1.0), 0.52));

    float s = max(dot(d, uSunDir), 0.0);
    col += vec3(1.0, 0.86, 0.62) * pow(s, 6.0) * 0.14;
    col += vec3(1.0, 0.94, 0.82) * pow(s, 220.0) * 2.6;

    if (y > 0.012) {
      vec2 uv = d.xz / (y + 0.14) * 4.5;
      vec2 drift = vec2(uTime * 0.0022, uTime * 0.0009);
      float warp = fbm(uv * 0.18 + drift);
      float c = fbm(uv * 0.34 + (warp - 0.5) * 2.4 + drift * 1.7);
      float cover = smoothstep(0.50, 0.76, c + 0.14 * pow(1.0 - y, 2.0));
      float fade = smoothstep(0.012, 0.10, y);
      float lit = smoothstep(0.42, 0.92, c);
      vec3 cloud = mix(vec3(0.56, 0.60, 0.68), vec3(0.98, 0.99, 1.03), lit);
      cloud += vec3(0.30, 0.24, 0.14) * pow(s, 3.0);
      col = mix(col, cloud, cover * fade * 0.94);
    }

    col = mix(col, horizon * vec3(1.04, 1.02, 0.99), smoothstep(0.045, -0.10, y));

    gl_FragColor = vec4(col, 1.0);
    #include <tonemapping_fragment>
    #include <colorspace_fragment>
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

export function Terrain3D({ result, config, cameraMode, cityFocus }: { result: GenerationResult; config: SimulationConfig; cameraMode: "3d" | "satellite"; cityFocus?: { xKm: number; yKm: number; spanKm: number; nonce: number } | null }) {
  const hostRef = useRef<HTMLDivElement>(null);
  // Camera jumps arrive as props; the render loop polls the ref so the heavy
  // scene is never rebuilt for a change of viewpoint.
  const cityFocusRef = useRef(cityFocus);
  cityFocusRef.current = cityFocus;

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;

    const scene = new THREE.Scene();
    const baseFogDensity = 0.085 + config.haze * 0.006;
    // 路口工坊移植:线性雾贴穹顶地平线色,距离随视野自适应(在 animate 中更新)。
    scene.fog = new THREE.Fog(SKY_FOG, 0.01, 1.0);
    void baseFogDensity;

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
    // 路口工坊移植:PCFSoft 阴影;太阳只随视野/时间低频刷新,不逐帧重绘。
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    renderer.shadowMap.autoUpdate = false;
    renderer.toneMappingExposure = 1.08;
    host.appendChild(renderer.domElement);

    // 路口工坊移植:写实天空穹顶(同一份材质烘成 PMREM 环境贴图,玻璃与
    // 水面反射真实天色),跟随相机的巨大球壳,renderOrder 最先绘制。
    const skyUniforms = { uSunDir: { value: SUN_DIR.clone() }, uTime: { value: 0 } };
    const skyMaterial = new THREE.ShaderMaterial({
      vertexShader: skyVertShader,
      fragmentShader: skyFragShader,
      uniforms: skyUniforms,
      side: THREE.BackSide,
      depthWrite: false,
      depthTest: false,
      fog: false,
    });
    const skyDome = new THREE.Mesh(new THREE.SphereGeometry(9000, 48, 28), skyMaterial);
    skyDome.frustumCulled = false;
    skyDome.renderOrder = -100;
    scene.add(skyDome);
    {
      const pmrem = new THREE.PMREMGenerator(renderer);
      const envScene = new THREE.Scene();
      const clone = new THREE.Mesh(new THREE.SphereGeometry(100, 32, 20), skyMaterial);
      clone.frustumCulled = false;
      envScene.add(clone);
      scene.environment = pmrem.fromScene(envScene, 0.05).texture;
      clone.geometry.dispose();
      pmrem.dispose();
    }
    scene.environmentIntensity = 0.5;
    renderer.toneMappingExposure = 1.05;

    // 冷色补光:阴影面不全是黑的,带一点天光蓝。
    const fillLight = new THREE.DirectionalLight(0xbcd6ff, 0.18);
    fillLight.position.set(0.5, 0.4, 0.6);
    scene.add(fillLight);

    // 路口工坊移植:HDR 后处理链(4x MSAA)→ GTAO 接触阴影 → 轻 bloom →
    // OutputPass。GTAO 是照片感的地基:建筑根部、路缘、树干与地面接触处
    // 的阴影不是模型自带的,少了它所有物体都像悬浮。
    let composer: THREE.WebGLRenderer | { render: () => void } | null = null;
    let gtaoPassRef: { enabled: boolean } | null = null;
    try {
      const composerTarget = new THREE.WebGLRenderTarget(2, 2, { type: THREE.HalfFloatType, samples: 4 });
      const post = new EffectComposer(renderer, composerTarget);
      post.setPixelRatio(renderer.getPixelRatio());
      const renderPass = new RenderPass(scene, camera);
      const gtaoPass = new GTAOPass(scene, camera, host.clientWidth || 2, host.clientHeight || 2);
      gtaoPass.output = GTAOPass.OUTPUT.Default;
      gtaoPass.blendIntensity = 0.85;
      const aoScale = 3.2 / (result.worldSizeKm * 1000);
      gtaoPass.updateGtaoMaterial({
        radius: 0.8 * aoScale,
        distanceExponent: 1.1,
        thickness: 1.4 * aoScale,
        scale: 1.0,
        samples: 16,
      });
      gtaoPass.updatePdMaterial({ lumaPhi: 10, depthPhi: 2, normalPhi: 3, radius: 4, rings: 2, samples: 8 });
      const bloomPass = new UnrealBloomPass(new THREE.Vector2(960, 540), 0.14, 0.5, 1.05);
      post.addPass(renderPass);
      post.addPass(gtaoPass);
      post.addPass(bloomPass);
      post.addPass(new OutputPass());
      post.setSize(host.clientWidth || 2, host.clientHeight || 2);
      composer = post as unknown as THREE.WebGLRenderer;
      gtaoPassRef = gtaoPass;
    } catch {
      composer = null;
      gtaoPassRef = null;
    }

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
      // Keep the camera continuous all the way down to a few centimetres. The
      // renderer streams the corresponding near-field LODs below, while the
      // baked surface remains the stable far-field representation. A large
      // finite value is preferable to an artificial "last zoom level":
      // OrbitControls still applies the same smooth exponential wheel motion.
      controls.maxZoom = 131_072;
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
      controls.minDistance = 1.2 * metresToScene;
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
    const cityFacadeTextures: THREE.DataTexture[] = [];
    const makeFacadeTexture = (base: [number, number, number], glass: [number, number, number], seed: number) => {
      const size = 128;
      const data = new Uint8Array(size * size * 4);
      for (let y = 0; y < size; y += 1) {
        for (let x = 0; x < size; x += 1) {
          const floorBand = y % 24;
          const bay = x % 18;
          const window = floorBand > 5 && floorBand < 18 && bay > 4 && bay < 14 && ((x + y + seed) % 11 !== 0);
          const mortar = floorBand === 0 || bay === 0;
          const noise = ((x * 17 + y * 31 + seed * 13) % 17) - 8;
          const i = (y * size + x) * 4;
          const source = window ? glass : base;
          data[i] = THREE.MathUtils.clamp(source[0] + noise + (mortar ? 7 : 0), 0, 255);
          data[i + 1] = THREE.MathUtils.clamp(source[1] + noise + (mortar ? 7 : 0), 0, 255);
          data[i + 2] = THREE.MathUtils.clamp(source[2] + noise + (mortar ? 7 : 0), 0, 255);
          data[i + 3] = 255;
        }
      }
      const texture = new THREE.DataTexture(data, size, size, THREE.RGBAFormat);
      texture.wrapS = THREE.RepeatWrapping;
      texture.wrapT = THREE.RepeatWrapping;
      texture.magFilter = THREE.LinearFilter;
      texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true;
      texture.colorSpace = THREE.SRGBColorSpace;
      texture.repeat.set(2.8, 3.8);
      texture.needsUpdate = true;
      cityFacadeTextures.push(texture);
      return texture;
    };
    // CPU-baked weathered textures from the Rust procedural kit carry the
    // grime, cracks and stains once for every surface; the local noise
    // generators remain as a fallback for older payloads.
    const bakedTextureCache = new Map<string, THREE.DataTexture>();
    const useBakedFacades = (result.materialTextures?.length ?? 0) > 0;
    const getBakedMaterialTexture = (name: string, repeat: [number, number], sink: THREE.DataTexture[]) => {
      if (!result.materialTextures?.length) return null;
      const cached = bakedTextureCache.get(`${name}|${repeat[0]}|${repeat[1]}`);
      if (cached) return cached;
      const payload = result.materialTextures.find((texture) => texture.name === name);
      if (!payload) return null;
      const binary = atob(payload.data);
      const bytes = new Uint8Array(binary.length);
      for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
      const texture = new THREE.DataTexture(bytes, payload.width, payload.height, THREE.RGBAFormat);
      texture.wrapS = THREE.RepeatWrapping;
      texture.wrapT = THREE.RepeatWrapping;
      texture.magFilter = THREE.LinearFilter;
      texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true;
      texture.colorSpace = THREE.SRGBColorSpace;
      texture.repeat.set(repeat[0], repeat[1]);
      texture.needsUpdate = true;
      bakedTextureCache.set(`${name}|${repeat[0]}|${repeat[1]}`, texture);
      sink.push(texture);
      return texture;
    };

    // One facade texture tile covers its designed real-world size (the
    // residential tile is 6 bays × 10 storeys ≈ 22 m × 30 m; the curtain
    // wall 8 bays × 6 storeys ≈ 43 m × 25 m), so metres-based UVs plus these
    // repeats reproduce the true floor and bay rhythm on every wall.
    const modernConcreteTexture = getBakedMaterialTexture("facade/residential", [1 / 22, 1 / 30], cityFacadeTextures)
      ?? makeFacadeTexture([144, 151, 150], [36, 66, 77], 3);
    const modernGlassTexture = getBakedMaterialTexture("facade/glass", [1 / 43, 1 / 25], cityFacadeTextures)
      ?? makeFacadeTexture([96, 122, 130], [18, 47, 59], 7);
    const modernStoneTexture = getBakedMaterialTexture("facade/stone", [1 / 7, 1 / 6], cityFacadeTextures)
      ?? makeFacadeTexture([177, 166, 144], [48, 66, 69], 11);
    const modernBrickTexture = getBakedMaterialTexture("facade/residential", [1 / 24, 1 / 32], cityFacadeTextures)
      ?? makeFacadeTexture([146, 108, 91], [42, 58, 63], 17);
    // 立面按 24 变体分桶合并;facadeMaterial 懒建,材质数组随用随增。
    const geometriesByStyle = new Map<string, THREE.BufferGeometry[]>();
    const facadeBucket = (variant: number) => {
      const key = `v${variant % FACADE_SPECS.length}`;
      let bucket = geometriesByStyle.get(key);
      if (!bucket) { bucket = []; geometriesByStyle.set(key, bucket); }
      return key;
    };
    const kilometreToScene = 1000 * metresToScene;
    const cityHalfExtentKm = result.worldSizeKm * 0.5;
    // ExtrudeGeometry's default UVs are in scene units, where a whole
    // 30 m wall spans ~0.002 UV — every building would sample a single
    // near-uniform texel and render as a flat colour block.  This generator
    // rewrites UVs in metres so one facade tile covers its real-world size
    // and the baked window/floor pattern tiles at true architectural rhythm.
    const metreUVGenerator = {
      generateTopUV(_geometry: THREE.ExtrudeGeometry, vertices: number[], indexA: number, indexB: number, indexC: number, indexD: number) {
        const s = 1 / metresToScene;
        return [
          new THREE.Vector2(vertices[indexA * 3] * s, vertices[indexA * 3 + 2] * s),
          new THREE.Vector2(vertices[indexB * 3] * s, vertices[indexB * 3 + 2] * s),
          new THREE.Vector2(vertices[indexC * 3] * s, vertices[indexC * 3 + 2] * s),
          new THREE.Vector2(vertices[indexD * 3] * s, vertices[indexD * 3 + 2] * s),
        ];
      },
      generateSideWallUV(_geometry: THREE.ExtrudeGeometry, vertices: number[], indexA: number, indexB: number, indexC: number, indexD: number) {
        const s = 1 / metresToScene;
        const ax = vertices[indexA * 3], ay = vertices[indexA * 3 + 1], az = vertices[indexA * 3 + 2];
        const bx = vertices[indexB * 3], by = vertices[indexB * 3 + 1], bz = vertices[indexB * 3 + 2];
        const cx = vertices[indexC * 3], cy = vertices[indexC * 3 + 1], cz = vertices[indexC * 3 + 2];
        const dx = vertices[indexD * 3], dy = vertices[indexD * 3 + 1], dz = vertices[indexD * 3 + 2];
        if (Math.abs(ay - by) < 1e-9) {
          return [
            new THREE.Vector2(ax * s, az * s),
            new THREE.Vector2(bx * s, bz * s),
            new THREE.Vector2(cx * s, cz * s),
            new THREE.Vector2(dx * s, dz * s),
          ];
        }
        return [
          new THREE.Vector2(ax * s, ay * s),
          new THREE.Vector2(bx * s, by * s),
          new THREE.Vector2(cx * s, cy * s),
          new THREE.Vector2(dx * s, dy * s),
        ];
      },
    };
    // Flat gravel-and-parapet roof tone, painted onto horizontal faces via
    // vertex colours so one facade material serves walls and caps alike.
    // ---- 路口工坊立面移植:24 种立面 tile,一格覆盖 3m × 12.8m(四层),
    // 四行窗型各不相同,打破"每层复印感";逐窗明暗 hash 模拟随机反光;
    // 玻璃塔楼低粗糙+金属度靠天空 IBL 出镜面感。
    const FACADE_SPECS: Array<{ base: number[]; window: number[]; cols: number; sill?: number; brick?: boolean; band?: boolean; glass?: boolean }> = [
      { base: [198, 188, 168], window: [96, 104, 112], cols: 2, sill: 0.34 },
      { base: [176, 179, 176], window: [88, 97, 106], cols: 2, sill: 0.32 },
      { base: [216, 211, 197], window: [112, 142, 154], cols: 3, sill: 0.28 },
      { base: [156, 100, 80], window: [64, 56, 54], cols: 2, sill: 0.36, brick: true },
      { base: [208, 196, 168], window: [98, 118, 128], cols: 3, sill: 0.3 },
      { base: [150, 151, 149], window: [80, 92, 100], cols: 4, sill: 0.24, band: true },
      { base: [190, 148, 120], window: [84, 90, 96], cols: 2, sill: 0.32 },
      { base: [132, 136, 142], window: [98, 130, 142], cols: 3, sill: 0.3 },
      { base: [76, 90, 100], window: [134, 170, 186], cols: 4, glass: true },
      { base: [66, 80, 88], window: [120, 160, 152], cols: 5, glass: true },
      { base: [90, 94, 106], window: [150, 152, 160], cols: 4, glass: true },
      { base: [112, 98, 90], window: [128, 150, 158], cols: 3, glass: true },
      { base: [58, 62, 70], window: [110, 150, 168], cols: 6, glass: true },
      { base: [168, 170, 172], window: [96, 118, 130], cols: 5, glass: true },
      { base: [122, 96, 72], window: [150, 138, 116], cols: 4, glass: true },
      { base: [70, 92, 96], window: [128, 164, 170], cols: 5, glass: true },
      { base: [214, 212, 206], window: [104, 116, 126], cols: 4, sill: 0.26 },
      { base: [142, 138, 130], window: [88, 96, 104], cols: 4, sill: 0.26, band: true },
      { base: [186, 172, 148], window: [96, 110, 118], cols: 3, sill: 0.3 },
      { base: [84, 86, 90], window: [118, 128, 136], cols: 5, sill: 0.22 },
      { base: [226, 222, 212], window: [100, 110, 118], cols: 2, sill: 0.4 },
      { base: [164, 132, 96], window: [76, 64, 54], cols: 2, sill: 0.36 },
      { base: [142, 74, 60], window: [70, 58, 52], cols: 2, sill: 0.36, brick: true },
      { base: [172, 178, 164], window: [92, 104, 110], cols: 3, sill: 0.34 },
    ];
    const facadeMaterialCache = new Map<number, THREE.MeshStandardMaterial>();
    const facadeVariants: THREE.MeshStandardMaterial[] = [];
    const facadeMaterial = (variant: number) => {
      const key = variant % FACADE_SPECS.length;
      const cached = facadeMaterialCache.get(key);
      if (cached) return cached;
      const spec = FACADE_SPECS[key];
      const S = 128;
      const data = new Uint8Array(S * S * 4);
      let fstate = key * 131 + 17;
      const rand = () => {
        fstate = (fstate * 1664525 + 1013904223) >>> 0;
        return fstate / 4294967296;
      };
      const cellHash = (i: number, j: number) => {
        const s = Math.sin(i * 127.1 + j * 311.7 + key * 74.7) * 43758.5453;
        return s - Math.floor(s);
      };
      const rowStyle = (row: number) => cellHash(row * 7.3, key * 3.1);
      for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) {
        const v = y / S;
        const u = x / S;
        const row = Math.min(3, Math.floor(v * 4));
        const f = v * 4 - row;
        const col = Math.floor(u * spec.cols);
        const cf = u * spec.cols - col;
        const style = rowStyle(row);
        const wTop = 0.8 + (style - 0.5) * 0.14;
        const wBot = spec.glass ? 0.1 : Math.max(0.26, 0.32 + (spec.sill ?? 0.3) * 0.4 - (style - 0.5) * 0.18);
        const mullion = spec.glass && cf > 0.92;
        let color: number[];
        if (col < spec.cols && cf < (spec.glass ? 0.92 : 0.6) && f > wBot && f < wTop && !mullion) {
          const h = cellHash(col + key * 31, row * 17.3 + Math.floor(key * 5.7));
          const sheen = Math.floor(f * 8) % 4 === 0 ? 12 : 0;
          color = spec.window.map((c) => c * (0.5 + h * 0.85) + sheen + (rand() - 0.5) * 8);
        } else if (f > 0.9 || f < 0.08) {
          color = spec.base.map((c) => c * 0.55 + 8);
        } else if (mullion) {
          color = spec.base.map((c) => c * 0.45);
        } else {
          color = spec.base.map((c) => c + (rand() - 0.5) * 10);
          if (spec.brick && y % 7 < 2) color = color.map((c) => c + 16);
        }
        const index = (y * S + x) * 4;
        for (let c = 0; c < 3; c++) data[index + c] = THREE.MathUtils.clamp(color[c], 0, 255);
        data[index + 3] = 255;
      }
      const texture = new THREE.DataTexture(data, S, S);
      texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
      texture.magFilter = THREE.LinearFilter; texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true; texture.colorSpace = THREE.SRGBColorSpace; texture.needsUpdate = true;
      // 一格 = 3m 宽 × 12.8m 高(四层):UV 按米,repeat 取倒数。
      texture.repeat.set(1 / 3, 1 / 12.8);
      const mat = spec.glass
        ? new THREE.MeshStandardMaterial({ map: texture, roughness: 0.34, metalness: 0.45, vertexColors: true, side: THREE.DoubleSide, envMapIntensity: 0.85 })
        : new THREE.MeshStandardMaterial({ map: texture, roughness: 0.82, vertexColors: true, side: THREE.DoubleSide, envMapIntensity: 0.55 });
      facadeMaterialCache.set(key, mat);
      facadeVariants.push(mat);
      return mat;
    };
    const roofVertexTone = [0.4, 0.41, 0.4];
    // Prefer the rich ModernCity graph when available. `cities` remains the
    // compatibility projection sent by older Rust builds, so selecting one
    // source avoids drawing every building and road twice.
    const renderCities = result.modernCities?.length
      ? result.modernCities.map((city) => ({
          style: "modernChinese" as const,
          streets: [],
          blocks: city.blocks,
          buildings: city.buildings.map((building) => ({
            id: building.id,
            parcelId: building.parcelId,
            footprint: building.footprint,
            courtyard: null,
            height_metres: building.heightMetres,
            floors: building.floors,
            variant: building.variant,
            style: building.style,
            pitched: building.pitched,
            tierRing: building.tierRing,
            podiumFloors: building.podiumFloors,
            facade: building.facade,
            podiumHeightMetres: building.podiumHeightMetres,
            windowBays: building.windowBays,
            balconyBays: building.balconyBays,
            entranceCount: building.entranceCount,
            roof: building.roof,
          })),
          parcels: city.parcels.map((parcel) => ({
            boundary: parcel.ring,
            landUse: parcel.useType,
          })),
          sdRoads: [],
          hdRoads: city.hdRoads.map((road) => ({
            id: road.id,
            class: road.class,
            widthMetres: road.widthMetres,
            pathKm: road.centreline.map((point) => [point.x_km, point.y_km] as [number, number]),
            lanesForward: road.lanesForward,
            lanesBackward: road.lanesBackward,
            medianMetres: road.medianMetres,
            layer: road.layer,
            structure: road.structure,
            lanes: road.lanes,
            connectors: road.connectors,
          })),
          lanes: city.lanes ?? city.hdRoads.flatMap((road) => road.lanes ?? []),
          connectors: city.connectors ?? city.hdRoads.flatMap((road) => road.connectors ?? []),
          junctions: city.junctions ?? [],
          river: city.river ? { centerline: city.river, widthMetres: city.riverWidthMetres ?? 64 } : undefined,
          compounds: city.compounds ?? [],
          trees: city.trees ?? [],
        }))
      : result.cities ?? [];
    // A single merged facade layer supplies window rhythm at neighbourhood
    // scale. It is intentionally capped: the satellite image remains the
    // source of fine texture at regional scale, while a close orbit gets real
    // openings and reflections instead of unbroken colour boxes.
    const windowPositions: number[] = [];
    const windowNormals: number[] = [];
    const windowIndices: number[] = [];
    let windowCount = 0;
    const maxWindowCount = 220_000;
    const addFacadeWindows = (footprint: Array<{ x: number; z: number }>, baseY: number, heightMetres: number) => {
      if (windowCount >= maxWindowCount || footprint.length < 3) return;
      let area = 0;
      for (let index = 0; index < footprint.length; index += 1) {
        const next = footprint[(index + 1) % footprint.length];
        area += footprint[index].x * next.z - next.x * footprint[index].z;
      }
      const ccw = area > 0;
      const floorCount = THREE.MathUtils.clamp(Math.floor(heightMetres / 3.25) - 1, 1, 28);
      for (let index = 0; index < footprint.length && windowCount < maxWindowCount; index += 1) {
        const a = footprint[index];
        const b = footprint[(index + 1) % footprint.length];
        const dx = b.x - a.x;
        const dz = b.z - a.z;
        const lengthScene = Math.hypot(dx, dz);
        const lengthMetres = lengthScene * sceneToMetres;
        if (lengthMetres < 7) continue;
        const ux = dx / Math.max(1.0e-9, lengthScene);
        const uz = dz / Math.max(1.0e-9, lengthScene);
        // Right-hand normal for CCW rings, left-hand for CW rings.
        const nx = ccw ? uz : -uz;
        const nz = ccw ? -ux : ux;
        const bays = THREE.MathUtils.clamp(Math.floor(lengthMetres / 5.4), 1, 30);
        const bayStep = lengthScene / bays;
        const halfWidth = Math.min(0.78, bayStep * 0.31);
        for (let floor = 0; floor < floorCount && windowCount < maxWindowCount; floor += 1) {
          const sill = (floor + 1) * 3.25 + 0.38;
          if (sill + 1.45 > heightMetres) break;
          const y0 = baseY + sill * metresToScene;
          const y1 = y0 + 1.38 * metresToScene;
          for (let bay = 0; bay < bays && windowCount < maxWindowCount; bay += 1) {
            const centreX = a.x + ux * bayStep * (bay + 0.5) + nx * 0.012 * metresToScene;
            const centreZ = a.z + uz * bayStep * (bay + 0.5) + nz * 0.012 * metresToScene;
            const half = halfWidth * metresToScene;
            const p0x = centreX - ux * half;
            const p0z = centreZ - uz * half;
            const p1x = centreX + ux * half;
            const p1z = centreZ + uz * half;
            const base = windowPositions.length / 3;
            // Double-sided material keeps both clockwise and counter-clockwise
            // building rings readable while normals still drive sun response.
            windowPositions.push(
              p0x, y0, p0z,
              p1x, y0, p1z,
              p1x, y1, p1z,
              p0x, y1, p0z,
            );
            for (let vertex = 0; vertex < 4; vertex += 1) windowNormals.push(nx, 0, nz);
            windowIndices.push(base, base + 1, base + 2, base, base + 2, base + 3);
            windowCount += 1;
          }
        }
      }
    };
    // LOD materials are made transparent once and their opacity is driven by
    // view span. This avoids the hard on/off pops that made satellite-to-city
    // transitions look like a texture swap. Depth writes are restored at full
    // opacity so close buildings retain normal occlusion.
    const fadeMaterial = (material: THREE.Material, opacity: number) => {
      const value = THREE.MathUtils.clamp(opacity, 0, 1);
      const candidate = material as THREE.Material & { opacity?: number; transparent?: boolean; depthWrite?: boolean };
      const wasTransparent = candidate.transparent === true;
      candidate.transparent = true;
      if (candidate.opacity !== undefined) candidate.opacity = value;
      if (candidate.depthWrite !== undefined) {
        const wantedDepthWrite = value > 0.985;
        if (candidate.depthWrite !== wantedDepthWrite) {
          candidate.depthWrite = wantedDepthWrite;
          material.needsUpdate = true;
        }
      }
      if (!wasTransparent) material.needsUpdate = true;
    };
    const smoothFadeIn = (spanMetres: number, fullyVisibleBelow: number, fullyHiddenAbove: number) =>
      1 - THREE.MathUtils.smoothstep(spanMetres, fullyVisibleBelow, fullyHiddenAbove);
    const smoothFadeOut = (spanMetres: number, fullyHiddenBelow: number, fullyVisibleAbove: number) =>
      THREE.MathUtils.smoothstep(spanMetres, fullyHiddenBelow, fullyVisibleAbove);
    let buildingCount = 0;
    const FACADE_SETS: Record<string, number[]> = {
      curtain: [8, 9, 10, 11, 12, 13, 14, 15],
      residential: [0, 1, 3, 22, 23],
      stone: [18, 19, 16],
      other: [4, 5, 6, 7, 17, 20],
    };
    for (const city of renderCities) {
      for (const building of city.buildings ?? []) {
        if (buildingCount >= 8_000 || building.footprint.length < 3) break;
        const facadeName = String((building as UrbanBuilding & { facade?: string }).facade ?? "").toLowerCase();
        const set = facadeName.includes("curtain") ? FACADE_SETS.curtain
          : facadeName.includes("brick") ? FACADE_SETS.residential
          : facadeName.includes("stone") ? FACADE_SETS.stone
          : FACADE_SETS.other;
        const variant = set[(Number(building.id ?? 0) * 7 + 3) % set.length];
        const style = facadeBucket(variant);
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
          // three's runtime passes indexD to generateTopUV even though the
          // bundled type omits it; the generator relies on that real call.
          UVGenerator: metreUVGenerator as unknown as THREE.ExtrudeGeometryOptions["UVGenerator"],
        });
        buildingGeometry.rotateX(-Math.PI / 2);
        buildingGeometry.translate(0, terrainHeightAt(centreX, centreZ) + 0.08 * metresToScene, 0);
        buildingGeometry.computeVertexNormals();
        // Extrusion vertices are triangle-local, so painting per-triangle is
        // safe: horizontal faces become flat roofs, walls stay white and let
        // the facade texture show at full strength.
        const buildingPositions = buildingGeometry.attributes.position as THREE.BufferAttribute;
        const buildingColours = new Float32Array(buildingPositions.count * 3).fill(1);
        const buildingUvs = buildingGeometry.attributes.uv.array as Float32Array;
        // 路口工坊移植:逐栋墙面色调(0.82-1.18 三通道微差),同一 tile
        // 的楼群从此读作各自不同的楼。
        const btint = 0.82 + (Number(building.id ?? 0) % 37) / 37 * 0.36;
        const wallTint = [btint * (0.92 + (Number(building.parcelId ?? 1) % 13) / 13 * 0.16), btint * 0.99, btint * (0.86 + (Number(building.id ?? 0) % 7) / 7 * 0.28)];
        const triA = new THREE.Vector3();
        const triB = new THREE.Vector3();
        const triC = new THREE.Vector3();
        for (let vertexIndex = 0; vertexIndex < buildingPositions.count; vertexIndex += 3) {
          triA.fromBufferAttribute(buildingPositions, vertexIndex);
          triB.fromBufferAttribute(buildingPositions, vertexIndex + 1);
          triC.fromBufferAttribute(buildingPositions, vertexIndex + 2);
          triB.sub(triA);
          triC.sub(triA);
          triA.crossVectors(triB, triC);
          if (Math.abs(triA.y) > 0.9 * triA.length()) {
            for (let corner = 0; corner < 3; corner += 1) {
              buildingColours[(vertexIndex + corner) * 3] = roofVertexTone[0];
              buildingColours[(vertexIndex + corner) * 3 + 1] = roofVertexTone[1];
              buildingColours[(vertexIndex + corner) * 3 + 2] = roofVertexTone[2];
              // Pin cap UVs to a flat wall texel so no window ghosting
              // appears on roofs viewed from above.
              buildingUvs[(vertexIndex + corner) * 2] = 0.02;
              buildingUvs[(vertexIndex + corner) * 2 + 1] = 0.06;
            }
          } else {
            for (let corner = 0; corner < 3; corner += 1) {
              buildingColours[(vertexIndex + corner) * 3] = wallTint[0];
              buildingColours[(vertexIndex + corner) * 3 + 1] = wallTint[1];
              buildingColours[(vertexIndex + corner) * 3 + 2] = wallTint[2];
            }
          }
        }
        buildingGeometry.setAttribute("color", new THREE.BufferAttribute(buildingColours, 3));
        buildingGeometry.attributes.uv.needsUpdate = true;
        buildingGeometries.push(buildingGeometry);
        geometriesByStyle.get(style)!.push(buildingGeometry);
        // Baked facade textures already carry the punched-window pattern; the
        // separate dark window quads would only paint every wall black twice.
        if (!useBakedFacades) {
          addFacadeWindows(footprint, terrainHeightAt(centreX, centreZ) + 0.10 * metresToScene, building.height_metres);
        }
        buildingCount += 1;
      }
    }
    for (const [style, parts] of geometriesByStyle) {
      if (parts.length === 0) continue;
      const merged = mergeGeometries(parts, false);
      if (!merged) continue;
      merged.computeBoundingSphere();
      mergedBuildingGeometries.push(merged);
      const variant = Number(style.slice(1));
      const mesh = new THREE.Mesh(merged, facadeMaterial(variant));
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      cityGroup.add(mesh);
    }
    cityGroup.visible = false;
    scene.add(cityGroup);

    const facadeWindowGeometry = new THREE.BufferGeometry();
    if (windowPositions.length > 0) {
      facadeWindowGeometry.setAttribute("position", new THREE.Float32BufferAttribute(windowPositions, 3));
      facadeWindowGeometry.setAttribute("normal", new THREE.Float32BufferAttribute(windowNormals, 3));
      facadeWindowGeometry.setIndex(windowIndices);
      facadeWindowGeometry.computeBoundingSphere();
    }
    const facadeWindowMaterial = new THREE.MeshStandardMaterial({
      color: 0x263f49,
      roughness: 0.34,
      metalness: 0.34,
      emissive: 0x071116,
      emissiveIntensity: 0.08,
      side: THREE.DoubleSide,
      polygonOffset: true,
      polygonOffsetFactor: -1,
      polygonOffsetUnits: -1,
    });
    const facadeWindows = new THREE.Mesh(facadeWindowGeometry, facadeWindowMaterial);
    facadeWindows.visible = false;
    cityGroup.add(facadeWindows);

    // City streets are generated in the Rust model in kilometres.  Preserve
    // that physical scale here and build a small, merged ribbon per class so
    // the city reads as a connected district at macro and neighbourhood LODs.
    // ---- 路口工坊材质移植:像素函数 → 颜色+高度 → 切线空间法线贴图。
    // 所有地面共用世界空间 UV(uv = 米/4),256px 一格对应 4m,骨料约
    // 1.6cm/px,铺装缝 0.5m。高度分量烘焙成法线,骨料/凹缝/草簇在斜阳下
    // 有立体感,不再是"印在地上的图"。
    const citySurfaceMats = new Map<string, THREE.MeshStandardMaterial>();
    const surfaceTextures: THREE.DataTexture[] = [];
    const proceduralMaterial = (
      key: string,
      pixel: (x: number, y: number, rand: () => number) => number[],
      options: { size?: number; roughness?: number; metalness?: number; bump?: number; normalScale?: number } = {},
    ) => {
      const cached = citySurfaceMats.get(key);
      if (cached) return cached;
      let state = key.length * 131 + 7;
      const rand = () => {
        state = (state * 1664525 + 1013904223) >>> 0;
        return state / 4294967296;
      };
      const size = options.size ?? 256;
      const data = new Uint8Array(size * size * 4);
      const height = new Float32Array(size * size);
      let hasHeight = false;
      for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
        const c = pixel(x, y, rand);
        const i = (y * size + x) * 4;
        for (let k = 0; k < 3; k++) data[i + k] = THREE.MathUtils.clamp(c[k], 0, 255);
        data[i + 3] = 255;
        if (c.length > 3) { hasHeight = true; height[y * size + x] = c[3]; }
      }
      const texture = new THREE.DataTexture(data, size, size);
      texture.wrapS = texture.wrapT = THREE.RepeatWrapping;
      texture.magFilter = THREE.LinearFilter; texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true; texture.colorSpace = THREE.SRGBColorSpace; texture.anisotropy = 16;
      texture.needsUpdate = true;
      surfaceTextures.push(texture);
      const mat = new THREE.MeshStandardMaterial({ map: texture, roughness: options.roughness ?? 0.95, metalness: options.metalness ?? 0 });
      if (hasHeight) {
        const bump = options.bump ?? 2.2;
        const nd = new Uint8Array(size * size * 4);
        for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
          const l = height[y * size + (x + size - 1) % size], r = height[y * size + (x + 1) % size];
          const u = height[((y + size - 1) % size) * size + x], d = height[((y + 1) % size) * size + x];
          const nx = (l - r) * bump, ny = (u - d) * bump;
          const len = Math.hypot(nx, ny, 1) || 1, i = (y * size + x) * 4;
          nd[i] = (nx / len * 0.5 + 0.5) * 255; nd[i + 1] = (ny / len * 0.5 + 0.5) * 255;
          nd[i + 2] = (1 / len * 0.5 + 0.5) * 255; nd[i + 3] = 255;
        }
        const normalMap = new THREE.DataTexture(nd, size, size);
        normalMap.wrapS = normalMap.wrapT = THREE.RepeatWrapping;
        normalMap.magFilter = THREE.LinearFilter; normalMap.minFilter = THREE.LinearMipmapLinearFilter;
        normalMap.generateMipmaps = true; normalMap.needsUpdate = true;
        surfaceTextures.push(normalMap);
        mat.normalMap = normalMap;
        mat.normalScale = new THREE.Vector2(options.normalScale ?? 0.6, options.normalScale ?? 0.6);
      }
      citySurfaceMats.set(key, mat);
      return mat;
    };
    // 沥青:低频修补/泛白斑块 + 中频摊铺团块 + 高频骨料 + 亮石子。
    const cityAsphaltMaterial = () => proceduralMaterial('asphalt', (x, y, rand) => {
      const patch = Math.sin(x * 0.043 + Math.sin(y * 0.031) * 2.7) * Math.cos(y * 0.037 + x * 0.017) * 6;
      const clump = (rand() - 0.5) * 22;
      const grain = (rand() - 0.5) * 13;
      const stone = rand() < 0.045 ? 24 : 0;
      const v = 62 + patch + clump * 0.5 + grain + stone;
      return [v * 0.97, v, v * 1.05, 0.5 + (grain + stone) / 56 + (clump < -9 ? -0.14 : 0)];
    }, { roughness: 0.97, bump: 1.7, normalScale: 0.5 });
    // 人行道铺装:0.5×0.25m 错缝砖 + 凹缝 + 逐块色差与污渍。
    const cityPavingMaterial = () => proceduralMaterial('paving', (x, y, rand) => {
      const row = Math.floor(y / 16), off = (row % 2) * 16;
      const gx = (x + off) % 32, gy = y % 16;
      const joint = gx < 2 || gy < 2 ? -30 : 0;
      const bond = (Math.floor((x + off) / 32) + row) % 2 ? 5 : 0;
      const stain = Math.sin((x + row * 11) * 0.021) * Math.cos(y * 0.019) * 7;
      const n = (rand() - 0.5) * 7;
      const v = 152 + joint + bond + stain + n;
      return [v, v * 0.985, v * 0.94, 0.6 + (joint ? -0.42 : 0) + (rand() - 0.5) * 0.08];
    }, { roughness: 0.88, bump: 2.6, normalScale: 0.7 });
    // 草地:低频色斑 + 叶簇 + 枯黄斑,压低饱和度(真草坪从不是纯绿)。
    const cityGrassMaterial = () => proceduralMaterial('grass', (x, y, rand) => {
      const patch = Math.sin(x * 0.09 + y * 0.05) * Math.sin(y * 0.07 + 1.3) * 10;
      const dry = Math.sin(x * 0.017 + 1.1) * Math.cos(y * 0.013) > 0.74;
      const blade = rand() < 0.09 ? 20 : (rand() - 0.5) * 13;
      const col = [76 + patch + blade, 98 + patch + blade, 54 + patch * 0.7 + blade * 0.8];
      if (dry) { col[0] += 32; col[1] += 16; col[2] -= 6; }
      return [...col, 0.5 + blade / 60 + (rand() - 0.5) * 0.3];
    }, { roughness: 0.98, bump: 1.4, normalScale: 0.55 });

    const cityStreetGroup = new THREE.Group();
    const cityStreetGeometries: THREE.BufferGeometry[] = [];
    const citySidewalkGeometries: THREE.BufferGeometry[] = [];
    const citySurfaceTextures: THREE.DataTexture[] = [];
    const makeCitySurfaceTexture = (base: [number, number, number], seed: number) => {
      const size = 96;
      const data = new Uint8Array(size * size * 4);
      for (let y = 0; y < size; y += 1) for (let x = 0; x < size; x += 1) {
        const grain = ((x * 37 + y * 19 + seed * 23) % 31) - 15;
        const chip = (x * 13 + y * 7 + seed) % 97 === 0 ? 28 : 0;
        const index = (y * size + x) * 4;
        data[index] = THREE.MathUtils.clamp(base[0] + grain + chip, 0, 255);
        data[index + 1] = THREE.MathUtils.clamp(base[1] + grain + chip, 0, 255);
        data[index + 2] = THREE.MathUtils.clamp(base[2] + grain + chip, 0, 255);
        data[index + 3] = 255;
      }
      const texture = new THREE.DataTexture(data, size, size, THREE.RGBAFormat);
      texture.wrapS = THREE.RepeatWrapping;
      texture.wrapT = THREE.RepeatWrapping;
      texture.magFilter = THREE.LinearFilter;
      texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true;
      texture.colorSpace = THREE.SRGBColorSpace;
      texture.repeat.set(12, 12);
      texture.needsUpdate = true;
      citySurfaceTextures.push(texture);
      return texture;
    };
    // 路面全部走程序化沥青;人行道走错缝铺装(路口工坊配方)。
    const asphaltTexture = cityAsphaltMaterial().map;
    const avenueTexture = asphaltTexture;
    const streetTexture = asphaltTexture;
    const sidewalkTexture = cityPavingMaterial().map;
    const cityStreetMaterials = {
      boulevard: new THREE.MeshStandardMaterial({ map: asphaltTexture, color: 0x8c8f8d, roughness: 0.90, polygonOffset: true, polygonOffsetFactor: -2, polygonOffsetUnits: -2 }),
      avenue: new THREE.MeshStandardMaterial({ map: avenueTexture, color: 0x90918e, roughness: 0.93, polygonOffset: true, polygonOffsetFactor: -2, polygonOffsetUnits: -2 }),
      street: new THREE.MeshStandardMaterial({ map: streetTexture, color: 0x94948f, roughness: 0.95, polygonOffset: true, polygonOffsetFactor: -2, polygonOffsetUnits: -2 }),
      service: new THREE.MeshStandardMaterial({ map: streetTexture, color: 0x999a94, roughness: 0.98, polygonOffset: true, polygonOffsetFactor: -2, polygonOffsetUnits: -2 }),
    };
    const citySidewalkMaterial = new THREE.MeshStandardMaterial({ map: sidewalkTexture, color: 0xb5b4aa, roughness: 0.98, polygonOffset: true, polygonOffsetFactor: -1, polygonOffsetUnits: -1 });
    const cityStreetParts: Record<keyof typeof cityStreetMaterials, THREE.BufferGeometry[]> = {
      boulevard: [],
      avenue: [],
      street: [],
      service: [],
    };

    const cityStreetRibbon = (from: UrbanPoint, to: UrbanPoint, widthMetres: number, liftMetres: number, elevationMetres = 0) => {
      const x0 = (from.x_km - cityHalfExtentKm) * kilometreToScene;
      const z0 = (from.y_km - cityHalfExtentKm) * kilometreToScene;
      const x1 = (to.x_km - cityHalfExtentKm) * kilometreToScene;
      const z1 = (to.y_km - cityHalfExtentKm) * kilometreToScene;
      const lengthMetres = Math.hypot(x1 - x0, z1 - z0) * sceneToMetres;
      const steps = Math.max(1, Math.ceil(lengthMetres / 90));
      const positions: number[] = [];
      const indices: number[] = [];
      for (let index = 0; index <= steps; index += 1) {
        const t = index / steps;
        const x = THREE.MathUtils.lerp(x0, x1, t);
        const z = THREE.MathUtils.lerp(z0, z1, t);
        const tangentX = x1 - x0;
        const tangentZ = z1 - z0;
        const tangentLength = Math.max(1.0e-9, Math.hypot(tangentX, tangentZ));
        const nx = -tangentZ / tangentLength;
        const nz = tangentX / tangentLength;
        const halfWidth = widthMetres * metresToScene * 0.5;
        const y = terrainHeightAt(x, z) + (liftMetres + elevationMetres) * metresToScene;
        positions.push(x + nx * halfWidth, y, z + nz * halfWidth, x - nx * halfWidth, y, z - nz * halfWidth);
      }
      for (let index = 0; index < steps; index += 1) {
        const vertex = index * 2;
        indices.push(vertex, vertex + 2, vertex + 1, vertex + 1, vertex + 2, vertex + 3);
      }
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
      const uv: number[] = [];
      for (let index = 0; index < positions.length; index += 3) {
        uv.push(positions[index] * sceneToMetres / 4, positions[index + 2] * sceneToMetres / 4);
      }
      geometry.setAttribute("uv", new THREE.Float32BufferAttribute(uv, 2));
      geometry.setIndex(indices);
      geometry.computeVertexNormals();
      return geometry;
    };

    const addCityStreet = (from: UrbanPoint, to: UrbanPoint, roadClass: string | undefined, width: number | undefined, elevationMetres = 0) => {
      if (!from || !to || !Number.isFinite(width) || (width ?? 0) <= 0) return;
      const normalizedClass = roadClass === "expressway" || roadClass === "arterial"
        ? "boulevard"
        : roadClass === "collector" ? "avenue"
        : roadClass === "local" ? "street"
        : roadClass;
      const style = (["boulevard", "avenue", "street", "service"] as string[]).includes(normalizedClass ?? "")
        ? normalizedClass as keyof typeof cityStreetParts
        : "street";
      // The pavement extends below the road to avoid a floating ribbon on
      // sloped terrain; the sidewalk is merged once per city for efficiency.
      const sidewalk = cityStreetRibbon(from, to, (width as number) + 5.0, 0.045, elevationMetres);
      const pavement = cityStreetRibbon(from, to, width as number, 0.075, elevationMetres);
      citySidewalkGeometries.push(sidewalk);
      cityStreetParts[style].push(pavement);
    };
    for (const city of renderCities) {
      for (const street of city.streets ?? []) {
            addCityStreet(street.from, street.to, street.class, street.widthMetres ?? street.width_metres);
      }
      // Modern Chinese payloads expose separate sparse (satellite) and high
      // detail road sets. Consume both so older `streets` payloads and the new
      // Rust model share one physical renderer.
      for (const road of [...(city.sdRoads ?? []), ...(city.hdRoads ?? [])]) {
        const width = ("widthMetres" in road ? road.widthMetres : undefined)
          ?? ("width_metres" in road ? road.width_metres : undefined);
        const path = ("pathKm" in road ? road.pathKm : undefined)
          ?? ("path_km" in road ? road.path_km : undefined);
        if (path && path.length >= 2) {
          for (let index = 0; index + 1 < path.length; index += 1) {
            addCityStreet(
              { x_km: path[index][0], y_km: path[index][1] },
              { x_km: path[index + 1][0], y_km: path[index + 1][1] },
              road.class,
              width,
              Number((road as UrbanRoad).layer ?? 0) > 0 ? 6.0 : 0,
            );
          }
        } else if ("from" in road && "to" in road && road.from && road.to) {
          addCityStreet(road.from, road.to, road.class, width);
        }
      }
    }
    const mergedSidewalks = citySidewalkGeometries.length > 0
      ? mergeGeometries(citySidewalkGeometries, false)
      : null;
    // mergeGeometries copies the attribute buffers. Dispose the per-street
    // staging meshes immediately; retaining hundreds of kilometre ribbons in
    // the heap made repeated terrain regeneration leak GPU memory.
    for (const geometry of citySidewalkGeometries) geometry.dispose();
    if (mergedSidewalks) {
      mergedSidewalks.computeBoundingSphere();
      cityStreetGroup.add(new THREE.Mesh(mergedSidewalks, citySidewalkMaterial));
      cityStreetGeometries.push(mergedSidewalks);
    }
    for (const style of Object.keys(cityStreetParts) as Array<keyof typeof cityStreetMaterials>) {
      const parts = cityStreetParts[style];
      if (parts.length === 0) continue;
      const merged = mergeGeometries(parts, false);
      for (const geometry of parts) geometry.dispose();
      if (!merged) continue;
      merged.computeBoundingSphere();
      cityStreetGroup.add(new THREE.Mesh(merged, cityStreetMaterials[style]));
      cityStreetGeometries.push(merged);
    }
    cityStreetGroup.visible = false;
    scene.add(cityStreetGroup);

    // The source city renderer carries markings as first-class geometry.  Do
    // the same here instead of trying to infer them from a single asphalt
    // ribbon: every lane keeps its physical offset, dashed divider and turn
    // arrow at close zoom.
    const cityDetailGroup = new THREE.Group();
    const cityDetailSourceGeometries: THREE.BufferGeometry[] = [];
    const cityDetailMergedGeometries: THREE.BufferGeometry[] = [];
    const cityDetailParts = new Map<THREE.Material, THREE.BufferGeometry[]>();
    // Every marking competes with the road ribbons in the depth buffer; the
    // road carries polygonOffset -2, so each marking pulls harder (-6) or it
    // silently loses the depth test and disappears under the asphalt.
    const markingOffset = { polygonOffset: true, polygonOffsetFactor: -6, polygonOffsetUnits: -6 };
    const cityDetailMaterials = {
      white: new THREE.MeshBasicMaterial({ color: 0xe9ebe4, transparent: true, opacity: 0.92, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      yellow: new THREE.MeshBasicMaterial({ color: 0xe6c35b, transparent: true, opacity: 0.96, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      median: new THREE.MeshStandardMaterial({ color: 0x7d827a, roughness: 0.9, transparent: true, opacity: 0.95, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      paving: new THREE.MeshStandardMaterial({ color: 0x8f8d80, roughness: 0.96, transparent: true, opacity: 0.85, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      arrow: new THREE.MeshBasicMaterial({ color: 0xdfe8df, transparent: true, opacity: 0.95, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      crosswalk: new THREE.MeshBasicMaterial({ color: 0xf2f0e4, transparent: true, opacity: 0.90, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      connectorLeft: new THREE.MeshBasicMaterial({ color: 0xcfd4cc, transparent: true, opacity: 0.30, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      connectorRight: new THREE.MeshBasicMaterial({ color: 0xcfd4cc, transparent: true, opacity: 0.30, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
      connectorThrough: new THREE.MeshBasicMaterial({ color: 0xcfd4cc, transparent: true, opacity: 0.30, side: THREE.DoubleSide, depthWrite: false, ...markingOffset }),
    };
    const addDetailGeometry = (geometry: THREE.BufferGeometry, material: THREE.Material) => {
      cityDetailSourceGeometries.push(geometry);
      const parts = cityDetailParts.get(material);
      if (parts) parts.push(geometry);
      else cityDetailParts.set(material, [geometry]);
    };
    const scenePath = (path: UrbanPoint[], offsetMetres = 0) => {
      const points = path.map((point) => ({
        ...point,
        x: (point.x_km - cityHalfExtentKm) * kilometreToScene,
        z: (point.y_km - cityHalfExtentKm) * kilometreToScene,
      }));
      return points.map((point, index) => {
        const previous = points[Math.max(0, index - 1)];
        const next = points[Math.min(points.length - 1, index + 1)];
        const tx = next.x - previous.x;
        const tz = next.z - previous.z;
        const length = Math.max(1.0e-9, Math.hypot(tx, tz));
        return {
          x: point.x - tz / length * offsetMetres * metresToScene,
          z: point.z + tx / length * offsetMetres * metresToScene,
        };
      });
    };
    const lineRibbon = (points: Array<{ x: number; z: number }>, widthMetres: number, liftMetres: number) => {
      if (points.length < 2) return null;
      const positions: number[] = [];
      const indices: number[] = [];
      for (let index = 0; index < points.length; index += 1) {
        const previous = points[Math.max(0, index - 1)];
        const next = points[Math.min(points.length - 1, index + 1)];
        const tx = next.x - previous.x;
        const tz = next.z - previous.z;
        const length = Math.max(1.0e-9, Math.hypot(tx, tz));
        const nx = -tz / length;
        const nz = tx / length;
        const half = widthMetres * metresToScene * 0.5;
        const y = terrainHeightAt(points[index].x, points[index].z) + liftMetres * metresToScene;
        positions.push(points[index].x + nx * half, y, points[index].z + nz * half,
          points[index].x - nx * half, y, points[index].z - nz * half);
      }
      for (let index = 0; index + 1 < points.length; index += 1) {
        const vertex = index * 2;
        indices.push(vertex, vertex + 2, vertex + 1, vertex + 1, vertex + 2, vertex + 3);
      }
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
      geometry.setIndex(indices);
      geometry.computeVertexNormals();
      return geometry;
    };
    const addDetailRibbon = (path: UrbanPoint[], offsetMetres: number, widthMetres: number, liftMetres: number, material: THREE.Material) => {
      const geometry = lineRibbon(scenePath(path, offsetMetres), widthMetres, liftMetres);
      if (!geometry) return;
      addDetailGeometry(geometry, material);
    };
    const addDashedDetail = (path: UrbanPoint[], offsetMetres: number, widthMetres: number, dashMetres: number, gapMetres: number, material: THREE.Material, liftMetres = 0.10) => {
      const points = scenePath(path, offsetMetres);
      for (let index = 0; index + 1 < points.length; index += 1) {
        const a = points[index], b = points[index + 1];
        const lengthScene = Math.hypot(b.x - a.x, b.z - a.z);
        const lengthMetres = lengthScene * sceneToMetres;
        for (let cursor = 0; cursor < lengthMetres; cursor += dashMetres + gapMetres) {
          const end = Math.min(lengthMetres, cursor + dashMetres);
          if (end <= cursor + 0.05) continue;
          const t0 = cursor / Math.max(1.0e-9, lengthMetres);
          const t1 = end / Math.max(1.0e-9, lengthMetres);
          const geometry = lineRibbon([
            { x: THREE.MathUtils.lerp(a.x, b.x, t0), z: THREE.MathUtils.lerp(a.z, b.z, t0) },
            { x: THREE.MathUtils.lerp(a.x, b.x, t1), z: THREE.MathUtils.lerp(a.z, b.z, t1) },
          ], widthMetres, liftMetres);
          if (!geometry) continue;
          addDetailGeometry(geometry, material);
        }
      }
    };
    const addArrow = (path: UrbanPoint[], offsetMetres: number, material: THREE.Material, turn = "straight", liftMetres = 0.12) => {
      const points = scenePath(path, offsetMetres);
      if (points.length < 2) return;
      let total = 0;
      for (let index = 0; index + 1 < points.length; index += 1) total += Math.hypot(points[index + 1].x - points[index].x, points[index + 1].z - points[index].z);
      const target = Math.max(2 * metresToScene, total - 16 * metresToScene);
      let travelled = 0;
      let a = points[0], b = points[1];
      for (let index = 0; index + 1 < points.length; index += 1) {
        const segment = Math.hypot(points[index + 1].x - points[index].x, points[index + 1].z - points[index].z);
        if (travelled + segment >= target) { a = points[index]; b = points[index + 1]; break; }
        travelled += segment;
      }
      const tx = b.x - a.x, tz = b.z - a.z, length = Math.max(1.0e-9, Math.hypot(tx, tz));
      const ux = tx / length, uz = tz / length, nx = -uz, nz = ux;
      const centre = { x: a.x + ux * Math.min(8 * metresToScene, length * 0.45), z: a.z + uz * Math.min(8 * metresToScene, length * 0.45) };
      const arrowLength = 4.8 * metresToScene;
      const half = 1.35 * metresToScene;
      const tip = { x: centre.x + ux * arrowLength * 0.55, z: centre.z + uz * arrowLength * 0.55 };
      const tail = { x: centre.x - ux * arrowLength * 0.45, z: centre.z - uz * arrowLength * 0.45 };
      const wing = { x: centre.x - ux * arrowLength * 0.05, z: centre.z - uz * arrowLength * 0.05 };
      const turnOffset = turn === "left" ? -1 : turn === "right" ? 1 : 0;
      const vertices = [tip, { x: wing.x + nx * half, z: wing.z + nz * half }, tail,
        { x: wing.x - nx * half, z: wing.z - nz * half }];
      if (turnOffset !== 0) {
        const bend = turnOffset * 1.2 * metresToScene;
        vertices[0] = { x: vertices[0].x + nx * bend, z: vertices[0].z + nz * bend };
      }
      const y = terrainHeightAt(centre.x, centre.z) + liftMetres * metresToScene;
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(vertices.flatMap((point) => [point.x, y, point.z]), 3));
      geometry.setIndex([0, 1, 2, 0, 2, 3]);
      geometry.computeVertexNormals();
      addDetailGeometry(geometry, material);
    };

    for (const city of renderCities) {
      for (const road of city.hdRoads ?? []) {
        const roadRecord = road as UrbanRoad & { centreline?: UrbanPoint[]; lanes?: UrbanLane[] };
        const path = roadRecord.centreline
          ?? roadRecord.pathKm?.map(([x_km, y_km]) => ({ x_km, y_km }))
          ?? roadRecord.path_km?.map(([x_km, y_km]) => ({ x_km, y_km }))
          ?? [];
        if (path.length < 2) continue;
        const roadLift = Number(roadRecord.layer ?? 0) > 0 ? 6.0 : 0.0;
        const lanes = roadRecord.lanes ?? [];
        const laneCount = Math.max(1, lanes.length ? Math.max(...lanes.map((lane) => Number(lane.indexFromCurb ?? lane.index ?? 0) + 1)) : road.class === "expressway" ? 3 : road.class === "arterial" ? 3 : road.class === "collector" ? 2 : 1);
        const halfRoad = (road.widthMetres ?? 10) * 0.5;
        addDetailRibbon(path, -halfRoad + 0.35, 0.14, 0.11 + roadLift, cityDetailMaterials.white);
        addDetailRibbon(path, halfRoad - 0.35, 0.14, 0.11 + roadLift, cityDetailMaterials.white);
        const laneWidth = (road.widthMetres ?? 10) / Math.max(2, laneCount * 2);
        const dividerOffsets = lanes.length > 0
          ? [-1, 1].flatMap((direction) => {
              const centres = lanes
                .filter((lane) => Number(lane.direction ?? direction) === direction)
                .map((lane) => Math.abs(Number(lane.offsetMetres ?? 0)))
                .filter((offset) => Number.isFinite(offset))
                .sort((a, b) => a - b);
              return centres.slice(0, -1).map((offset, index) => direction * ((offset + centres[index + 1]) * 0.5));
            })
          : [-1, 1].flatMap((direction) => Array.from({ length: Math.max(0, laneCount - 1) }, (_, index) => direction * (index + 1) * laneWidth));
        for (const offset of dividerOffsets) {
          addDashedDetail(path, offset, 0.13, road.class === "expressway" ? 6 : 3, road.class === "expressway" ? 9 : 5, cityDetailMaterials.white, 0.10 + roadLift);
        }
        const medianMetres = Number((roadRecord as UrbanRoad & { medianMetres?: number }).medianMetres ?? 0);
        if (medianMetres > 0.3) {
          // Divided arterial/expressway: a physical median with barrier kerbs
          // instead of a painted centre line.
          addDetailRibbon(path, 0, medianMetres * 0.85, 0.115 + roadLift, cityDetailMaterials.median);
        } else if (road.class === "arterial" || road.class === "expressway") {
          addDetailRibbon(path, 0, 0.16, 0.12 + roadLift, cityDetailMaterials.yellow);
        }
        for (const lane of lanes) {
          const laneOffset = Number(lane.offsetMetres ?? 0);
          const markings = lane.markings ?? [];
          const arrow = markings.find((marking) => String(marking.kind).toLowerCase() === "arrow")?.arrow;
          const lanePath = Number(lane.direction ?? 1) < 0 ? path.slice().reverse() : path;
          if (arrow) addArrow(lanePath, laneOffset, cityDetailMaterials.arrow, String(arrow).toLowerCase().includes("left") ? "left" : String(arrow).toLowerCase().includes("right") ? "right" : "straight", 0.12 + roadLift);
        }
      }
      for (const lane of city.lanes ?? []) {
        const path = lane.path;
        if (path && path.length >= 2) {
          const lanePath = Number(lane.direction ?? 1) < 0 ? path.slice().reverse() : path;
          for (const marking of lane.markings ?? []) {
            if (String(marking.kind).toLowerCase() === "arrow") addArrow(lanePath, Number(lane.offsetMetres ?? 0), cityDetailMaterials.arrow, String(marking.arrow ?? "straight").toLowerCase().includes("left") ? "left" : String(marking.arrow ?? "").toLowerCase().includes("right") ? "right" : "straight");
          }
        }
      }
      for (const connector of [...(city.connectors ?? []), ...(city.junctions ?? []).flatMap((junction) => junction.connectors ?? [])]) {
        const path = connector.path ?? connector.centreline;
        if (!path || path.length < 2) continue;
        const movement = String(connector.movement ?? "through").toLowerCase();
        const material = movement.includes("left") ? cityDetailMaterials.connectorLeft : movement.includes("right") ? cityDetailMaterials.connectorRight : cityDetailMaterials.connectorThrough;
        addDetailRibbon(path, 0, connector.widthMetres ?? 0.22, 0.14, material);
        addArrow(path, 0, material, movement);
      }
      for (const junction of city.junctions ?? []) {
        for (const crosswalk of junction.crosswalks ?? []) {
          if (crosswalk.length < 2) continue;
          const a = crosswalk[0], b = crosswalk[crosswalk.length - 1];
          const dx = b.x_km - a.x_km, dz = b.y_km - a.y_km;
          const lengthKm = Math.hypot(dx, dz);
          if (lengthKm < 0.001) continue;
          const nx = -dz / lengthKm, nz = dx / lengthKm;
          for (let stripe = -3; stripe <= 3; stripe += 1) {
            const shift = stripe * 0.9 / 1000;
            const p0 = { x_km: a.x_km + nx * shift - dx / lengthKm * 2.0 / 1000, y_km: a.y_km + nz * shift - dz / lengthKm * 2.0 / 1000 };
            const p1 = { x_km: a.x_km + nx * shift + dx / lengthKm * 2.0 / 1000, y_km: a.y_km + nz * shift + dz / lengthKm * 2.0 / 1000 };
            addDetailRibbon([p0, p1], 0, 0.45, 0.15, cityDetailMaterials.crosswalk);
          }
        }
      }
    }
    for (const [material, parts] of cityDetailParts) {
      if (parts.length === 0) continue;
      const merged = parts.length === 1 ? parts[0] : mergeGeometries(parts, false);
      if (!merged) {
        for (const geometry of parts) geometry.dispose();
        continue;
      }
      if (merged !== parts[0]) for (const geometry of parts) geometry.dispose();
      merged.computeBoundingSphere();
      cityDetailMergedGeometries.push(merged);
      cityDetailGroup.add(new THREE.Mesh(merged, material));
    }
    cityDetailSourceGeometries.length = 0;
    cityDetailGroup.visible = false;
    scene.add(cityDetailGroup);

    // Compound walls, entrance portals and floor bands are the visual cues
    // that distinguish a Chinese neighbourhood from anonymous extruded
    // parcels.  Keep them in a separate close LOD so the regional terrain is
    // unaffected by thousands of small meshes.
    const cityArchitectureGroup = new THREE.Group();
    // Keep the rich close LOD, but submit one mesh per material instead of a
    // mesh for every wall, floor band and entrance.  The source city can
    // contain tens of thousands of these small parts.
    const cityArchitectureSourceGeometries: THREE.BufferGeometry[] = [];
    const cityArchitectureMergedGeometries: THREE.BufferGeometry[] = [];
    const cityArchitectureParts = new Map<THREE.Material, THREE.BufferGeometry[]>();
    const addArchitectureGeometry = (geometry: THREE.BufferGeometry, material: THREE.Material) => {
      cityArchitectureSourceGeometries.push(geometry);
      const parts = cityArchitectureParts.get(material);
      if (parts) parts.push(geometry);
      else cityArchitectureParts.set(material, [geometry]);
    };
    const cityFacadeMaterials: THREE.MeshStandardMaterial[] = [];
    const cityFacadeMaterialByColor = new Map<number, THREE.MeshStandardMaterial>();
    const citySignalGeometries: THREE.BufferGeometry[] = [];
    const citySignalMaterials: THREE.MeshStandardMaterial[] = [];
    const cityTreeGeometries: THREE.BufferGeometry[] = [];
    const cityTreeMaterials: THREE.MeshStandardMaterial[] = [];
    const compoundWallMaterial = new THREE.MeshStandardMaterial({ color: 0xcfc7b6, roughness: 0.86, transparent: true, opacity: 0.96 });
    const compoundCapMaterial = new THREE.MeshStandardMaterial({ color: 0x77736b, roughness: 0.74, transparent: true, opacity: 0.96 });
    const compoundGateMaterial = new THREE.MeshStandardMaterial({ color: 0x343b42, roughness: 0.46, metalness: 0.12, transparent: true, opacity: 0.98 });
    const balconyMaterial = new THREE.MeshStandardMaterial({ color: 0x646d70, roughness: 0.72, transparent: true, opacity: 0.78 });
    const podiumMaterial = new THREE.MeshStandardMaterial({ color: 0x8f9695, roughness: 0.82, transparent: true, opacity: 0.94 });
    const facadeDetailMaterial = new THREE.MeshStandardMaterial({ color: 0x667d85, roughness: 0.36, metalness: 0.08, transparent: true, opacity: 0.85 });
    const addCityBox = (x: number, y: number, z: number, width: number, height: number, depth: number, material: THREE.Material, angle = 0) => {
      const geometry = new THREE.BoxGeometry(width * metresToScene, height * metresToScene, depth * metresToScene);
      geometry.rotateY(-angle);
      geometry.translate(x, y, z);
      addArchitectureGeometry(geometry, material);
    };
    const addWallSegment = (a: UrbanPoint, b: UrbanPoint, heightMetres: number, material: THREE.Material) => {
      const A = scenePath([a])[0], B = scenePath([b])[0];
      if (!A || !B) return;
      const lengthMetres = Math.hypot(B.x - A.x, B.z - A.z) * sceneToMetres;
      if (lengthMetres < 1.2) return;
      const angle = Math.atan2(B.z - A.z, B.x - A.x);
      const x = (A.x + B.x) * 0.5, z = (A.z + B.z) * 0.5;
      const y = terrainHeightAt(x, z) + heightMetres * metresToScene * 0.5;
      addCityBox(x, y, z, lengthMetres, heightMetres, 0.28, material, angle);
      addCityBox(x, y + heightMetres * metresToScene * 0.53, z, lengthMetres + 0.1, 0.12, 0.44, compoundCapMaterial, angle);
    };
    const facadeColor = (facade: string | undefined) => {
      if (facade?.toLowerCase().includes("curtain")) return 0x6b858e;
      if (facade?.toLowerCase().includes("stone")) return 0xb1a58f;
      if (facade?.toLowerCase().includes("brick")) return 0xa98573;
      return 0x858c8d;
    };
    for (const city of renderCities) {
      for (const compound of city.compounds ?? []) {
        const boundary = compound.boundary ?? compound.wallRing;
        if (!boundary || boundary.length < 3) continue;
        for (let index = 0; index < boundary.length; index += 1) addWallSegment(boundary[index], boundary[(index + 1) % boundary.length], compound.fenceHeightMetres ?? 1.8, compoundWallMaterial);
        const gatePoints = compound.gatePoints ?? [];
        if (gatePoints.length >= 2) {
          const gateA = scenePath([gatePoints[0]])[0], gateB = scenePath([gatePoints[1]])[0];
          const gx = (gateA.x + gateB.x) * 0.5, gz = (gateA.z + gateB.z) * 0.5;
          const angle = Math.atan2(gateB.z - gateA.z, gateB.x - gateA.x);
          const nx = -Math.sin(angle), nz = Math.cos(angle);
          const base = terrainHeightAt(gx, gz);
          addCityBox(gx - nx * 5.5 * metresToScene, base + 2.1 * metresToScene, gz - nz * 5.5 * metresToScene, 0.9, 4.2, 0.9, compoundCapMaterial, angle);
          addCityBox(gx + nx * 5.5 * metresToScene, base + 2.1 * metresToScene, gz + nz * 5.5 * metresToScene, 0.9, 4.2, 0.9, compoundCapMaterial, angle);
          addCityBox(gx, base + 3.4 * metresToScene, gz, 12.4, 1.15, 0.45, compoundGateMaterial, angle);
          addCityBox(gx, base + 1.0 * metresToScene, gz, 0.22, 1.8, 0.18, compoundGateMaterial, angle);
        }
        for (const path of compound.paths ?? []) {
          const geometry = lineRibbon(scenePath(path), (compound.roadWidthMetres ?? 5) + 0.3, 0.12);
          if (!geometry) continue;
          addArchitectureGeometry(geometry, cityDetailMaterials.paving);
        }
      }
      for (const building of city.buildings ?? []) {
        if (building.footprint.length < 3) continue;
        const footprint = scenePath(building.footprint);
        const baseY = terrainHeightAt(footprint.reduce((sum, point) => sum + point.x, 0) / footprint.length, footprint.reduce((sum, point) => sum + point.z, 0) / footprint.length);
        const buildingRecord = building as UrbanBuilding & { heightMetres?: number };
        const buildingHeight = Number(buildingRecord.heightMetres ?? buildingRecord.height_metres ?? 3.2);
        const floors = Number(building.floors ?? Math.max(1, Math.round(buildingHeight / 3.2)));
        const podiumHeight = Number(building.podiumHeightMetres ?? 0);
        if (podiumHeight > 0) {
          const geometry = lineRibbon([...footprint, footprint[0]], 0.24, 0);
          if (geometry) {
            const positions = geometry.attributes.position as THREE.BufferAttribute;
            for (let index = 0; index < positions.count; index += 1) positions.setY(index, baseY + podiumHeight * metresToScene);
            positions.needsUpdate = true;
            addArchitectureGeometry(geometry, podiumMaterial);
          }
        }
        const facade = facadeColor(building.facade);
        let facadeMat = cityFacadeMaterialByColor.get(facade);
        if (!facadeMat) {
          facadeMat = new THREE.MeshStandardMaterial({ color: facade, roughness: 0.56, metalness: building.facade?.toLowerCase().includes("curtain") ? 0.18 : 0.02, transparent: true, opacity: 0.9 });
          cityFacadeMaterialByColor.set(facade, facadeMat);
          cityFacadeMaterials.push(facadeMat);
        }
        const bandEvery = building.balconyBays && building.balconyBays > 0 ? 2 : 4;
        for (let floor = 1; floor < floors; floor += bandEvery) {
          const geometry = lineRibbon([...footprint, footprint[0]], building.balconyBays && building.balconyBays > 0 ? 0.34 : 0.16, 0);
          if (!geometry) continue;
          const positions = geometry.attributes.position as THREE.BufferAttribute;
          const floorY = baseY + floor * 3.2 * metresToScene;
          for (let index = 0; index < positions.count; index += 1) positions.setY(index, floorY);
          positions.needsUpdate = true;
          addArchitectureGeometry(geometry, building.balconyBays && building.balconyBays > 0 ? balconyMaterial : facadeMat);
        }
        // Ground-floor entrance rhythm and curtain-wall mullions prevent the
        // close view from collapsing into a single coloured extrusion.
        const entranceCount = Math.max(1, Math.min(8, Number(building.entranceCount ?? 1)));
        for (let entrance = 0; entrance < entranceCount; entrance += 1) {
          const edge = footprint[entrance % footprint.length];
          const next = footprint[(entrance + 1) % footprint.length];
          const t = (entrance + 0.5) / entranceCount;
          const x = THREE.MathUtils.lerp(edge.x, next.x, t), z = THREE.MathUtils.lerp(edge.z, next.z, t);
          const dx = next.x - edge.x, dz = next.z - edge.z, length = Math.max(1.0e-9, Math.hypot(dx, dz));
          addCityBox(x, baseY + 1.15 * metresToScene, z, 1.4, 2.3, 0.08, facadeDetailMaterial, Math.atan2(dz, dx));
        }
      }
      for (const junction of city.junctions ?? []) {
        for (const head of junction.signalHeads ?? []) {
          const point = head.position;
          if (!point) continue;
          const x = (point.x_km - cityHalfExtentKm) * kilometreToScene;
          const z = (point.y_km - cityHalfExtentKm) * kilometreToScene;
          const base = terrainHeightAt(x, z);
          const height = Number(head.heightMetres ?? 5.8);
          addCityBox(x, base + height * metresToScene * 0.5, z, 0.12, height, 0.12, compoundGateMaterial);
          const aspect = String(head.aspects?.[0] ?? "red").toLowerCase();
          const signalMaterial = new THREE.MeshStandardMaterial({
            color: aspect.includes("green") ? 0x58d27d : aspect.includes("yellow") ? 0xe7c34f : 0xd74d4d,
            emissive: aspect.includes("green") ? 0x163d24 : aspect.includes("yellow") ? 0x493b12 : 0x461313,
            emissiveIntensity: 0.55,
            roughness: 0.30,
            transparent: true,
            opacity: 0.98,
          });
          citySignalMaterials.push(signalMaterial);
          const signalGeometry = new THREE.SphereGeometry(0.22 * metresToScene, 8, 6);
          signalGeometry.translate(x, base + (height - 0.45) * metresToScene, z);
          citySignalGeometries.push(signalGeometry);
          cityArchitectureGroup.add(new THREE.Mesh(signalGeometry, signalMaterial));
        }
      }
    }
    for (const [material, parts] of cityArchitectureParts) {
      if (parts.length === 0) continue;
      const merged = parts.length === 1 ? parts[0] : mergeGeometries(parts, false);
      if (!merged) {
        for (const geometry of parts) geometry.dispose();
        continue;
      }
      if (merged !== parts[0]) for (const geometry of parts) geometry.dispose();
      merged.computeBoundingSphere();
      cityArchitectureMergedGeometries.push(merged);
      cityArchitectureGroup.add(new THREE.Mesh(merged, material));
    }

    // ---- 路口工坊街景配件移植:灌木(低多边形球+flatShading)是绿篱/
    // 树冠/分隔带共用的植物体块原语。
    const barrierMaterial = new THREE.MeshStandardMaterial({ color: 0xb3b4a9, roughness: 0.85 });
    cityTreeMaterials.push(barrierMaterial);
    const BUSH_GREENS = [0x3f7038, 0x4c7f40, 0x37632f, 0x568a45];
    const bushMaterials = new Map<number, THREE.MeshStandardMaterial>();
    const bushMaterial = (color: number) => {
      let material = bushMaterials.get(color);
      if (!material) {
        material = new THREE.MeshStandardMaterial({ color, roughness: 0.95, flatShading: true });
        bushMaterials.set(color, material);
        cityTreeMaterials.push(material);
      }
      return material;
    };
    const bushPartsByMaterial = new Map<THREE.MeshStandardMaterial, THREE.BufferGeometry[]>();
    const addBush = (x: number, y: number, z: number, r: number, squish: number, seedValue: number) => {
      let state = seedValue >>> 0;
      const rand = () => { state = (state * 1664525 + 1013904223) >>> 0; return state / 4294967296; };
      const geometry = new THREE.SphereGeometry(r, 7, 5);
      geometry.scale(1, squish, 1);
      geometry.translate(x, y + r * squish * 0.62, z);
      const material = bushMaterial(BUSH_GREENS[Math.floor(rand() * BUSH_GREENS.length)]);
      const parts = bushPartsByMaterial.get(material);
      if (parts) parts.push(geometry);
      else bushPartsByMaterial.set(material, [geometry]);
    };

    // 观赏草丛:两片交叉的 alpha 裁剪面片,顶点色控制浓淡 —— 街景里最提
    // 气的一层细节。
    const tuftMaterialDef = (() => {
      const size = 64;
      const data = new Uint8Array(size * size * 4);
      let state = 9137;
      const rand = () => { state = (state * 1664525 + 1013904223) >>> 0; return state / 4294967296; };
      const blades = Array.from({ length: 10 }, () => ({
        x: 6 + rand() * (size - 12), w: 2.1 + rand() * 2.5, lean: (rand() - 0.5) * 20, tint: rand(),
      }));
      for (let y = 0; y < size; y++) for (let x = 0; x < size; x++) {
        const v = 1 - y / size;
        let tint = -1;
        for (const b of blades) {
          const cx = b.x + b.lean * (1 - v);
          if (Math.abs(x - cx) < b.w * (0.3 + v * 0.7)) { tint = b.tint; break; }
        }
        const i = (y * size + x) * 4;
        data[i] = 58 + tint * 34;
        data[i + 1] = tint < 0 ? 0 : 124 + tint * 58 - v * 22;
        data[i + 2] = 48 + tint * 26;
        data[i + 3] = tint < 0 ? 0 : 255;
      }
      const texture = new THREE.DataTexture(data, size, size);
      texture.magFilter = THREE.LinearFilter; texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true; texture.colorSpace = THREE.SRGBColorSpace; texture.needsUpdate = true;
      return new THREE.MeshStandardMaterial({ map: texture, alphaTest: 0.5, side: THREE.DoubleSide, roughness: 0.95, vertexColors: true });
    })();
    cityTreeMaterials.push(tuftMaterialDef);
    {
      const tuftPos: number[] = [], tuftUv: number[] = [], tuftColors: number[] = [], tuftIndices: number[] = [];
      let tuftV = 0;
      let tuftBudget = 4200;
      let tuftState = 5151;
      const tuftRand = () => { tuftState = (tuftState * 1664525 + 1013904223) >>> 0; return tuftState / 4294967296; };
      const addTuft = (x: number, z: number, y: number, s: number) => {
        if (tuftBudget-- <= 0) return;
        const shade = 0.75 + tuftRand() * 0.45;
        for (let k = 0; k < 2; k++) {
          const a = tuftRand() * Math.PI + k * Math.PI / 2;
          const hx = Math.cos(a) * s / 2, hz = Math.sin(a) * s / 2;
          tuftPos.push(x - hx, y, z - hz, x + hx, y, z + hz, x + hx, y + s * 0.95, z + hz, x - hx, y + s * 0.95, z - hz);
          tuftUv.push(0, 0, 1, 0, 1, 1, 0, 1);
          for (let c = 0; c < 4; c++) tuftColors.push(shade, shade * (0.95 + tuftRand() * 0.12), shade * 0.88);
          tuftIndices.push(tuftV, tuftV + 1, tuftV + 2, tuftV, tuftV + 2, tuftV + 3);
          tuftV += 4;
        }
      };
      for (const city of result.modernCities ?? []) {
        for (const parcel of city.parcels ?? []) {
          const use = String((parcel as { useType?: string }).useType ?? "").toLowerCase();
          if (!use.includes("park")) continue;
          if (!parcel.ring || parcel.ring.length < 3) continue;
          let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
          for (const p of parcel.ring) {
            minX = Math.min(minX, p.x_km); maxX = Math.max(maxX, p.x_km);
            minY = Math.min(minY, p.y_km); maxY = Math.max(maxY, p.y_km);
          }
          const n = Math.min(24, Math.max(3, Math.round((maxX - minX) * (maxY - minY) * 1e6 * 0.03)));
          for (let k = 0; k < n * 3 && k < 90; k++) {
            const xKm = minX + tuftRand() * (maxX - minX);
            const zKm = minY + tuftRand() * (maxY - minY);
            const x = (xKm - cityHalfExtentKm) * kilometreToScene;
            const z = (zKm - cityHalfExtentKm) * kilometreToScene;
            addTuft(x, z, terrainHeightAt(x, z) + 0.06 * metresToScene, 0.55 * metresToScene * (0.75 + tuftRand() * 0.5) * 12.5);
          }
        }
        // 主干路中央绿化带:草地条 + 灌木列 + 草丛;快速路用混凝土护栏。
        for (const road of city.hdRoads ?? []) {
          const median = Number((road as { medianMetres?: number }).medianMetres ?? 0);
          if (median < 1.5 || road.bridge || Number(road.layer ?? 0) !== 0) continue;
          const path = road.centreline;
          if (!path || path.length < 2) continue;
          const scenePts = path.map((p) => ({
            x: (p.x_km - cityHalfExtentKm) * kilometreToScene,
            z: (p.y_km - cityHalfExtentKm) * kilometreToScene,
          }));
          const isExpressway = road.class === "expressway";
          for (let i = 0; i + 1 < scenePts.length; i++) {
            const a = scenePts[i], b = scenePts[i + 1];
            const segLen = Math.hypot(b.x - a.x, b.z - a.z) * sceneToMetres;
            const steps = Math.max(1, Math.round(segLen / 2.2));
            for (let step = 0; step < steps; step++) {
              const t = (step + 0.5) / steps;
              const x = a.x + (b.x - a.x) * t;
              const z = a.z + (b.z - a.z) * t;
              const y = terrainHeightAt(x, z) + 0.10 * metresToScene;
              const lateral = (tuftRand() - 0.5) * Math.max(0.3, median * 0.42) * metresToScene;
              const nx = -((b.z - a.z) / Math.max(1e-6, Math.hypot(b.x - a.x, b.z - a.z)));
              const nz = (b.x - a.x) / Math.max(1e-6, Math.hypot(b.x - a.x, b.z - a.z));
              if (isExpressway) {
                // 混凝土防撞护栏:0.8m 高的连续条带(分段箱体)。
                const geometry = new THREE.BoxGeometry(segLen / steps * metresToScene * 1.05, 0.8 * metresToScene, 0.35 * metresToScene);
                geometry.rotateY(-Math.atan2(b.z - a.z, b.x - a.x));
                geometry.translate(x, y + 0.4 * metresToScene, z);
                const parts = bushPartsByMaterial.get(barrierMaterial);
                if (parts) parts.push(geometry);
                else bushPartsByMaterial.set(barrierMaterial, [geometry]);
                continue;
              }
              addBush(
                x + nx * lateral, y, z + nz * lateral,
                (0.42 + tuftRand() * 0.4) * metresToScene * 12.5 * 0.09 + 0.42 * metresToScene,
                0.7 + tuftRand() * 0.3,
                (i * 31 + step * 7 + (road.id ?? 0) * 977) >>> 0,
              );
              if (tuftRand() < 0.35) addTuft(x, z, y, (0.4 + tuftRand() * 0.25) * metresToScene * 12.5 * 0.09 + 0.3 * metresToScene);
            }
          }
        }
        // 路灯:主干路/次干路两侧交错,~35m 一杆。杆 + 弯臂 + 灯头。
        const poleParts: THREE.BufferGeometry[] = [];
        const headParts: THREE.BufferGeometry[] = [];
        for (const road of city.hdRoads ?? []) {
          if (Number(road.layer ?? 0) !== 0) continue;
          if (road.class !== "arterial" && road.class !== "collector") continue;
          const path = road.centreline;
          if (!path || path.length < 2) continue;
          const scenePts = path.map((p) => ({
            x: (p.x_km - cityHalfExtentKm) * kilometreToScene,
            z: (p.y_km - cityHalfExtentKm) * kilometreToScene,
          }));
          let acc = 0;
          for (let i = 0; i + 1 < scenePts.length; i++) {
            const a = scenePts[i], b = scenePts[i + 1];
            const segLen = Math.hypot(b.x - a.x, b.z - a.z);
            const dirX = (b.x - a.x) / Math.max(1e-9, segLen);
            const dirZ = (b.z - a.z) / Math.max(1e-9, segLen);
            let cursor = acc;
            while (cursor < segLen) {
              const side = (Math.round((cursor + acc) / (35 * metresToScene)) % 2 === 0) ? 1 : -1;
              const x = a.x + dirX * cursor;
              const z = a.z + dirZ * cursor;
              const yaw = Math.atan2(-(dirZ), dirX);
              const nx = -dirZ * side, nz = dirX * side;
              const px = x + nx * (road.widthMetres * 0.5 + 4.5) * metresToScene;
              const pz = z + nz * (road.widthMetres * 0.5 + 4.5) * metresToScene;
              const py = terrainHeightAt(px, pz);
              const pole = new THREE.CylinderGeometry(0.06 * metresToScene, 0.1 * metresToScene, 7.6 * metresToScene, 6);
              pole.translate(px, py + 3.8 * metresToScene, pz);
              poleParts.push(pole);
              const armLen = 2.4 * metresToScene;
              const arm = new THREE.BoxGeometry(armLen, 0.09 * metresToScene, 0.09 * metresToScene);
              arm.translate(armLen / 2, 0, 0);
              arm.rotateY(yaw);
              arm.translate(px, py + 7.45 * metresToScene, pz);
              poleParts.push(arm);
              const head = new THREE.BoxGeometry(0.3 * metresToScene, 0.13 * metresToScene, 0.8 * metresToScene);
              head.translate(armLen - 0.25 * metresToScene, -0.14 * metresToScene, 0);
              head.rotateY(yaw);
              head.translate(px, py + 7.45 * metresToScene, pz);
              headParts.push(head);
              cursor += 35 * metresToScene;
            }
            acc += segLen;
          }
        }
        const lampPoleMaterial = new THREE.MeshStandardMaterial({ color: 0x4a5560, roughness: 0.6 });
        const lampHeadMaterial = new THREE.MeshStandardMaterial({ color: 0xd8e2e8, roughness: 0.4 });
        cityTreeMaterials.push(lampPoleMaterial, lampHeadMaterial);
        if (poleParts.length) {
          const merged = mergeGeometries(poleParts, false);
          for (const geometry of poleParts) geometry.dispose();
          if (merged) {
            merged.computeBoundingSphere();
            const mesh = new THREE.Mesh(merged, lampPoleMaterial);
            mesh.castShadow = true;
            cityArchitectureGroup.add(mesh);
          }
        }
        if (headParts.length) {
          const merged = mergeGeometries(headParts, false);
          for (const geometry of headParts) geometry.dispose();
          if (merged) {
            merged.computeBoundingSphere();
            cityArchitectureGroup.add(new THREE.Mesh(merged, lampHeadMaterial));
          }
        }
        // 人行道栏杆:主干路两侧,3m 一段,双横杆 + 立柱。
        const railingMaterial = new THREE.MeshStandardMaterial({ color: 0x9aa4a8, roughness: 0.5, metalness: 0.35 });
        cityTreeMaterials.push(railingMaterial);
        const railParts: THREE.BufferGeometry[] = [];
        for (const road of city.hdRoads ?? []) {
          if (Number(road.layer ?? 0) !== 0 || road.class !== "arterial" || road.bridge) continue;
          const path = road.centreline;
          if (!path || path.length < 2) continue;
          const scenePts = path.map((p) => ({
            x: (p.x_km - cityHalfExtentKm) * kilometreToScene,
            z: (p.y_km - cityHalfExtentKm) * kilometreToScene,
          }));
          for (const side of [-1, 1]) {
            for (let i = 0; i + 1 < scenePts.length; i++) {
              const a = scenePts[i], b = scenePts[i + 1];
              const segLen = Math.hypot(b.x - a.x, b.z - a.z) * sceneToMetres;
              const steps = Math.max(1, Math.round(segLen / 3));
              const nx = -((b.z - a.z) / Math.max(1e-9, Math.hypot(b.x - a.x, b.z - a.z)));
              const nz = (b.x - a.x) / Math.max(1e-9, Math.hypot(b.x - a.x, b.z - a.z));
              const offset = side * (road.widthMetres * 0.5 + 3.6) * metresToScene;
              for (let step = 0; step < steps; step++) {
                const t0 = step / steps, t1 = (step + 1) / steps;
                const x0 = a.x + (b.x - a.x) * t0 + nx * offset;
                const z0 = a.z + (b.z - a.z) * t0 + nz * offset;
                const x1 = a.x + (b.x - a.x) * t1 + nx * offset;
                const z1 = a.z + (b.z - a.z) * t1 + nz * offset;
                const y = terrainHeightAt((x0 + x1) / 2, (z0 + z1) / 2);
                const angle = Math.atan2(-(z1 - z0), x1 - x0);
                const len = Math.max(0.05, Math.hypot(x1 - x0, z1 - z0));
                for (const h of [0.52, 1.02]) {
                  const geometry = new THREE.BoxGeometry(len, 0.05 * metresToScene, 0.045 * metresToScene);
                  geometry.rotateY(-angle);
                  geometry.translate((x0 + x1) / 2, y + h * metresToScene, (z0 + z1) / 2);
                  railParts.push(geometry);
                }
                const post = new THREE.CylinderGeometry(0.022 * metresToScene, 0.022 * metresToScene, 1.06 * metresToScene, 5);
                post.translate(x1, y + 0.53 * metresToScene, z1);
                railParts.push(post);
              }
            }
          }
        }
        if (railParts.length) {
          const merged = mergeGeometries(railParts, false);
          for (const geometry of railParts) geometry.dispose();
          if (merged) {
            merged.computeBoundingSphere();
            const mesh = new THREE.Mesh(merged, railingMaterial);
            mesh.castShadow = true;
            cityArchitectureGroup.add(mesh);
          }
        }
      }
      for (const [material, parts] of bushPartsByMaterial) {
        if (parts.length === 0) continue;
        const merged = parts.length === 1 ? parts[0] : mergeGeometries(parts, false);
        if (!merged) continue;
        merged.computeBoundingSphere();
        const mesh = new THREE.Mesh(merged, material);
        cityArchitectureGroup.add(mesh);
      }
      {
        if (tuftV > 0) {
          const geo = new THREE.BufferGeometry();
          geo.setAttribute('position', new THREE.Float32BufferAttribute(tuftPos, 3));
          geo.setAttribute('uv', new THREE.Float32BufferAttribute(tuftUv, 2));
          geo.setAttribute('color', new THREE.Float32BufferAttribute(tuftColors, 3));
          geo.setIndex(tuftIndices);
          geo.computeVertexNormals();
          cityArchitectureGroup.add(new THREE.Mesh(geo, tuftMaterialDef));
        }
      }
    }

    cityArchitectureSourceGeometries.length = 0;
    cityArchitectureGroup.visible = false;
    scene.add(cityArchitectureGroup);

    // ---- 路口工坊树移植:锥形主干 + 手工五棱管枝干,枝梢围绕椭球壳分布
    // 22-32 张 alpha 裁剪叶簇卡(每张贴图约 26 片椭圆叶)。卡片法向向上
    // 偏置,背光叶片不会黑成剪影;顶点色 = 5 种叶色 × 高度阴影(廉价冠内
    // AO)。全城树按材质合并成两个 draw call。
    const LEAF_TONES = [0x5f8348, 0x6f9152, 0x4e7340, 0x7d9a55, 0x8a9d5e].map((c) => new THREE.Color(c));
    let leafCardTextureCache: THREE.DataTexture | null = null;
    const leafCardTexture = () => {
      if (leafCardTextureCache) return leafCardTextureCache;
      const S = 64;
      const data = new Uint8Array(S * S * 4);
      let state = 9931;
      const rand = () => { state = (state * 1664525 + 1013904223) >>> 0; return state / 4294967296; };
      for (let i = 0; i < 26; i++) {
        const cx = 6 + rand() * 52, cy = 6 + rand() * 52, ang = rand() * Math.PI;
        const len = 5.5 + rand() * 5.5, wid = 2.4 + rand() * 2.4;
        const ca = Math.cos(ang), sa = Math.sin(ang), shade = 0.72 + rand() * 0.36;
        for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) {
          const dx = x + 0.5 - cx, dy = y + 0.5 - cy;
          const u = dx * ca + dy * sa, v = -dx * sa + dy * ca;
          if (u * u / (len * len) + v * v / (wid * wid) > 1) continue;
          const idx = (y * S + x) * 4;
          const v255 = 226 * shade;
          data[idx] = v255; data[idx + 1] = v255; data[idx + 2] = v255; data[idx + 3] = 255;
        }
      }
      const texture = new THREE.DataTexture(data, S, S);
      texture.colorSpace = THREE.SRGBColorSpace;
      texture.magFilter = THREE.LinearFilter; texture.minFilter = THREE.LinearMipmapLinearFilter;
      texture.generateMipmaps = true; texture.needsUpdate = true;
      leafCardTextureCache = texture;
      return texture;
    };
    const leafCardMaterial = new THREE.MeshStandardMaterial({
      map: leafCardTexture(), color: 0xffffff, vertexColors: true, roughness: 0.88,
      side: THREE.DoubleSide, alphaTest: 0.45,
    });
    const barkMaterial = new THREE.MeshStandardMaterial({ color: 0x6b5945, roughness: 0.95 });
    cityTreeMaterials.push(leafCardMaterial, barkMaterial);

    const makeTreeGeometries = (xScene: number, yScene: number, zScene: number, size: number, seed: number) => {
      let state = seed >>> 0;
      const rand = () => { state = (state * 1664525 + 1013904223) >>> 0; return state / 4294967296; };
      const trunkH = size * (0.36 + rand() * 0.12);
      const woodPos: number[] = [], woodNorm: number[] = [], woodIdx: number[] = [];
      let wv = 0;
      const addBranch = (x0: number, y0: number, z0: number, bx: number, by: number, bz: number, r0: number, r1: number) => {
        const dx = bx - x0, dy = by - y0, dz = bz - z0;
        const len = Math.hypot(dx, dy, dz) || 0.01;
        const ux = dx / len, uy = dy / len, uz = dz / len;
        let px = -uy, py = ux, pz = 0;
        const pl = Math.hypot(px, py, pz);
        if (pl < 0.05) { px = 1; py = 0; pz = 0; } else { px /= pl; py /= pl; pz /= pl; }
        const qx = uy * pz - uz * py, qy = uz * px - ux * pz, qz = ux * py - uy * px;
        const sides = 5;
        for (let i = 0; i < sides; i++) {
          const a0 = i / sides * Math.PI * 2, a1 = (i + 1) / sides * Math.PI * 2;
          const ring = (t: number, r: number, a: number) => {
            const c = Math.cos(a) * r, s = Math.sin(a) * r;
            return [
              x0 + ux * len * t + px * c + qx * s, y0 + uy * len * t + py * c + qy * s, z0 + uz * len * t + pz * c + qz * s,
              Math.cos(a) * px + Math.sin(a) * qx, Math.cos(a) * py + Math.sin(a) * qy, Math.cos(a) * pz + Math.sin(a) * qz,
            ];
          };
          const v00 = ring(0, r0, a0), v01 = ring(0, r0, a1), v10 = ring(1, r1, a0), v11 = ring(1, r1, a1);
          woodPos.push(v00[0], v00[1], v00[2], v10[0], v10[1], v10[2], v11[0], v11[1], v11[2], v01[0], v01[1], v01[2]);
          woodNorm.push(v00[3], v00[4], v00[5], v10[3], v10[4], v10[5], v11[3], v11[4], v11[5], v01[3], v01[4], v01[5]);
          woodIdx.push(wv, wv + 1, wv + 2, wv, wv + 2, wv + 3);
          wv += 4;
        }
      };
      addBranch(0, 0, 0, 0, trunkH, 0, 0.07 + size * 0.032, 0.05 + size * 0.016);
      const branches = 8 + Math.floor(rand() * 4);
      const leafPos: number[] = [], leafColor: number[] = [], leafIdx: number[] = [], leafUv: number[] = [], leafNorm: number[] = [];
      let v = 0;
      const addCard = (cx: number, cy: number, cz: number, nx: number, ny: number, nz: number, s: number, tone: THREE.Color, shade: number) => {
        let ux = -nz, uy = 0, uz = nx;
        const ul = Math.hypot(ux, uy, uz);
        if (ul < 0.05) { ux = 1; uy = 0; uz = 0; } else { ux /= ul; uy /= ul; uz /= ul; }
        const bx = ny * uz - nz * uy, by = nz * ux - nx * uz, bz = nx * uy - ny * ux;
        const h = s * 0.5;
        // 法向向上偏置:叶面散射 + 透光,背光半冠不发黑。
        const nnx = nx * 0.4, nny = ny * 0.4 + 1.0, nnz = nz * 0.4;
        const nl = Math.hypot(nnx, nny, nnz) || 1;
        const corners: Array<[number, number]> = [[-1, -1], [1, -1], [1, 1], [-1, 1]];
        for (const [su, sv] of corners) {
          leafPos.push(cx + (ux * su + bx * sv) * h, cy + (uy * su + by * sv) * h, cz + (uz * su + bz * sv) * h);
          leafNorm.push(nnx / nl, nny / nl, nnz / nl);
        }
        leafUv.push(0, 0, 1, 0, 1, 1, 0, 1);
        for (let k = 0; k < 4; k++) leafColor.push(tone.r * shade, tone.g * shade, tone.b * shade);
        leafIdx.push(v, v + 1, v + 2, v, v + 2, v + 3); v += 4;
      };
      for (let i = 0; i < branches; i++) {
        const angle = (i + rand() * 0.7) * (Math.PI * 2 / branches);
        const y = trunkH * (0.55 + (i / branches) * 0.6);
        const reach = size * (0.16 + rand() * 0.16);
        const bx = Math.cos(angle) * reach, bz = Math.sin(angle) * reach;
        const by = y + size * (0.18 + rand() * 0.24);
        addBranch(0, y, 0, bx, by, bz, 0.05, 0.02);
        const cr = size * (0.16 + rand() * 0.08);
        const cards = 22 + Math.floor(rand() * 10);
        const tone = LEAF_TONES[Math.floor(rand() * LEAF_TONES.length)];
        for (let c = 0; c < cards; c++) {
          const th = rand() * Math.PI * 2, ph = Math.acos(2 * rand() - 1);
          const rr = cr * (0.35 + rand() * 0.75);
          const px = bx + Math.sin(ph) * Math.cos(th) * rr;
          const py = by + Math.cos(ph) * rr * 0.85;
          const pz = bz + Math.sin(ph) * Math.sin(th) * rr;
          const nth = rand() * Math.PI * 2, nph = Math.acos(2 * rand() - 1);
          const nx = Math.sin(nph) * Math.cos(nth), ny = Math.cos(nph) * 0.6, nz = Math.sin(nph) * Math.sin(nth);
          const s = size * (0.15 + rand() * 0.13);
          const lift = Math.max(0, Math.min(1, (py - 0.4 * size) / (size * 0.9)));
          const shade = (0.74 + rand() * 0.36) * (0.78 + 0.3 * lift);
          addCard(px, py, pz, nx, ny, nz, s, tone, shade);
        }
      }
      const scale = metresToScene;
      const wood = new THREE.BufferGeometry();
      const woodScaled: number[] = [];
      for (const value of woodPos) woodScaled.push(value * scale);
      wood.setAttribute('position', new THREE.Float32BufferAttribute(woodScaled, 3));
      wood.setAttribute('normal', new THREE.Float32BufferAttribute(woodNorm, 3));
      wood.setAttribute('uv', new THREE.Float32BufferAttribute(new Array(wv * 2).fill(0), 2));
      wood.setIndex(woodIdx);
      wood.translate(xScene, yScene, zScene);
      const leaves = new THREE.BufferGeometry();
      const leafScaled: number[] = [];
      for (const value of leafPos) leafScaled.push(value * scale);
      leaves.setAttribute('position', new THREE.Float32BufferAttribute(leafScaled, 3));
      leaves.setAttribute('color', new THREE.Float32BufferAttribute(leafColor, 3));
      leaves.setAttribute('normal', new THREE.Float32BufferAttribute(leafNorm, 3));
      leaves.setAttribute('uv', new THREE.Float32BufferAttribute(leafUv, 2));
      leaves.setIndex(leafIdx);
      leaves.translate(xScene, yScene, zScene);
      return { wood, leaves };
    };

    const cityTrees = renderCities.flatMap((city) => city.trees ?? []);
    {
      // 密度上限:一座城最多 1200 棵(等距抽样),其余由卫星纹理承载。
      const cap = 1200;
      const stride = Math.max(1, Math.ceil(cityTrees.length / cap));
      const woodParts: THREE.BufferGeometry[] = [];
      const leafParts: THREE.BufferGeometry[] = [];
      let treeIndex = 0;
      for (let index = 0; index < cityTrees.length; index += stride) {
        const tree = cityTrees[index];
        const point = tree?.point ?? { x_km: tree?.x_km ?? 0, y_km: tree?.y_km ?? 0 };
        const x = (point.x_km - cityHalfExtentKm) * kilometreToScene;
        const z = (point.y_km - cityHalfExtentKm) * kilometreToScene;
        const y = terrainHeightAt(x, z);
        const size = THREE.MathUtils.clamp(Number(tree?.heightMetres ?? 10), 4, 26);
        const pair = makeTreeGeometries(x, y, z, size, ((tree?.id ?? treeIndex) * 7919 + 17) >>> 0);
        woodParts.push(pair.wood);
        leafParts.push(pair.leaves);
        treeIndex += 1;
      }
      const mergedWood = woodParts.length ? mergeGeometries(woodParts, false) : null;
      for (const geometry of woodParts) geometry.dispose();
      const mergedLeaves = leafParts.length ? mergeGeometries(leafParts, false) : null;
      for (const geometry of leafParts) geometry.dispose();
      if (mergedWood) {
        mergedWood.computeBoundingSphere();
        const mesh = new THREE.Mesh(mergedWood, barkMaterial);
        mesh.castShadow = true;
        cityArchitectureGroup.add(mesh);
        cityTreeGeometries.push(mergedWood);
      }
      if (mergedLeaves) {
        mergedLeaves.computeBoundingSphere();
        const mesh = new THREE.Mesh(mergedLeaves, leafCardMaterial);
        cityArchitectureGroup.add(mesh);
        cityTreeGeometries.push(mergedLeaves);
      }
    }

    // Parcel and river overlays are optional in the legacy payload.  When the
    // Rust city generator supplies them they provide a useful intermediate LOD
    // between the baked terrain texture and individual building masses.
    const cityParcelGroup = new THREE.Group();
    const cityParcelGeometries: THREE.BufferGeometry[] = [];
    const cityParcelMergedGeometries: THREE.BufferGeometry[] = [];
    const cityParcelMaterials = {
      residential: new THREE.MeshStandardMaterial({ color: 0x8f9690, roughness: 1.0, transparent: true, opacity: 0.34, depthWrite: false }),
      commercial: new THREE.MeshStandardMaterial({ color: 0xa69779, roughness: 0.98, transparent: true, opacity: 0.38, depthWrite: false }),
      green: new THREE.MeshStandardMaterial({ color: 0x5e815c, roughness: 1.0, transparent: true, opacity: 0.42, depthWrite: false }),
    };
    const cityParcelParts: Record<keyof typeof cityParcelMaterials, THREE.BufferGeometry[]> = {
      residential: [], commercial: [], green: [],
    };
    const cityRiverGroup = new THREE.Group();
    const cityRiverGeometries: THREE.BufferGeometry[] = [];
    const cityRiverSourceGeometries: THREE.BufferGeometry[] = [];
    const cityRiverMaterial = new THREE.MeshStandardMaterial({ color: 0x2d6e7a, roughness: 0.2, metalness: 0.08, transparent: true, opacity: 0.84, depthWrite: false });
    for (const city of renderCities) {
      for (const parcel of city.parcels ?? []) {
        if (!parcel.boundary || parcel.boundary.length < 3) continue;
        const footprint = parcel.boundary.map((point) => ({
          x: (point.x_km - cityHalfExtentKm) * kilometreToScene,
          z: (point.y_km - cityHalfExtentKm) * kilometreToScene,
        }));
        const shape = new THREE.Shape();
        shape.moveTo(footprint[0].x, -footprint[0].z);
        for (const point of footprint.slice(1)) shape.lineTo(point.x, -point.z);
        shape.closePath();
        const geometry = new THREE.ShapeGeometry(shape);
        geometry.rotateX(-Math.PI / 2);
        const centreX = footprint.reduce((sum, point) => sum + point.x, 0) / footprint.length;
        const centreZ = footprint.reduce((sum, point) => sum + point.z, 0) / footprint.length;
        geometry.translate(0, terrainHeightAt(centreX, centreZ) + 0.032 * metresToScene, 0);
        const use = (("landUse" in parcel ? parcel.landUse : "land_use" in parcel ? parcel.land_use : undefined) ?? "residential").toLowerCase();
        const style = use.includes("park") || use.includes("green") || use.includes("water")
          ? "green" : use.includes("commercial") || use.includes("office") || use.includes("mixed") ? "commercial" : "residential";
        cityParcelParts[style].push(geometry);
        cityParcelGeometries.push(geometry);
      }
      const river = city.river;
      if (river) {
        const tuplePath = ("centerline_km" in river ? river.centerline_km : undefined)
          ?? ("path_km" in river ? river.path_km : undefined);
        const pointPath = ("centerline" in river ? river.centerline : undefined)
          ?? ("path" in river ? river.path : undefined);
        const points: UrbanPoint[] = pointPath ?? tuplePath?.map(([x_km, y_km]) => ({ x_km, y_km })) ?? [];
        const width = ("widthMetres" in river ? river.widthMetres : undefined)
          ?? ("width_metres" in river ? river.width_metres : undefined)
          ?? 24;
        for (let index = 0; index + 1 < points.length; index += 1) {
          const geometry = cityStreetRibbon(points[index], points[index + 1], width, 0.055);
          cityRiverGeometries.push(geometry);
          cityRiverSourceGeometries.push(geometry);
        }
      }
    }
    for (const style of Object.keys(cityParcelParts) as Array<keyof typeof cityParcelMaterials>) {
      const parts = cityParcelParts[style];
      if (parts.length === 0) continue;
      const merged = mergeGeometries(parts, false);
      if (!merged) continue;
      merged.computeBoundingSphere();
      cityParcelGroup.add(new THREE.Mesh(merged, cityParcelMaterials[style]));
      cityParcelMergedGeometries.push(merged);
    }
    const mergedRiver = cityRiverGeometries.length > 0
      ? mergeGeometries(cityRiverGeometries, false)
      : null;
    if (mergedRiver) {
      mergedRiver.computeBoundingSphere();
      cityRiverGroup.add(new THREE.Mesh(mergedRiver, cityRiverMaterial));
      cityRiverGeometries.length = 0;
      cityRiverGeometries.push(mergedRiver);
    }
    // Sources are copied by mergeGeometries; release them before the first
    // frame. The merged buffers are retained for cleanup below.
    for (const source of cityParcelGeometries) source.dispose();
    for (const source of cityRiverSourceGeometries) source.dispose();
    cityParcelGroup.visible = false;
    cityRiverGroup.visible = false;
    scene.add(cityParcelGroup, cityRiverGroup);

    // City ground: a terrain-following paved surface under each settlement.
    // The baked satellite texture runs tens of metres per pixel, so at street
    // zoom the ground between roads is featureless mush — the city carries
    // its own high-frequency paver ground instead, with parcels and grass
    // tinted on top by the overlay above.
    // 城市基底改为程序化草地(真草坪不是纯绿),地块 tint 叠加其上。
    const cityGroundMaterial = cityGrassMaterial();
    for (const city of result.modernCities ?? []) {
      const cityNodes = city.nodes ?? [];
      if (cityNodes.length === 0) continue;
      let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
      for (const node of cityNodes) {
        minX = Math.min(minX, node.point.x_km); maxX = Math.max(maxX, node.point.x_km);
        minY = Math.min(minY, node.point.y_km); maxY = Math.max(maxY, node.point.y_km);
      }
      const marginKm = 0.12;
      minX -= marginKm; maxX += marginKm; minY -= marginKm; maxY += marginKm;
      const divisions = 18;
      const x0 = (minX - cityHalfExtentKm) * kilometreToScene;
      const x1 = (maxX - cityHalfExtentKm) * kilometreToScene;
      const z0 = (minY - cityHalfExtentKm) * kilometreToScene;
      const z1 = (maxY - cityHalfExtentKm) * kilometreToScene;
      const positions: number[] = [];
      const uvs: number[] = [];
      const indices: number[] = [];
      for (let iz = 0; iz <= divisions; iz += 1) {
        for (let ix = 0; ix <= divisions; ix += 1) {
          const x = x0 + (x1 - x0) * ix / divisions;
          const z = z0 + (z1 - z0) * iz / divisions;
          positions.push(x, terrainHeightAt(x, z) + 0.02 * metresToScene, z);
          uvs.push(x / metresToScene / 4, z / metresToScene / 4);
        }
      }
      for (let iz = 0; iz < divisions; iz += 1) {
        for (let ix = 0; ix < divisions; ix += 1) {
          const a = iz * (divisions + 1) + ix;
          indices.push(a, a + divisions + 1, a + 1, a + 1, a + divisions + 1, a + divisions + 2);
        }
      }
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
      geometry.setAttribute("uv", new THREE.Float32BufferAttribute(uvs, 2));
      geometry.setIndex(indices);
      geometry.computeVertexNormals();
      const groundMesh = new THREE.Mesh(geometry, cityGroundMaterial);
      groundMesh.renderOrder = -1;
      cityGroup.add(groundMesh);
    }

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

    // 路口工坊移植:半球底光取冷天蓝/地面暖绿;太阳直射负责阴影,颜色与
    // 强度对齐原版(暖白 1.95),投影参数随视野在 animate 中更新。
    const ambient = new THREE.HemisphereLight(0xcfe2f8, 0x4a5340, 0.4);
    scene.add(ambient);
    const azimuth = THREE.MathUtils.degToRad(config.sunAzimuth);
    const elevation = THREE.MathUtils.degToRad(config.sunElevation);
    const sun = new THREE.DirectionalLight(0xfff1d6, 1.95);
    sun.position.set(Math.sin(azimuth) * Math.cos(elevation) * 5, Math.sin(elevation) * 5, Math.cos(azimuth) * Math.cos(elevation) * 5);
    sun.castShadow = true;
    sun.shadow.mapSize.set(2048, 2048);
    sun.shadow.camera.near = 0.001;
    sun.shadow.camera.far = 12;
    sun.shadow.bias = -0.0003;
    sun.shadow.normalBias = 0.5 * metresToScene;
    scene.add(sun);
    scene.add(sun.target);

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
      if (composer && "setSize" in composer) {
        (composer as unknown as { setSize: (w: number, h: number) => void }).setSize(width, height);
      }
    };
    const observer = new ResizeObserver(resize);
    observer.observe(host);
    resize();

    const clock = new THREE.Clock();
    const cameraDirection = new THREE.Vector3();
    let worldTime = 0;
    let frame = 0;
    let handledFocusNonce = 0;
    let frameCounter = 0;
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
      // City-focus jumps: reposition the orbit target and set the viewing
      // distance so the requested span fills the frame.  Handled before
      // controls.update() so the orbit math adopts the new target at once.
      const focusRequest = cityFocusRef.current;
      if (focusRequest && focusRequest.nonce !== handledFocusNonce) {
        handledFocusNonce = focusRequest.nonce;
        const fx = (focusRequest.xKm - cityHalfExtentKm) * kilometreToScene;
        const fz = (focusRequest.yKm - cityHalfExtentKm) * kilometreToScene;
        const fy = terrainHeightAt(fx, fz);
        controls.target.set(fx, fy + targetClearance, fz);
        if (camera instanceof THREE.PerspectiveCamera) {
          const distance = focusRequest.spanKm * 1000 * metresToScene
            / (2 * Math.tan(THREE.MathUtils.degToRad(camera.fov * 0.5)));
          const direction = camera.position.clone().sub(controls.target);
          if (direction.lengthSq() < 1e-9) direction.set(0.7, 0.55, 0.7);
          camera.position.copy(controls.target).add(direction.setLength(distance));
        } else {
          const ortho = camera as THREE.OrthographicCamera;
          ortho.zoom = 3.4 / Math.max(1e-6, focusRequest.spanKm * 1000 * metresToScene);
          ortho.updateProjectionMatrix();
          camera.position.set(fx, fy + 5.2, fz + 0.001);
        }
      }
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
      } else {
        // OrbitControls permits unrestricted panning by design. Keep the
        // orthographic satellite cursor inside the generated tile so an
        // extreme zoom cannot reveal an empty canvas beside the terrain.
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
      }
      const viewSpanScene = camera instanceof THREE.OrthographicCamera
        ? 3.4 / camera.zoom
        : camera.position.distanceTo(controls.target) * Math.tan(THREE.MathUtils.degToRad(camera.fov * 0.5)) * 2;
      camera.getWorldDirection(cameraDirection);
      const groundFootprintScale = camera instanceof THREE.PerspectiveCamera
        ? Math.min(3, 1 / Math.max(0.34, Math.abs(cameraDirection.y)))
        : 1;
      const viewSpanMetres = viewSpanScene * sceneToMetres;
      // Wheel zoom is multiplicative by nature (constant fraction per tick),
      // which is the only way an 80 km → 1.2 m range stays traversable.  The
      // per-tick fraction eases smoothly with the current framing instead of
      // stepping between hard tiers: brisk at regional range, fine in a
      // neighbourhood, so zooming always feels like one continuous gesture.
      const zoomT = THREE.MathUtils.clamp(
        (Math.log10(Math.max(viewSpanMetres, 1)) - Math.log10(2_000))
          / (Math.log10(20_000) - Math.log10(2_000)),
        0, 1,
      );
      controls.zoomSpeed = 1.0 + zoomT * 2.0;
      // Near and regional vegetation overlap for a short band and cross-fade
      // instead of replacing one another at a single threshold. The overlap
      // keeps a tree stand visually stable while the user continuously zooms.
      const detailTreeFade = smoothFadeIn(viewSpanMetres, 3_200, 7_200);
      const regionalTreeFade = smoothFadeOut(viewSpanMetres, 3_800, 19_000);
      const showDetailTrees = detailTreeFade > 0.001;
      detailCrowns.visible = showDetailTrees;
      detailTrunks.visible = showDetailTrees;
      fadeMaterial(detailCrownMaterial, detailTreeFade);
      fadeMaterial(detailTrunkMaterial, detailTreeFade);
      // Do not draw the sparse regional markers over the detailed trees at
      // full strength: they are deliberately blended in the overlap band.
      trees.visible = regionalTreeFade > 0.001;
      fadeMaterial(treeMaterial, regionalTreeFade);
      if (showDetailTrees) updateDetailForest(viewSpanScene, groundFootprintScale);
      const cropSpanMetres = viewSpanScene / metresToScene * groundFootprintScale;
      const cropRowFade = smoothFadeIn(cropSpanMetres, 380, 2_450);
      const cropPlantFade = smoothFadeIn(cropSpanMetres, 230, 760);
      const showCropRows = cropRowFade > 0.001;
      const showCropPlants = cropPlantFade > 0.001;
      cropRows.visible = showCropRows;
      fadeMaterial(cropRowMaterial, cropRowFade);
      for (const mesh of cropPlantMeshes) {
        mesh.visible = showCropPlants;
      }
      for (const material of [grassMaterial, wheatMaterial, wheatHeadMaterial, cornMaterial, cornLeafMaterial, cornCobMaterial, kernelMaterial]) {
        fadeMaterial(material, cropPlantFade);
      }
      if (showCropRows || showCropPlants) updateCropDetail(viewSpanScene, groundFootprintScale);
      // The baked satellite surface remains the far LOD. Vector ribbons only
      // take over when their real metre widths can occupy useful screen pixels.
      const roadSurfaceFade = smoothFadeIn(viewSpanMetres, 7_500, 18_000);
      const roadMarkingFade = smoothFadeIn(viewSpanMetres, 1_800, 5_500);
      // City masses become visible well before neighbourhood scale so a
      // zoom toward a settlement reveals towers while they still share the
      // frame with the regional terrain.  Streets wait until road widths
      // resolve to a handful of pixels.  Both thresholds are real metres.
      const cityFade = smoothFadeIn(viewSpanMetres, 9_000, 26_000);
      const cityStreetFade = smoothFadeIn(viewSpanMetres, 650, 7_500);
      roadSurfaceGroup.visible = roadSurfaceFade > 0.001;
      roadMarkingGroup.visible = roadMarkingFade > 0.001;
      cityStreetGroup.visible = cityStreetFade > 0.001;
      cityDetailGroup.visible = cityStreetFade > 0.001;
      // The legacy metre-scale city (striped boxes, lane lattice) is retired: cities
      // are drawn by CityViewer from the Rust city-scene. Kept hidden, not deleted, so
      // the terrain view stays untouched.
      cityGroup.visible = false;
      cityArchitectureGroup.visible = cityFade > 0.001;
      facadeWindows.visible = cityFade > 0.001;
      cityParcelGroup.visible = cityFade > 0.001;
      cityRiverGroup.visible = cityFade > 0.001;
      fadeMaterial(asphaltMaterial, roadSurfaceFade);
      fadeMaterial(dirtRoadMaterial, roadSurfaceFade);
      fadeMaterial(whiteMarkingMaterial, roadMarkingFade);
      fadeMaterial(yellowMarkingMaterial, roadMarkingFade);
      for (const material of facadeVariants) fadeMaterial(material, cityFade);
      fadeMaterial(cityGroundMaterial, cityFade);
      fadeMaterial(facadeWindowMaterial, cityFade);
      for (const material of Object.values(cityStreetMaterials)) fadeMaterial(material, cityStreetFade);
      fadeMaterial(citySidewalkMaterial, cityStreetFade);
      for (const material of Object.values(cityDetailMaterials)) fadeMaterial(material, cityStreetFade);
      fadeMaterial(compoundWallMaterial, cityFade);
      fadeMaterial(compoundCapMaterial, cityFade);
      fadeMaterial(compoundGateMaterial, cityFade);
      fadeMaterial(balconyMaterial, cityFade);
      fadeMaterial(podiumMaterial, cityFade);
      fadeMaterial(facadeDetailMaterial, cityFade);
      for (const material of cityFacadeMaterials) fadeMaterial(material, cityFade);
      for (const material of citySignalMaterials) fadeMaterial(material, cityFade);
      for (const material of cityTreeMaterials) fadeMaterial(material, cityFade);
      for (const material of Object.values(cityParcelMaterials)) fadeMaterial(material, cityFade);
      fadeMaterial(cityRiverMaterial, cityFade);
      // 路口工坊移植:天空穹顶跟随相机并推进云漂移;雾距随视野缩放,远景
      // 融进天穹地平线色;太阳阴影相机贴着轨道目标,低频刷新投影。
      skyUniforms.uTime.value = worldTime;
      skyDome.position.copy(camera.position);
      (scene.fog as THREE.Fog).near = Math.max(120 * metresToScene, viewSpanScene * 0.30);
      (scene.fog as THREE.Fog).far = Math.max(600 * metresToScene, viewSpanScene * 1.25);
      const shadowSpan = THREE.MathUtils.clamp(viewSpanScene * 0.85, 40 * metresToScene, 1.5);
      sun.target.position.copy(controls.target);
      sun.position.copy(controls.target).addScaledVector(SUN_DIR, shadowSpan * 2.5 + 0.05);
      sun.shadow.camera.left = -shadowSpan;
      sun.shadow.camera.right = shadowSpan;
      sun.shadow.camera.top = shadowSpan;
      sun.shadow.camera.bottom = -shadowSpan;
      sun.shadow.camera.near = 0.0001;
      sun.shadow.camera.far = shadowSpan * 6 + 0.2;
      sun.shadow.camera.updateProjectionMatrix();
      frameCounter += 1;
      if (frameCounter % 45 === 1) renderer.shadowMap.needsUpdate = true;
      if (composer && cameraMode === "3d") {
        (composer as unknown as { render: () => void }).render();
      } else {
        renderer.render(scene, camera);
      }
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
      for (const material of facadeVariants) material.dispose();
      for (const material of citySurfaceMats.values()) { material.map?.dispose(); material.normalMap?.dispose(); material.dispose(); }
      for (const texture of cityFacadeTextures) texture.dispose();
      facadeWindowGeometry.dispose();
      facadeWindowMaterial.dispose();
      for (const geometry of cityDetailMergedGeometries) geometry.dispose();
      for (const material of Object.values(cityDetailMaterials)) material.dispose();
      for (const geometry of cityArchitectureMergedGeometries) geometry.dispose();
      for (const material of cityFacadeMaterials) material.dispose();
      for (const geometry of citySignalGeometries) geometry.dispose();
      for (const material of citySignalMaterials) material.dispose();
      for (const geometry of cityTreeGeometries) geometry.dispose();
      for (const material of cityTreeMaterials) material.dispose();
      compoundWallMaterial.dispose();
      compoundCapMaterial.dispose();
      compoundGateMaterial.dispose();
      balconyMaterial.dispose();
      podiumMaterial.dispose();
      facadeDetailMaterial.dispose();
      for (const cityStreetGeometry of cityStreetGeometries) cityStreetGeometry.dispose();
      citySidewalkMaterial.dispose();
      for (const material of Object.values(cityStreetMaterials)) material.dispose();
      for (const texture of citySurfaceTextures) texture.dispose();
      for (const cityParcelGeometry of cityParcelMergedGeometries) cityParcelGeometry.dispose();
      for (const material of Object.values(cityParcelMaterials)) material.dispose();
      for (const cityRiverGeometry of cityRiverGeometries) cityRiverGeometry.dispose();
      cityRiverMaterial.dispose();
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
