/**
 * The facade of the merged far form of a city, drawn analytically from the
 * building's own look (`farLook.ts`).
 *
 * Why it is not a texture. A texture of a window grid has to be mip-mapped, and
 * a box-filtered mip of windows and wall is a flat grey: the building turns
 * into a plain box at the distance where it is still tens of pixels tall, and
 * anisotropic filtering cannot bring the floors back because the mips it picks
 * from are already grey. Here every pattern is a box integrated over the
 * pixel's own footprint, so a window a pixel wide is a darker, bluer pixel, the
 * slab lines stay as a faint tone, and only a pattern finer than a pixel fades
 * to its mean, which is the share it covers.
 *
 * What it draws, per building, from the table row the vertex points at:
 *
 * - floors: the storey height of the real design, so floor lines are where the
 *   detail has them; the shop storey taller than the rest, with a lit shopfront,
 *   a fascia band and piers, shuttered in some bays;
 * - bays: the wall is cut into whole flats of the design's width, mirrored at
 *   every second one, as the detail cuts it, with the design's window, its
 *   balcony door, its slab band and its window surrounds;
 * - curtain walls: mullions at the design's module, a dark spandrel at every
 *   slab, glass in the building's coating;
 * - colour: the body colour, a base band and a top band as the design has them
 *   (三段式), the glass colour, and each window its own curtain or blind;
 * - weathering: rain streaks and a dirty foot, the means of the detail's.
 *
 * Every fade is driven by the pixel footprint in metres, so there is no
 * distance threshold at which anything switches.
 */

import { INTERIOR_LIGHT } from "./facade";

export const MASS_VERTEX_PARS = /* glsl */ `
attribute float aBld;
attribute vec2 aFace;
varying float vBld;
varying vec2 vFace;
`;

export const MASS_VERTEX_MAIN = /* glsl */ `
vBld = aBld;
vFace = aFace;
`;

export const MASS_FRAGMENT_PARS = /* glsl */ `
uniform sampler2D uMassLook;
varying float vBld;
varying vec2 vFace;

vec4 mLook(int row, int texel) { return texelFetch(uMassLook, ivec2(texel, row), 0); }
float mHash(vec3 p) {
  p = fract(p * vec3(0.1031, 0.1030, 0.0973));
  p += dot(p, p.yxz + 33.33);
  return fract((p.x + p.y) * p.z);
}
float mNoise(vec2 p) {
  vec2 i = floor(p);
  vec2 f = fract(p);
  f = f * f * (3.0 - 2.0 * f);
  return mix(mix(mHash(vec3(i, 1.0)), mHash(vec3(i + vec2(1.0, 0.0), 1.0)), f.x),
             mix(mHash(vec3(i + vec2(0.0, 1.0), 1.0)), mHash(vec3(i + vec2(1.0, 1.0), 1.0)), f.x), f.y);
}
// 1 inside [a, b], integrated over a pixel of width w.
float mBox(float x, float a, float b, float w) {
  return clamp((x - a) / w + 0.5, 0.0, 1.0) * clamp((b - x) / w + 0.5, 0.0, 1.0);
}
// The same box repeating every 'period': once a pixel spans a good part of the
// period the position inside it is noise, so it fades to the box's share.
float mBoxP(float x, float a, float b, float w, float period) {
  float mean = clamp((b - a) / period, 0.0, 1.0);
  return mix(mBox(x, a, b, w), mean, smoothstep(0.2 * period, 0.8 * period, w));
}
// Lines of half-width hw every 'period', fading to their share.
float mLines(float x, float period, float hw, float w) {
  float d = abs(fract(x / period + 0.5) - 0.5) * period;
  float line = clamp((hw + w * 0.5 - d) / max(w, 1e-4), 0.0, 1.0);
  return mix(line, min(1.0, 2.0 * hw / period), smoothstep(0.25 * period, 0.7 * period, w));
}

// What is seen through one window: the room's mean, or a curtain or blind of
// this window's own. 'pix' is the pixel's size relative to the window, 0..1+.
vec3 mWindowInside(vec2 cell, float gu, float gv, float office, float pix, vec3 roomMean) {
  float g2 = mHash(vec3(cell, 3.0));
  float g3 = mHash(vec3(cell, 5.0));
  float g4 = mHash(vec3(cell, 7.0));
  float du = 0.04 + pix;
  float curtain = 0.0;
  vec3 curtainC = vec3(0.0);
  if (office > 0.5) {
    if (g2 < 0.45) {
      float f = 0.15 + 0.85 * g3;
      curtain = smoothstep(1.0 - f - du, 1.0 - f + du, gv);
      curtainC = vec3(0.45, 0.45, 0.43);
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
    curtain = smoothstep(1.0 - f - du, 1.0 - f + du, gv);
    curtainC = vec3(0.56, 0.54, 0.50);
  }
  curtain = clamp(curtain, 0.0, 1.0);
  return mix(roomMean, curtainC * uInteriorLightM * 1.6, curtain);
}
// The share of a window a curtain covers, and its mean colour, over all windows.
vec3 mWindowMean(float office, vec3 roomMean) {
  float cover = office > 0.5 ? 0.26 : 0.40;
  vec3 curtainMean = (office > 0.5 ? vec3(0.45, 0.45, 0.43) : vec3(0.50, 0.46, 0.40)) * uInteriorLightM * 1.6;
  return mix(roomMean, curtainMean, cover);
}
`;

/** Replaces `#include <color_fragment>` on the mass material, after the tile discard. */
export const MASS_COLOR_CHUNK = /* glsl */ `
float massGlass = 0.0;
float massRough = 0.92;
vec3 massEmit = vec3(0.0);
vec3 massF0 = vec3(0.04);
float massTone = 1.0;
#include <color_fragment>
float wallness = 1.0 - smoothstep(0.35, 0.7, abs(vMassNormal.y));
float hM = (vMassWorld.y - vBase) / uMetre;
float seedKind = floor(vSeed);
float seed = fract(vSeed);
if (vBld > -0.5 && wallness > 0.01) {
  int bi = int(vBld + 0.5);
  vec4 t0 = mLook(bi, 0);
  vec4 t1 = mLook(bi, 1);
  vec4 t2 = mLook(bi, 2);
  vec4 t3 = mLook(bi, 3);
  vec4 t4 = mLook(bi, 4);
  vec4 t5 = mLook(bi, 5);
  vec4 t6 = mLook(bi, 6);
  vec4 t7 = mLook(bi, 7);
  float storeyM = max(t0.a, 2.4);
  float baseN = mod(t1.a, 256.0);
  float total = floor(t1.a / 256.0);
  float topN = t2.a;
  float unitW = max(t3.x, 1.0);
  int lookKind = int(t3.w + 0.5);
  float shopM = t7.x;
  float weather = t7.y;
  int glassKind = int(t7.z + 0.5);
  int skinFlags = int(t6.a + 0.5);
  int flags = skinFlags - (skinFlags / 8) * 8;
  bool surround = (flags - (flags / 2) * 2) == 1;
  bool slabBand = ((flags / 2) - (flags / 4) * 2) == 1;
  bool office = glassKind >= 2;
  vec3 trimC = mix(t0.rgb, vec3(0.70, 0.70, 0.67), 0.35);

  // Which flat, and where in it: the wall is cut into whole flats, mirrored at
  // every second, as the detail cuts it.
  float L = max(vFace.y / uMetre, 0.5);
  float u = vFace.x / uMetre;
  float units = max(floor(L / unitW + 0.5), 1.0);
  float U = u * units * unitW / L;
  float k = floor(U / unitW);
  float x = U - k * unitW;
  if (mod(k, 2.0) > 0.5) x = unitW - x;
  float fw = max(fwidth(U), 1e-4);
  float fh = max(fwidth(hM), 1e-4);

  bool shop = shopM > 0.5 && hM < shopM;
  float hh = shopM > 0.5 ? hM - shopM : hM;
  float fl = floor(hh / storeyM);
  float ml = hh - fl * storeyM;
  float fadeCell = smoothstep(0.12 * unitW, 0.4 * unitW, fw);
  float fadeCellY = smoothstep(0.12 * storeyM, 0.4 * storeyM, fh);
  float fadeAny = max(fadeCell, fadeCellY);

  // The body colour, by storey: the base band, the body, the top band.
  vec3 wallC = t0.rgb;
  float inBase = (1.0 - smoothstep(baseN - 0.02 - fh / storeyM, baseN + 0.02 + fh / storeyM, hh / storeyM)) * step(0.5, baseN);
  float inTop = smoothstep(total - topN - 0.02 - fh / storeyM, total - topN + 0.02 + fh / storeyM, hh / storeyM) * step(0.5, topN);
  wallC = mix(wallC, t1.rgb, inBase);
  wallC = mix(wallC, t2.rgb, inTop);
  if (shop) wallC = t1.rgb;
  // Plant, parapets and gables have no design: they keep their own colour.
  if (lookKind == 4) wallC = diffuseColor.rgb;
  // Patch repairs and fading leave the paint a few per cent off from flat to flat.
  float patchN = mNoise(vec2(U * 0.23 + t0.a * 3.1, hh * 0.11 + t3.x)) - 0.5;
  float blotch = mNoise(vMassWorld.xz / uMetre * 0.31 + hM * 0.27) - 0.5;
  float wallTone = 1.0 + (0.05 + 0.07 * weather) * blotch + 0.07 * patchN;
  vec3 colour = wallC * wallTone;
  float rough = 0.9;

  float glassCov = 0.0;     // reflective glass, any kind
  float emitCov = 0.0;      // glass with a room behind it
  vec3 inside = vec3(0.0);
  vec3 roomMean = vec3(0.55, 0.52, 0.47) * uInteriorLightM * 0.55;

  if (lookKind == 4) {
    // No windows: shed, plant, parapet. Profiled cladding gets its ribs.
    float ribs = mLines(U, 0.3, 0.02, fw);
    colour *= 1.0 - 0.10 * ribs * step(1.5, seedKind) * (1.0 - step(2.5, seedKind));
  } else if (shop) {
    // The shop storey: piers, a lit shop window, a fascia sign, a stall riser,
    // and a roller shutter down in some bays.
    float cellId = mHash(vec3(k, fl, t0.a * 7.0));
    float shut = step(0.80, cellId);
    float pierW = 0.45;
    float win = mBoxP(x, pierW, unitW - pierW, fw, unitW) * mBoxP(hM, 0.55, min(3.35, shopM - 0.9), fh, shopM);
    float fascia = mBoxP(hM, min(3.45, shopM - 0.85), shopM - 0.12, fh, shopM);
    vec3 signC = mix(vec3(0.62, 0.10, 0.08), vec3(0.08, 0.20, 0.45), step(0.5, mHash(vec3(k, 3.0, t0.a))));
    signC = mix(signC, vec3(0.74, 0.70, 0.60), step(0.7, mHash(vec3(k, 5.0, t0.a * 3.0))));
    colour = mix(colour, mix(signC, wallC, fadeCell) * 0.9, fascia * (1.0 - 0.5 * fadeCell));
    vec3 shutter = vec3(0.42, 0.43, 0.42);
    colour = mix(colour, mix(shutter, wallC, fadeCell), win * shut);
    float lit = mix(0.35 + 0.65 * mHash(vec3(k, 11.0, t0.a)), 0.62, fadeCell);
    inside = vec3(1.0, 0.88, 0.68) * uInteriorLightM * 1.5 * lit;
    glassCov = win * (1.0 - shut);
    emitCov = glassCov;
  } else if (lookKind == 1) {
    // Curtain wall: mullions at the module, a transom at every slab, a dark
    // spandrel under the vision glass.
    float module = max(t5.w, 0.6);
    float mull = max(mLines(U, module, 0.035, fw), mLines(hh, storeyM, 0.035, fh));
    glassCov = 1.0 - mull;
    colour = mix(colour, t6.rgb, mull);
    float vision = mBoxP(ml, 0.9, storeyM - 0.04, fh, storeyM);
    emitCov = glassCov * vision;
    float pix = fw / module;
    vec2 cell = vec2(floor(U / module), fl) + t0.a * 31.0;
    float gu = fract(U / module);
    float gv = clamp((ml - 0.9) / max(storeyM - 0.9, 0.5), 0.0, 1.0);
    inside = mix(mWindowInside(cell, gu, gv, 1.0, pix, roomMean), mWindowMean(1.0, roomMean), smoothstep(0.15, 0.6, pix));
    // Pane to pane the coating reads a little differently: a tilt, a sky patch.
    massTone = mix(0.80 + 0.40 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
    // Upper floors take more sky.
    massTone *= mix(0.9, 1.12, smoothstep(0.0, 60.0, hM));
  } else {
    // Punched windows: grid, ribbon and the residential window all one rule.
    float sill = t4.x;
    float head = t4.y;
    float winX = mBoxP(x, t3.y, t3.z, fw, unitW);
    float winY = mBoxP(ml, sill, head, fh, storeyM);
    glassCov = winX * winY;
    emitCov = glassCov;
    float gu = clamp((x - t3.y) / max(t3.z - t3.y, 0.3), 0.0, 1.0);
    float gv = clamp((ml - sill) / max(head - sill, 0.3), 0.0, 1.0);
    float pix = fw / max(t3.z - t3.y, 0.3);
    vec2 cell = vec2(k, fl) + t0.a * 31.0;
    inside = mix(mWindowInside(cell, gu, gv, office ? 1.0 : 0.0, pix, roomMean), mWindowMean(office ? 1.0 : 0.0, roomMean), smoothstep(0.15, 0.6, pix));
    massTone = mix(0.85 + 0.30 * mHash(vec3(cell, 13.0)), 1.0, smoothstep(0.15, 0.6, pix));
    // Moulded surround and the shadowed head reveal.
    if (surround) {
      float ring = mBoxP(x, t3.y - 0.14, t3.z + 0.14, fw, unitW) * mBoxP(ml, sill - 0.16, head + 0.18, fh, storeyM);
      colour = mix(colour, trimC, clamp(ring - glassCov, 0.0, 1.0) * (1.0 - 0.4 * fadeAny));
    }
    colour *= 1.0 - 0.30 * mBoxP(x, t3.y, t3.z, fw, unitW) * mBoxP(ml, head, head + 0.1, fh, storeyM);
    // A balcony or loggia door in the flat.
    if (t4.w > 0.1) {
      float bx0 = t4.z + 0.3;
      float bx1 = t4.w - 0.3;
      float bhead = t7.w;
      float recess = mBoxP(x, t4.z, t4.w, fw, unitW) * mBoxP(ml, 0.0, bhead, fh, storeyM);
      colour *= 1.0 - 0.18 * recess;
      float door = mBoxP(x, bx0, bx1, fw, unitW) * mBoxP(ml, 1.1, bhead - 0.03, fh, storeyM);
      float dgu = clamp((x - bx0) / max(bx1 - bx0, 0.3), 0.0, 1.0);
      vec3 doorIn = mix(mWindowInside(cell + 5.0, dgu, 0.5, 0.0, fw / max(bx1 - bx0, 0.3), roomMean), mWindowMean(0.0, roomMean), smoothstep(0.15, 0.6, fw / max(bx1 - bx0, 0.3)));
      float gcOld = glassCov;
      glassCov = clamp(glassCov + door, 0.0, 1.0);
      inside = (inside * gcOld + doorIn * door) / max(glassCov, 1e-3);
      emitCov = glassCov;
    }
  }

  // The slab: a band of trim at the edge and the grime under it.
  if (lookKind != 4 && !shop) {
    if (slabBand) {
      float band = mBoxP(ml, -0.05, 0.22, fh, storeyM);
      colour = mix(colour, trimC, band * (1.0 - glassCov) * 0.8);
    }
    colour *= 1.0 - 0.10 * mBoxP(ml, storeyM - 0.12, storeyM + 0.01, fh, storeyM) * (0.5 + weather) * (1.0 - glassCov);
  }

  // Glass: the room's radiance and the coating, in place of the wall.
  float transmit = office ? 0.45 : glassKind == 1 ? 0.6 : 0.8;
  vec3 glassTint = glassKind == 1 ? mix(vec3(1.0), t5.rgb * 1.5, 0.5) : glassKind == 3 ? mix(vec3(1.0), t5.rgb * 1.4, 0.6) : vec3(1.0);
  if (lookKind == 1) transmit = 0.6;
  massEmit = inside * transmit * glassTint * emitCov * massTone;
  colour = mix(colour, vec3(0.0), glassCov);
  vec3 f0 = glassKind == 3 ? mix(vec3(0.04), t5.rgb, 0.3) : glassKind == 2 ? vec3(0.10, 0.11, 0.12) : glassKind == 1 ? mix(vec3(0.05), t5.rgb * 0.12, 0.5) : vec3(0.045);
  massF0 = f0;
  rough = mix(rough, 0.07, glassCov);
  massGlass = glassCov;

  // Dirt washed down the wall in columns: the detail's streaks, as their mean
  // once a column is under a pixel.
  float column = floor(U / 0.8);
  float columnR = fract(sin(column * 12.9898 + t0.a * 78.233 + t3.x) * 43758.5453);
  float runR = fract(sin(column * 4.13 + floor(hM / 9.0) * 7.71 + t3.x * 3.3) * 43758.5453);
  float streak = smoothstep(0.55, 0.95, columnR) * (0.55 + 0.45 * runR);
  streak = mix(streak, 0.13, smoothstep(0.1, 0.6, fw));
  colour *= 1.0 - vec3(0.85, 0.92, 1.0) * (0.10 + 0.20 * weather) * streak * (1.0 - glassCov);
  // A foot that is darker where the street's dirt and the neighbour's shade meet it.
  float foot = 1.0 - smoothstep(0.0, 6.0, hM);
  if (lookKind != 4) colour *= 1.0 - 0.22 * foot * (1.0 - glassCov * 0.5);

  diffuseColor.rgb = mix(diffuseColor.rgb, colour, wallness);
  massRough = mix(0.92, rough, wallness);
  massGlass *= wallness;
  massEmit *= wallness;
}
if (vBld > -0.5 && wallness <= 0.01 && vMassNormal.y > 0.5) {
  // A roof deck: screed, membrane or tile, patched and stained, never one flat grey.
  vec2 rp = vMassWorld.xz / uMetre;
  float rfw = length(fwidth(rp));
  float rn = mNoise(rp * 0.35) - 0.5;
  float rf = mNoise(rp * 1.9) - 0.5;
  float coarse = 1.0 + 0.10 * rn;
  float fineN = 1.0 + 0.06 * rf * (1.0 - smoothstep(0.3, 1.2, rfw));
  float joints = 1.0 - 0.07 * max(mLines(rp.x, 1.2, 0.012, rfw), mLines(rp.y, 3.0, 0.012, rfw));
  diffuseColor.rgb *= coarse * fineN * joints;
}
`;

export const MASS_ROUGHNESS_CHUNK = /* glsl */ `
#include <roughnessmap_fragment>
roughnessFactor = massRough;
`;

export const MASS_SPECULAR_CHUNK = /* glsl */ `
#include <lights_physical_fragment>
material.specularColor = mix(material.specularColor, massF0, massGlass);
material.specularColorBlended = mix(material.specularColorBlended, massF0, massGlass);
`;

export const MASS_EMISSIVE_CHUNK = /* glsl */ `
#include <emissivemap_fragment>
{
  // The room behind the glass, and a sky patch where there is no environment map.
  float facing = clamp(abs(dot(normalize(vViewPosition), normal)), 0.0, 1.0);
  float fresnel = pow(1.0 - facing, 3.0);
  totalEmissiveRadiance += massEmit;
  totalEmissiveRadiance += massGlass * massTone * vec3(0.030, 0.037, 0.048) * (0.4 + 3.0 * fresnel);
}
`;

/** The `INTERIOR_LIGHT` the shader's room radiance uses, as a literal. */
export const MASS_INTERIOR_DEFINE = `#define uInteriorLightM ${INTERIOR_LIGHT.toFixed(4)}\n`;
