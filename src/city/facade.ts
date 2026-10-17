/**
 * The procedural facade (`cnfacade`): every Chinese-style wall in the city,
 * drawn from the elevation table the Rust grammar writes
 * (`crates/city-scene/src/facades/cn_grammar.rs`).
 *
 * Nothing here is a texture lookup of the facade itself.  Each wall vertex
 * carries its elevation's table row in its colour alpha and its position as UV
 * (metres along the wall, storeys up from the elevation's base).  The fragment
 * finds the span under it (window, bay, balcony door, loggia, AC louvre,
 * curtain wall, pier...) and draws frame, sill, glass, skin and weathering
 * analytically, so a facade is sharp at two metres and does not shimmer at a
 * kilometre: every edge is a box filtered by the pixel's own footprint, and
 * every pattern finer than a pixel fades to its mean.
 *
 * Glass is drawn in three levels of detail:
 *
 * - **near** (a pixel under ~3 cm of wall): interior mapping.  A ray from the
 *   eye is traced through the pane into a room box behind it — back wall, side
 *   walls, floor, ceiling and one piece of furniture — lit by daylight that
 *   falls off away from the window, with a ceiling lamp on in some rooms, and
 *   curtains hung just inside the glass;
 * - **middle**: the curtains and blinds as a flat layer over the room's mean;
 * - **far**: the colour a window averages to.
 *
 * The room is emitted light, not a lit surface (the sun does not light the
 * back wall of a room through the facade), and the glass on top of it is a
 * real dielectric with a per-pane tilt, so the sky's reflection differs from
 * pane to pane as it does on any real tower.
 */

import * as THREE from "three";

const PARS = /* glsl */ `
uniform sampler2D uFacadeTable;
uniform float uInteriorLight;
uniform float uInteriorMap;
varying vec3 vFacadeWorld;

vec4 fTex(int x, int row) { return texelFetch(uFacadeTable, ivec2(x, row), 0); }
float fU16(vec2 hl) { return (floor(hl.x * 255.0 + 0.5) * 256.0 + floor(hl.y * 255.0 + 0.5)) / 1000.0; }
int fByte(float v) { return int(v * 255.0 + 0.5); }

float fHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
float fHash2(vec2 p) { return fHash(vec3(p, 7.13)); }
float fNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(fHash2(i), fHash2(i + vec2(1.0, 0.0)), f.x),
             mix(fHash2(i + vec2(0.0, 1.0)), fHash2(i + vec2(1.0, 1.0)), f.x), f.y);
}

// 1 inside [a, b], filtered over a pixel of width w.
float fBox(float x, float a, float b, float w) {
  return clamp((x - a) / w + 0.5, 0.0, 1.0) * clamp((b - x) / w + 0.5, 0.0, 1.0);
}
// Thin lines of half-width hw every 'period', filtered; fades to the lines'
// mean coverage once they are finer than a pixel.
float fLines(float x, float period, float hw, float w) {
  float d = abs(fract(x / period + 0.5) - 0.5) * period;
  float line = 1.0 - smoothstep(hw - w * 0.5, hw + w * 0.5, d);
  float mean = min(1.0, 2.0 * hw / period);
  return mix(line, mean, smoothstep(hw, hw * 4.0, w));
}

// A room behind a pane, traced from the glass.  Returns radiance.
// p: entry point (x along the room from its left, y above its floor),
// rd: ray in room space (x along, y up, z into the room), size: (w, h, d).
vec3 fRoom(vec2 p, vec3 rd, vec3 size, float seed, float office) {
  rd.z = max(rd.z, 1e-3);
  if (abs(rd.x) < 1e-4) rd.x = 1e-4;
  if (abs(rd.y) < 1e-4) rd.y = 1e-4;
  float tx = rd.x > 0.0 ? (size.x - p.x) / max(rd.x, 1e-4) : -p.x / min(rd.x, -1e-4);
  float ty = rd.y > 0.0 ? (size.y - p.y) / max(rd.y, 1e-4) : -p.y / min(rd.y, -1e-4);
  float tz = size.z / rd.z;
  float t = min(min(tx, ty), tz);
  vec3 hit = vec3(p, 0.0) + rd * t;

  float h1 = fHash(vec3(seed, 1.0, 3.0));
  float h2 = fHash(vec3(seed, 5.0, 7.0));
  float h3 = fHash(vec3(seed, 11.0, 13.0));
  vec3 wallC = h1 < 0.45 ? vec3(0.72, 0.70, 0.66)
    : h1 < 0.65 ? vec3(0.70, 0.64, 0.52)
    : h1 < 0.80 ? vec3(0.58, 0.64, 0.66)
    : h1 < 0.92 ? vec3(0.66, 0.66, 0.60)
    : vec3(0.52, 0.40, 0.30);
  vec3 floorC = h2 < 0.55 ? vec3(0.34, 0.22, 0.13) : h2 < 0.8 ? vec3(0.60, 0.58, 0.54) : vec3(0.42, 0.40, 0.38);
  vec3 c;
  if (t == tz) {
    c = wallC;
    // A door or a wardrobe on the back wall, and a picture.
    float door = fBox(hit.x, size.x * (0.15 + 0.5 * h3), size.x * (0.15 + 0.5 * h3) + 0.9, 0.02) * step(hit.y, 2.05);
    c = mix(c, office > 0.5 ? vec3(0.30) : vec3(0.40, 0.28, 0.18), door);
  } else if (t == ty) {
    if (rd.y < 0.0) {
      c = floorC * (0.9 + 0.1 * step(0.5, fract(hit.z / 0.15)));
    } else {
      c = vec3(0.80, 0.80, 0.78);
      if (office > 0.5) {
        // Ceiling tiles with light panels.
        c *= 0.85 + 0.15 * step(0.04, fract(hit.x / 0.6)) * step(0.04, fract(hit.z / 0.6));
      }
    }
  } else {
    c = wallC * 0.9;
  }
  // Furniture against the back wall: a sofa, a bed, a desk row, a cabinet.
  vec3 bmin = vec3(size.x * (0.1 + 0.2 * h2), 0.0, size.z - (office > 0.5 ? 1.4 : 0.9));
  vec3 bmax = vec3(bmin.x + size.x * (0.35 + 0.3 * h3), office > 0.5 ? 0.75 : 0.85 + 0.9 * step(0.7, h1), size.z);
  vec3 o = vec3(p, 0.0);
  vec3 inv = 1.0 / rd;
  vec3 t0 = (bmin - o) * inv;
  vec3 t1 = (bmax - o) * inv;
  vec3 tn = min(t0, t1);
  vec3 tf = max(t0, t1);
  float enter = max(max(tn.x, tn.y), tn.z);
  float leave = min(min(tf.x, tf.y), tf.z);
  if (enter > 0.0 && enter < leave && enter < t) {
    t = enter;
    hit = o + rd * t;
    vec3 cloth = vec3(fHash(vec3(seed, 17.0, 1.0)), fHash(vec3(seed, 19.0, 2.0)), fHash(vec3(seed, 23.0, 3.0)));
    c = office > 0.5 ? vec3(0.35, 0.34, 0.33) : mix(vec3(0.30), cloth, 0.5);
    c *= tn.y == enter ? 1.0 : 0.75;
  }
  // Daylight from the window falls off into the room; a lamp is on in some.
  float daylight = uInteriorLight * mix(1.0, 0.35, clamp(hit.z / size.z, 0.0, 1.0));
  float lamp = step(office > 0.5 ? 0.3 : 0.8, fHash(vec3(seed, 29.0, 31.0)));
  vec3 light = vec3(daylight) + lamp * uInteriorLight * vec3(1.1, 0.95, 0.75)
    * smoothstep(size.y * 1.6, 0.0, length(hit - vec3(size.x * 0.5, size.y, size.z * 0.5)));
  vec3 radiance = c * light;
  // The lamp itself.
  if (t == ty && rd.y > 0.0) {
    float disc = 1.0 - smoothstep(0.18, 0.22, length(hit.xz - vec2(size.x * 0.5, size.z * 0.5)));
    radiance += lamp * disc * vec3(3.0, 2.7, 2.2);
  }
  return radiance;
}
`;

const VERTEX_PARS = /* glsl */ `
varying vec3 vFacadeWorld;
`;

const VERTEX_MAIN = /* glsl */ `
#include <worldpos_vertex>
vFacadeWorld = (modelMatrix * vec4(transformed, 1.0)).xyz;
`;

/**
 * Replaces `color_fragment`: decides what the fragment is and sets the
 * diffuse colour, and leaves the glass coverage, the interior radiance, the
 * roughness and the pane tilt for the later chunks.
 */
const COLOR_CHUNK = /* glsl */ `
float fGlass = 0.0;
float fRough = 0.9;
vec3 fEmit = vec3(0.0);
vec3 fF0 = vec3(0.04);
vec2 fTilt = vec2(0.0);
{
  int row = fByte(vColor.a);
  vec3 tint = vColor.rgb;
  vec4 h0 = fTex(0, row);
  vec4 h1 = fTex(1, row);
  vec4 h2 = fTex(2, row);
  vec4 h3 = fTex(3, row);
  vec4 h4 = fTex(4, row);
  float W = max(fU16(h0.rg), 0.5);
  float H = max(h0.b * 255.0 / 50.0, 2.0);
  int n = fByte(h0.a);
  int skin = fByte(h1.r);
  float tileModule = h1.g * 255.0 / 100.0;
  int glassKind = fByte(h1.b);
  float grilles = h1.a * 255.0;
  vec3 frameC = h2.rgb;
  vec3 trimC = h3.rgb;
  int flags = fByte(h3.a);
  vec3 coatC = h4.rgb;
  float weather = h4.a;

  float U = vUv.x;
  float V = vUv.y;
  float k = floor(U / W);
  bool odd = mod(k, 2.0) > 0.5;
  float x = U - k * W;
  if (odd) x = W - x;
  float s = floor(V);
  float ml = (V - s) * H;
  // Metres of wall per pixel, across and up.
  float fw = max(fwidth(U), 1e-4);
  float fh = max(fwidth(V) * H, 1e-4);

  // The span under the fragment.
  int kind = 0;
  float x0 = 0.0;
  float x1 = W;
  float p0 = 0.0;
  float p1 = 0.0;
  int spanBits = 0;
  int spanIndex = 0;
  for (int i = 0; i < 20; i++) {
    if (i >= n) break;
    vec4 a = fTex(8 + 2 * i, row);
    float sx1 = fU16(a.ba);
    if (x < sx1 || i == n - 1) {
      vec4 b = fTex(9 + 2 * i, row);
      x0 = fU16(a.rg);
      x1 = sx1;
      kind = fByte(b.r);
      p0 = b.g * 255.0 / 100.0;
      p1 = b.b * 255.0 / 50.0;
      spanBits = fByte(b.a);
      spanIndex = i;
      break;
    }
  }
  int panes = spanBits - (spanBits / 16) * 16;
  int style = spanBits / 16;

  // --- the skin -------------------------------------------------------------
  vec2 wp = vec2(U, V * H);
  float blotch = fNoise(vFacadeWorld.xz * 0.31 + vFacadeWorld.y * 0.27) - 0.5;
  float fine = fNoise(wp * 2.7) - 0.5;
  float streak = max(0.0, fNoise(vec2(U * 9.0, V * H * 0.35)) - 0.45) / 0.55;
  float paint = 0.86 * (1.0 + (0.035 + 0.05 * weather) * blotch + 0.02 * fine);
  paint *= 1.0 - (0.04 + 0.10 * weather) * streak * smoothstep(0.0, 0.9, ml / H + 0.2);
  vec3 skinC = tint * paint;
  float skinRough = 0.9;
  if (skin == 1) {
    // 面砖: small glazed tiles, a joint every module up and every 0.24 m across.
    float jointH = fLines(wp.y, max(tileModule, 0.04), 0.004, fh);
    float jointV = fLines(wp.x + 0.12 * mod(floor(wp.y / max(tileModule, 0.04)), 2.0), 0.24, 0.004, fw);
    float joint = max(jointH, jointV);
    vec2 cell = floor(vec2(wp.x / 0.24, wp.y / max(tileModule, 0.04)));
    float tile = 1.0 + 0.05 * (fHash2(cell) - 0.5) * smoothstep(0.06, 0.02, max(fw, fh));
    skinC = mix(tint * paint * tile, vec3(0.55, 0.54, 0.52) * paint, joint * 0.6);
    skinRough = 0.5;
  } else if (skin == 2) {
    float m = max(tileModule, 0.3);
    float joint = max(fLines(wp.y, m, 0.005, fh), fLines(wp.x, m * 2.0, 0.005, fw));
    vec2 cell = floor(vec2(wp.x / (m * 2.0), wp.y / m));
    skinC = tint * paint * (1.0 + 0.06 * (fHash2(cell) - 0.5)) * (1.0 - 0.25 * joint);
    skinRough = 0.7;
  } else if (skin == 3) {
    // Running-bond brick in a pale mortar.
    float course = floor(wp.y / 0.0625);
    float bx = wp.x + 0.12 * mod(course, 2.0);
    float mortar = max(fLines(wp.y, 0.0625, 0.005, fh), fLines(bx, 0.24, 0.005, fw));
    float brick = 1.0 + 0.16 * (fHash2(vec2(floor(bx / 0.24), course)) - 0.5) * smoothstep(0.05, 0.015, max(fw, fh));
    skinC = mix(tint * paint * brick, vec3(0.50, 0.48, 0.45), mortar * 0.7);
  } else if (skin == 4) {
    float joint = max(fLines(wp.y, 1.2, 0.006, fh), fLines(wp.x, 1.2, 0.006, fw));
    skinC = tint * paint * (1.0 - 0.3 * joint);
    skinRough = 0.45;
  } else if (skin == 5) {
    skinC = tint * 0.86 * (1.0 + 0.12 * blotch + 0.05 * fine)
      * (1.0 - (0.08 + 0.12 * weather) * streak);
  }
  // A band at every slab.
  if ((flags / 2) - (flags / 4) * 2 == 1) {
    float band = fBox(ml, -0.05, 0.22, fh) + fBox(ml, H - 0.02, H + 0.05, fh);
    skinC = mix(skinC, trimC * (1.0 - 0.3 * fBox(ml, -0.05, 0.02, fh)), clamp(band, 0.0, 1.0));
  }
  // Grime under the slab line, a soft shadow the projecting slab edge leaves.
  skinC *= 1.0 - 0.10 * fBox(ml, H - 0.12, H + 0.01, fh) * (0.5 + weather);

  vec3 colour = skinC;
  float rough = skinRough;

  // --- the openings --------------------------------------------------------
  // Opening rectangle, in the span's own x and in metres above the floor.
  float ox0 = x0;
  float ox1 = x1;
  float oy0 = 0.0;
  float oy1 = 0.0;
  float opening = 0.0;
  int sash = style;
  if (kind == 2) {
    oy0 = p0; oy1 = p1; opening = 1.0;
  } else if (kind == 3) {
    ox0 += 0.06; ox1 -= 0.06; oy0 = p0; oy1 = p1; opening = 1.0; panes = 3;
  } else if (kind == 4 || kind == 10) {
    ox0 += 0.3; ox1 -= 0.3; oy0 = 0.02; oy1 = p1; opening = 1.0;
    panes = int(clamp(floor((ox1 - ox0) / 0.9 + 0.5), 2.0, 6.0));
    sash = 0;
  } else if (kind == 6) {
    oy0 = p0; oy1 = p1; opening = 1.0; panes = 1; sash = 1;
  }
  bool surround = (flags - (flags / 2) * 2) == 1 && kind == 2;
  if (surround) { ox0 += 0.14; ox1 -= 0.14; }

  float glassCov = 0.0;
  float gu = 0.0;
  float gv = 0.0;
  float ow = max(ox1 - ox0, 0.1);
  float oh = max(oy1 - oy0, 0.1);
  if (kind == 1) {
    colour = trimC * (1.0 - 0.25 * (fBox(x, x0 - 0.1, x0 + 0.025, fw) + fBox(x, x1 - 0.025, x1 + 0.1, fw)));
    rough = 0.7;
  } else if (kind == 5) {
    // AC louvre screen.
    float screen = fBox(ml, p0, p1, fh) * fBox(x, x0, x1, fw);
    float slats = fLines(ml, 0.09, 0.022, fh);
    vec3 louvre = mix(trimC * 0.95, vec3(0.08), slats);
    louvre = mix(louvre, trimC, fBox(x, x0, x0 + 0.04, fw) + fBox(x, x1 - 0.04, x1, fw));
    colour = mix(colour, louvre, screen);
    rough = mix(rough, 0.6, screen);
  } else if (kind == 7) {
    // Curtain wall: mullions, transom at the spandrel, glass everywhere else.
    float m = max(p0, 0.6);
    float mull = max(fLines(x, m, 0.035, fw), fLines(ml, H, 0.035, fh));
    mull = max(mull, fLines(ml - p1, H * 4.0, 0.03, fh));
    glassCov = 1.0 - mull;
    gu = fract(x / m);
    gv = ml < p1 ? 0.0 : (ml - p1) / max(H - p1, 0.5);
    ow = m;
    oh = H - p1;
    colour = frameC;
    rough = 0.4;
  } else if (kind == 8) {
    float joint = max(fLines(ml, max(tileModule, 0.3), 0.005, fh), fBox(x, (x0 + x1) * 0.5 - 0.004, (x0 + x1) * 0.5 + 0.004, fw));
    colour = skinC * (1.0 - 0.2 * joint);
  } else if (kind == 9) {
    colour = skinC * (1.0 - 0.25 * fLines(ml, 1.2, 0.006, fh));
    rough = 0.45;
  }

  if (opening > 0.5) {
    float inOpen = fBox(x, ox0, ox1, fw) * fBox(ml, oy0, oy1, fh);
    float f = 0.055;
    float inGlass = fBox(x, ox0 + f, ox1 - f, fw) * fBox(ml, oy0 + f, oy1 - f, fh);
    float pw = (ow - 2.0 * f) / float(max(panes, 1));
    float gx = x - ox0 - f;
    float mull = panes > 1 ? fLines(gx, pw, 0.022, fw) * fBox(gx, pw * 0.5, ow - 2.0 * f - pw * 0.5, fw) : 0.0;
    float transom = 0.0;
    if (sash == 1 && oh > 1.5) transom = fBox(ml, oy1 - 0.55, oy1 - 0.51, fh);
    if (sash == 2) transom = fBox(ml, oy1 - 0.62, oy1 - 0.58, fh);
    if (sash == 0 && panes >= 2) mull = max(mull, fBox(gx, pw - 0.07, pw - 0.04, fw));
    float lattice = sash == 3 ? fLines(gx, 0.12, 0.012, fw) * 0.85 : 0.0;
    float glass = inGlass * (1.0 - mull) * (1.0 - transom);
    // Security grilles over the lowest storeys.
    float grille = 0.0;
    if (s < grilles) grille = max(fLines(gx, 0.11, 0.008, fw), fLines(ml, 0.55, 0.01, fh)) * inOpen;
    vec3 frameCol = mix(frameC, vec3(0.20, 0.18, 0.16), grille);
    colour = mix(colour, frameCol, clamp(inOpen - glass + grille * glass, 0.0, 1.0));
    rough = mix(rough, 0.45, inOpen);
    glassCov = glass * (1.0 - grille) * (1.0 - lattice);
    colour = mix(colour, trimC * 0.35, lattice * glass);
    gu = clamp((x - ox0) / ow, 0.0, 1.0);
    gv = clamp((ml - oy0) / oh, 0.0, 1.0);
    // The sill, its drip, and the shadowed head reveal.
    float sill = fBox(x, ox0 - 0.05, ox1 + 0.05, fw) * fBox(ml, oy0 - 0.06, oy0, fh);
    colour = mix(colour, trimC, sill * step(0.5, float(kind == 2)));
    float head = fBox(x, ox0, ox1, fw) * fBox(ml, oy1, oy1 + 0.09, fh);
    colour *= 1.0 - 0.32 * head;
    if (surround) {
      float ring = fBox(x, ox0 - 0.14, ox1 + 0.14, fw) * fBox(ml, oy0 - 0.16, oy1 + 0.18, fh) * (1.0 - inOpen);
      colour = mix(colour, trimC, ring);
    }
    if (kind == 4) {
      // Under the balcony above: the soffit's shadow on the door wall.
      colour *= 1.0 - 0.25 * fBox(ml, oy1 + 0.05, H + 0.01, fh);
    }
    if (kind == 10) {
      // A loggia: the whole span is a recess in shadow, its rail in front.
      float recess = fBox(x, x0, x1, fw) * fBox(ml, 0.0, H - 0.16, fh) * (1.0 - inOpen);
      colour = mix(colour, skinC * 0.42, recess);
      float rail = fBox(x, x0, x1, fw) * fBox(ml, 0.0, 1.1, fh);
      int railKind = style;
      vec3 railC = railKind == 0 ? skinC : railKind == 3 ? trimC : vec3(0.10, 0.13, 0.14);
      float railCov = railKind == 3 ? max(fLines(x, 0.12, 0.015, fw), fBox(ml, 1.02, 1.1, fh)) : 1.0;
      colour = mix(colour, railC, rail * railCov);
      glassCov *= 1.0 - rail * railCov;
      if (railKind == 1) { fGlass = rail; }
    }
  }

  // --- glass ----------------------------------------------------------------
  if (glassCov > 0.001) {
    float spandrel = kind == 7 && ml < p1 ? 1.0 : 0.0;
    float office = glassKind >= 2 ? 1.0 : 0.0;
    // Which window this is: the row, the span, the flat, the storey, and on a
    // curtain wall the tileModule.
    float cell = kind == 7 ? floor(U / max(p0, 0.6)) : 0.0;
    float seed = fHash(vec3(float(row) * 1.37 + float(spanIndex) * 0.61, k * 3.1 + s * 17.3, cell * 0.73)) * 1000.0;
    float g2 = fHash(vec3(seed, 3.0, 4.0));
    float g3 = fHash(vec3(seed, 5.0, 6.0));
    float g4 = fHash(vec3(seed, 7.0, 8.0));
    float detail = 1.0 - smoothstep(0.03, 0.10, max(fw, fh));
    float far = smoothstep(0.08, 0.25, max(fw, fh));

    // Curtains in the plane of the glass, by window.
    float du = fw / ow * 1.5;
    float curtain = 0.0;
    vec3 curtainC = vec3(0.0);
    if (office > 0.5) {
      if (g2 < 0.45) {
        float f = 0.15 + 0.85 * g3;
        curtain = smoothstep(1.0 - f - 0.01, 1.0 - f + 0.01, gv);
        curtainC = vec3(0.45, 0.45, 0.43) * (0.8 + 0.2 * fLines(gv * oh, 0.05, 0.012, fh));
      }
    } else if (g2 < 0.30) {
      curtain = 0.0;
    } else if (g2 < 0.68) {
      float f = 0.25 + 0.75 * g3;
      curtain = 0.82 * (g4 < 0.5 ? smoothstep(f + du, f - du, gu) : smoothstep(1.0 - f - du, 1.0 - f + du, gu));
      curtainC = vec3(0.62, 0.60, 0.55);
    } else if (g2 < 0.9) {
      float f = 0.3 + 0.7 * g3;
      curtain = smoothstep(f * 0.5 + du, f * 0.5 - du, gu) + smoothstep(1.0 - f * 0.5 - du, 1.0 - f * 0.5 + du, gu);
      curtainC = g4 < 0.3 ? vec3(0.42, 0.33, 0.22) : g4 < 0.55 ? vec3(0.22, 0.26, 0.33) : g4 < 0.8 ? vec3(0.55, 0.50, 0.40) : vec3(0.26, 0.16, 0.12);
    } else {
      float f = 0.2 + 0.8 * g3;
      curtain = smoothstep(1.0 - f - 0.01, 1.0 - f + 0.01, gv);
      curtainC = vec3(0.56, 0.54, 0.50);
    }
    curtain = clamp(curtain, 0.0, 1.0);
    float folds = 1.0 - 0.12 * (1.0 - smoothstep(0.02, 0.05, fw)) * (0.5 - 0.5 * sin(gu * ow / 0.11 * 6.2831));
    vec3 curtainLit = curtainC * folds * uInteriorLight * 1.6;

    // The room: traced near, averaged far.
    vec3 room = vec3(0.0);
    vec3 roomMean = vec3(0.55, 0.52, 0.47) * uInteriorLight * 0.55;
    if (detail > 0.001 && uInteriorMap > 0.5 && spandrel < 0.5) {
      vec3 nV = normalize(vNormal) * (gl_FrontFacing ? 1.0 : -1.0);
      vec3 nW = normalize((vec4(nV, 0.0) * viewMatrix).xyz);
      vec3 tW = normalize(cross(vec3(0.0, 1.0, 0.0), nW));
      vec3 dpx = dFdx(vFacadeWorld);
      vec3 dpy = dFdy(vFacadeWorld);
      if (dot(tW, dpx) * dFdx(U) + dot(tW, dpy) * dFdy(U) < 0.0) tW = -tW;
      vec3 rdW = normalize(vFacadeWorld - cameraPosition);
      vec3 rd = vec3(dot(rdW, tW), rdW.y, -dot(rdW, nW));
      // The room spans the opening and a little more, centred on it, in
      // global U so mirrored flats need no special case.
      float uo0 = kind == 7 ? cell * max(p0, 0.6) : odd ? k * W + (W - ox1) : k * W + ox0;
      float roomW = max(ow + 1.4, office > 0.5 ? 4.0 : 3.0);
      float px = U - (uo0 + ow * 0.5) + roomW * 0.5;
      vec3 size = vec3(roomW, H - 0.15, office > 0.5 ? 7.0 : 4.2);
      room = fRoom(vec2(px, ml), rd, size, seed, office);
    }
    vec3 inside = mix(roomMean, room, detail);
    inside = mix(inside, curtainLit, curtain);
    if (spandrel > 0.5) inside = vec3(0.012);
    float transmit = office > 0.5 ? 0.45 : glassKind == 1 ? 0.6 : 0.8;
    vec3 glassTint = glassKind == 1 ? mix(vec3(1.0), coatC * 1.5, 0.5) : glassKind == 3 ? mix(vec3(1.0), coatC * 1.4, 0.6) : vec3(1.0);
    inside *= transmit * glassTint;
    inside = mix(inside, vec3(0.55, 0.52, 0.47) * uInteriorLight * 0.45 * transmit, far);

    fEmit = inside * glassCov;
    fGlass = max(fGlass, glassCov);
    colour = mix(colour, vec3(0.0), glassCov);
    rough = mix(rough, 0.035, glassCov);
    fF0 = glassKind == 3 ? mix(vec3(0.04), coatC, 0.3) : glassKind == 2 ? vec3(0.10, 0.11, 0.12) : glassKind == 1 ? mix(vec3(0.05), coatC * 0.12, 0.5) : vec3(0.045);
    // Each pane is glazed a fraction of a degree off true.
    float pane = floor(gu * float(max(panes, 1)));
    fTilt = (vec2(fHash(vec3(seed, pane, 41.0)), fHash(vec3(seed, pane, 43.0))) - 0.5) * 0.06;
  }
  fRough = rough;
  diffuseColor.rgb = diffuse * colour;
  diffuseColor.a = 1.0;
}
`;

const ROUGHNESS_CHUNK = /* glsl */ `
#include <roughnessmap_fragment>
roughnessFactor = fRough;
`;

const NORMAL_CHUNK = /* glsl */ `
#include <normal_fragment_maps>
normal = normalize(normal + fGlass * vec3(fTilt, 0.0));
`;

const SPECULAR_CHUNK = /* glsl */ `
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, fF0, fGlass);
material.specularColorBlended = mix(material.specularColorBlended, fF0, fGlass);
`;

const EMISSIVE_CHUNK = /* glsl */ `
#include <emissivemap_fragment>
totalEmissiveRadiance += fEmit;
`;

/**
 * Daylight inside a room, as radiance on a white surface just inside the
 * window.  About a tenth of a sunlit wall's: what a photograph of a tower in
 * daylight shows through its windows.
 */
export const INTERIOR_LIGHT = 0.16;

/** The `cnfacade` material, reading `table` (the `cnfacade/table` bake). */
export function createFacadeMaterial(table: THREE.Texture): THREE.MeshStandardMaterial {
  const material = new THREE.MeshStandardMaterial({
    color: 0xffffff,
    roughness: 0.9,
    metalness: 0,
    envMapIntensity: 1.0,
    vertexColors: true,
  });
  material.defines = { ...(material.defines ?? {}), USE_UV: "" };
  material.onBeforeCompile = (shader) => {
    shader.uniforms.uFacadeTable = { value: table };
    shader.uniforms.uInteriorLight = { value: INTERIOR_LIGHT };
    // `?interior=0` turns interior mapping off, for measuring what it costs.
    shader.uniforms.uInteriorMap = {
      value: new URLSearchParams(window.location.search).get("interior") === "0" ? 0 : 1,
    };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", `#include <common>\n${VERTEX_PARS}`)
      .replace("#include <worldpos_vertex>", VERTEX_MAIN);
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", `#include <common>\n${PARS}`)
      .replace("#include <color_fragment>", COLOR_CHUNK)
      .replace("#include <roughnessmap_fragment>", ROUGHNESS_CHUNK)
      .replace("#include <normal_fragment_maps>", NORMAL_CHUNK)
      .replace("#include <lights_physical_fragment>", SPECULAR_CHUNK)
      .replace("#include <emissivemap_fragment>", EMISSIVE_CHUNK);
  };
  material.customProgramCacheKey = () => "cn-facade";
  return material;
}

/**
 * The elevation table as a texture: linear bytes, one texel per field, read
 * with `texelFetch` and never filtered.
 */
export function facadeTableTexture(bytes: Uint8Array, width: number, height: number): THREE.DataTexture {
  const texture = new THREE.DataTexture(bytes, width, height, THREE.RGBAFormat);
  texture.colorSpace = THREE.NoColorSpace;
  texture.magFilter = THREE.NearestFilter;
  texture.minFilter = THREE.NearestFilter;
  texture.generateMipmaps = false;
  texture.flipY = false;
  texture.needsUpdate = true;
  return texture;
}
