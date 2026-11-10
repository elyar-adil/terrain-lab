/**
 * One atmosphere for the sky and for the air in front of everything else.
 *
 * The sky dome used to be a hand-painted gradient and distance was a linear fog to
 * one fixed colour. Those two never agree: a ridge fades to a blue-grey the sky
 * behind it does not have, the haze is the same colour toward the sun and away
 * from it, and a mountain forty kilometres off keeps its full contrast until the
 * fog ramp suddenly takes it. The eye reads all of that as "rendered".
 *
 * Here both come from the same few lines of single scattering in an exponential
 * atmosphere (Rayleigh for the air, Mie for the haze):
 *
 * * `atmSource(direction)` is the radiance a long path through lit air converges
 *   to: what the horizon looks like in that direction. Blue-white away from the
 *   sun, warm and bright around it.
 * * The sky is that source times how much air the view ray crosses
 *   (`1 - exp(-tau)`), so it is saturated and pale at the horizon and thins to a
 *   deep blue overhead, and to near black when the camera is in the stratosphere.
 * * Aerial perspective is the same source times `1 - transmittance` of the
 *   segment between the eye and the surface, added to the surface colour times
 *   the transmittance, per channel. Thin air scatters blue first, so far slopes
 *   turn blue before they turn pale, and anything far enough becomes exactly the
 *   sky colour behind it.
 *
 * The fog half replaces three's fog shader chunks, so every lit material gets it
 * without knowing: `scene.fog` must be a `THREE.Fog` whose `near` carries metres
 * per scene unit and whose `far` carries the haze coefficient (`fogParameters`).
 * `FogExp2` scenes (the harness viewer) keep three's own formula.
 */
import * as THREE from "three";

/** Mie (haze) scattering at the ground, per metre, from the UI's 0-10 haze value. */
export function hazeCoefficient(haze: number): number {
  // 0: air with almost nothing in it; 2.5 (default): a clear day, far ranges blue
  // rather than white; 10: a visual range of about 30 km, an ordinary hazy day.
  return 4e-6 * Math.pow(30, THREE.MathUtils.clamp(haze, 0, 10) / 10);
}

const glslFloat = (value: number) => (Number.isInteger(value) ? `${value}.0` : `${value}`);

/** GLSL shared by the dome and the fog. `uniform`-free: the callers pass everything. */
export function atmosphereGlsl(sun: THREE.Vector3): string {
  return /* glsl */ `
const vec3 ATM_SUN = vec3(${glslFloat(sun.x)}, ${glslFloat(sun.y)}, ${glslFloat(sun.z)});
// Rayleigh scattering of air at sea level (1/m) for 680, 550, 440 nm, and the
// scale heights of air and of the haze layer.
const vec3 ATM_BR = vec3(5.8e-6, 13.5e-6, 33.1e-6);
const float ATM_HR = 8000.0;
const float ATM_HM = 1300.0;
// Sun irradiance in scene radiance units, set so a clear horizon sits where a
// photograph exposed for the land puts it.
const float ATM_E = 4.5;

// Relative optical air mass (Kasten and Young): 1 overhead, about 38 at the horizon.
float atmAirmass(float y) {
  float z = degrees(acos(clamp(y, 0.0, 1.0)));
  return 1.0 / (max(y, 0.0) + 0.50572 * pow(96.07995 - z, -1.6364));
}
// Colour of direct sunlight after the air above the ground.
vec3 atmSunTransmittance(float betaM) {
  return exp(-(ATM_BR * ATM_HR + vec3(betaM * ATM_HM)) * atmAirmass(ATM_SUN.y));
}
// What a long lit path converges to in direction d (unit): the horizon's colour.
vec3 atmSource(vec3 d, float betaM) {
  float c = dot(d, ATM_SUN);
  float pR = 0.0597 * (1.0 + c * c);
  // Two lobes: a tight one for the glare around the sun, a broad one for the
  // whitening of the whole sunward half of the sky.
  float g1 = 0.80, g2 = 0.35;
  float pM = 0.0796 * (0.55 * (1.0 - g1 * g1) / pow(1.0 + g1 * g1 - 2.0 * g1 * c, 1.5)
                     + 0.45 * (1.0 - g2 * g2) / pow(1.0 + g2 * g2 - 2.0 * g2 * c, 1.5));
  vec3 tR = ATM_BR * ATM_HR;
  vec3 tM = vec3(betaM * ATM_HM);
  // The sun reaches the scattering air already reddened; multiple scattering puts
  // most of the blue back, so only part of that tint is applied.
  vec3 sun = pow(atmSunTransmittance(betaM), vec3(0.3));
  vec3 single = (tR * pR + tM * pM) / (tR + tM);
  // Light scattered more than once: isotropic, the colour of the open sky.
  vec3 multiple = vec3(0.006, 0.011, 0.019) * (0.6 + 0.4 * ATM_SUN.y);
  return ATM_E * sun * (single + multiple);
}
// Optical depth (Rayleigh rgb, Mie a) of the segment from height h0 (m) along a
// ray climbing dy per metre for dist metres, in an exponential atmosphere.
vec4 atmSegment(float h0, float dy, float dist, float betaM) {
  float h1 = max(h0 + dy * dist, 0.0);
  float sR, sM;
  if (abs(dy) < 1.0e-3) {
    sR = exp(-h0 / ATM_HR) * dist;
    sM = exp(-h0 / ATM_HM) * dist;
  } else {
    sR = ATM_HR * (exp(-h0 / ATM_HR) - exp(-h1 / ATM_HR)) / dy;
    sM = ATM_HM * (exp(-h0 / ATM_HM) - exp(-h1 / ATM_HM)) / dy;
  }
  return vec4(ATM_BR * sR, betaM * sM);
}
// Sky radiance in direction d seen from height h0 (m).
vec3 atmSky(vec3 d, float h0, float betaM) {
  float m = atmAirmass(d.y);
  vec3 up = (ATM_BR * ATM_HR * exp(-h0 / ATM_HR) + vec3(betaM * ATM_HM * exp(-h0 / ATM_HM))) * m;
  // Below the horizon the ray ends on the ground: all the air beneath the eye.
  float slope = max(-d.y, 1.0e-4);
  vec3 down = (ATM_BR * ATM_HR * (1.0 - exp(-h0 / ATM_HR)) + vec3(betaM * ATM_HM * (1.0 - exp(-h0 / ATM_HM)))) / slope
    + (ATM_BR * ATM_HR + vec3(betaM * ATM_HM)) * 38.0 * exp(-h0 / 2500.0);
  // Blend what is seen, not the depths: the depth below the horizon is all but
  // infinite and would hold the blend at "saturated" until its last pixel.
  vec3 filled = mix(1.0 - exp(-down), 1.0 - exp(-up), smoothstep(-0.02, 0.035, d.y));
  return atmSource(d, betaM) * filled;
}
// Surface radiance after the air between it and the eye. rayDir is unit, from the
// eye; dist in metres; h0 the eye's height in metres.
vec3 atmAerial(vec3 colour, vec3 rayDir, float dist, float h0, float betaM, out vec3 airlight, out vec3 transmittance) {
  vec4 seg = atmSegment(max(h0, 0.0), rayDir.y, dist, betaM);
  transmittance = exp(-(seg.rgb + vec3(seg.a)));
  airlight = atmSource(rayDir, betaM);
  // Looking down from high up, the whole Rayleigh column lies over the land as a
  // blue veil. Every high-altitude camera filters most of it out; so does this
  // one, increasingly with height. Level views are untouched, so the far ground
  // still meets the sky at the sky's own colour.
  airlight *= mix(1.0, 0.45, smoothstep(1500.0, 14000.0, h0) * smoothstep(0.0, -0.2, rayDir.y));
  return colour * transmittance + airlight * (1.0 - transmittance);
}
`;
}

let installedFor: string | null = null;

/**
 * Replace three's fog chunks with aerial perspective. Global (shader chunks are),
 * so it is installed once per page for one sun direction; materials compiled
 * afterwards pick it up.
 */
export function installAtmosphere(sun: THREE.Vector3): void {
  const key = sun.toArray().join(",");
  if (installedFor === key) return;
  installedFor = key;
  const chunks = THREE.ShaderChunk as Record<string, string>;
  chunks.fog_pars_vertex = /* glsl */ `
#ifdef USE_FOG
  varying float vFogDepth;
  varying vec3 vFogView;
#endif
`;
  chunks.fog_vertex = /* glsl */ `
#ifdef USE_FOG
  vFogDepth = - mvPosition.z;
  vFogView = mvPosition.xyz;
#endif
`;
  chunks.fog_pars_fragment = /* glsl */ `
#ifdef USE_FOG
  uniform vec3 fogColor;
  varying float vFogDepth;
  varying vec3 vFogView;
  #ifdef FOG_EXP2
    uniform float fogDensity;
  #else
    // near: metres per scene unit. far: haze (Mie) coefficient per metre.
    uniform float fogNear;
    uniform float fogFar;
    ${atmosphereGlsl(sun)}
  #endif
#endif
`;
  chunks.fog_fragment = /* glsl */ `
#ifdef USE_FOG
  #ifdef FOG_EXP2
    float fogFactor = 1.0 - exp( - fogDensity * fogDensity * vFogDepth * vFogDepth );
    gl_FragColor.rgb = mix( gl_FragColor.rgb, fogColor, fogFactor );
  #else
    // The orthographic satellite view looks straight down through the whole
    // column from nowhere in particular: its haze belongs to the image, not here.
    if ( ! isOrthographic ) {
      vec3 fogRay = normalize( ( vec4( vFogView, 0.0 ) * viewMatrix ).xyz );
      vec3 fogAir, fogT;
      atmAerial( vec3( 0.0 ), fogRay, length( vFogView ) * fogNear, cameraPosition.y * fogNear, fogFar, fogAir, fogT );
      // Drawn straight to the screen, the surface is already tone mapped and
      // encoded at this point, so the air has to be too.
      #ifdef TONE_MAPPING
        fogAir = toneMapping( fogAir );
      #endif
      fogAir = linearToOutputTexel( vec4( fogAir, 1.0 ) ).rgb;
      gl_FragColor.rgb = gl_FragColor.rgb * fogT + fogAir * ( 1.0 - fogT );
    }
  #endif
#endif
`;
}

/** The `THREE.Fog` whose two numbers the patched chunks read as parameters. */
export function createAtmosphereFog(sceneToMetres: number, haze: number): THREE.Fog {
  return new THREE.Fog(0xffffff, sceneToMetres, hazeCoefficient(haze));
}

const SKY_VERTEX = /* glsl */ `
  varying vec3 vDir;
  void main() {
    vDir = position;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;

const skyFragment = (sun: THREE.Vector3) => /* glsl */ `
  varying vec3 vDir;
  uniform float uTime;
  uniform float uBetaM;
  uniform float uCameraHeight;
  uniform float uCoverage;
  ${atmosphereGlsl(sun)}

  float hash(vec2 p) { return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453); }
  float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), f.x), f.y);
  }
  float fbm(vec2 p) {
    float v = 0.0, a = 0.5;
    mat2 r = mat2(0.80, -0.60, 0.60, 0.80);
    for (int i = 0; i < 6; i++) { v += a * vnoise(p); p = r * p * 2.07 + vec2(19.7, 7.3); a *= 0.5; }
    return v;
  }
  // Cloud density over the layer's plane (unit: about 1.5 km).
  float cloudField(vec2 uv, vec2 drift) {
    float warp = fbm(uv * 0.18 + drift);
    float broad = fbm(uv * 0.30 + (warp - 0.5) * 2.2 + drift * 1.7);
    float detail = fbm(uv * 1.9 + warp * 3.0 - drift * 2.3);
    return broad * 0.80 + detail * 0.20;
  }

  void main() {
    vec3 d = normalize(vDir);
    float y = d.y;
    vec3 col = atmSky(d, uCameraHeight, uBetaM);

    // The disc: bright enough to bloom, coloured by the air it came through.
    float s = max(dot(d, ATM_SUN), 0.0);
    vec3 sunColour = atmSunTransmittance(uBetaM);
    col += sunColour * (smoothstep(0.99989, 0.99996, s) * 90.0 + pow(s, 1800.0) * 3.0) * step(0.0, y);

    // Cumulus on a plane overhead, seen from below. From the stratosphere the
    // layer is underneath, where the terrain view draws its own.
    float below = 1.0 - smoothstep(2500.0, 7000.0, uCameraHeight);
    if (y > 0.004 && below > 0.0) {
      vec2 uv = d.xz / (y + 0.10) * 3.2;
      vec2 drift = vec2(uTime * 0.0022, uTime * 0.0009);
      float f = cloudField(uv, drift);
      float edge = mix(0.72, 0.40, uCoverage);
      float cover = smoothstep(edge, edge + 0.20, f);
      if (cover > 0.001) {
        // Light through the cloud: sample the field a step toward the sun; a
        // thick neighbour there puts this spot in shade (grey base, lit rim).
        vec2 toSun = normalize(ATM_SUN.xz - d.xz * (ATM_SUN.y / max(y, 0.05)) + 1.0e-4);
        float toward = cloudField(uv + toSun * 0.55, drift);
        float thickness = smoothstep(edge, edge + 0.34, f);
        float shade = clamp(1.0 - 1.15 * smoothstep(edge - 0.05, edge + 0.30, toward) * thickness, 0.0, 1.0);
        vec3 ambient = atmSky(vec3(0.0, 1.0, 0.0), uCameraHeight, uBetaM) * 1.35 + atmSource(vec3(d.x, 0.0, d.z), uBetaM) * 0.30;
        vec3 lit = sunColour * (0.95 + 1.6 * pow(s, 8.0) * (1.0 - thickness));
        vec3 cloud = ambient * (0.55 + 0.45 * (1.0 - thickness)) + lit * (0.16 + 0.84 * shade);
        // The layer is 2 km up: toward the horizon it is tens of kilometres of
        // air away and goes the way of everything else that far.
        float dist = 2000.0 / (y + 0.02);
        vec3 airlight, trans;
        cloud = atmAerial(cloud, d, dist, uCameraHeight, uBetaM, airlight, trans);
        col = mix(col, cloud, cover * 0.96 * below * smoothstep(0.004, 0.05, y));
      }
    }

    gl_FragColor = vec4(col, 1.0);
    #include <tonemapping_fragment>
    #include <colorspace_fragment>
  }
`;

export interface AtmosphereSky {
  material: THREE.ShaderMaterial;
  uniforms: {
    uTime: { value: number };
    uBetaM: { value: number };
    uCameraHeight: { value: number };
    uCoverage: { value: number };
  };
}

export function createAtmosphereSky(sun: THREE.Vector3, haze: number, coverage: number): AtmosphereSky {
  const uniforms = {
    uTime: { value: 0 },
    uBetaM: { value: hazeCoefficient(haze) },
    uCameraHeight: { value: 2 },
    uCoverage: { value: coverage },
  };
  const material = new THREE.ShaderMaterial({
    vertexShader: SKY_VERTEX,
    fragmentShader: skyFragment(sun),
    uniforms,
    side: THREE.BackSide,
    depthWrite: false,
    depthTest: false,
    fog: false,
  });
  return { material, uniforms };
}
