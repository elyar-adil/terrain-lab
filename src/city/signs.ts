/**
 * Shop signs, hanging signs, company lettering and the barber pole, drawn with
 * canvas text and vector shapes.
 *
 * Rust (`facades/cn_signs.rs`) owns the catalogue: which designs exist, their
 * wording, colours, style, font class and mark, and where each sits in its
 * sheet.  It ships that as a UTF-8 manifest in the `sign/manifest` texture.
 * This file only rasterises it.  Brand names are plain text in a colour
 * scheme; every mark is a simple geometric shape drawn here, no imagery.
 */
import * as THREE from "three";

interface SheetDef {
  cols: number;
  rows: number;
  cw: number;
  ch: number;
}

interface BoardSpec {
  cell: number;
  bays: number;
  style: number;
  font: number;
  bg: string;
  fg: string;
  acc: string;
  mark: number;
  text: string;
  sub: string;
}

interface VertSpec {
  cell: number;
  style: number;
  font: number;
  bg: string;
  fg: string;
  acc: string;
  text: string;
}

interface CrownSpec {
  cell: number;
  style: number;
  font: number;
  colour: string;
  mark: number;
  name: string;
}

export interface SignTextures {
  board: THREE.Texture;
  vert: THREE.Texture;
  crown: THREE.Texture;
  pole: THREE.Texture;
}

const SANS =
  '"Microsoft YaHei","PingFang SC","Noto Sans CJK SC","Source Han Sans SC","WenQuanYi Zen Hei","Noto Sans SC",sans-serif';
const HEI = '"SimHei","Heiti SC","Microsoft YaHei","PingFang SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif';
const SONG = '"SimSun","Songti SC","Noto Serif CJK SC","Source Han Serif SC","STSong","WenQuanYi Zen Hei",serif';
const KAI = '"KaiTi","STKaiti","Kaiti SC","AR PL UKai CN","DFKai-SB","WenQuanYi Zen Hei",serif';
const ROUND = '"Microsoft YaHei UI","PingFang SC","HarmonyOS Sans SC","Noto Sans CJK SC","WenQuanYi Zen Hei",sans-serif';

/** Font class → CSS weight and family. */
const FONTS: Array<[number, string, number]> = [
  [800, SANS, 0.0],
  [900, HEI, 0.035],
  [700, SONG, 0.02],
  [400, KAI, 0.045],
  [700, ROUND, 0.02],
];

const hex = (value: string) => `#${value}`;

function parseSheet(parts: string[]): SheetDef {
  return { cols: +parts[2], rows: +parts[3], cw: +parts[4], ch: +parts[5] };
}

function canvas(w: number, h: number): CanvasRenderingContext2D | null {
  if (typeof document === "undefined") return null;
  const element = document.createElement("canvas");
  element.width = w;
  element.height = h;
  return element.getContext("2d");
}

/** Per-character advance of `text` at `size`, with `gap` em between glyphs. */
function advances(g: CanvasRenderingContext2D, text: string, gap: number, size: number): number[] {
  return [...text].map((c) => g.measureText(c).width + gap * size);
}

interface TextOpts {
  fill: string;
  size: number;
  font: number;
  /** Letter gap in em; two-character names are spaced out wider. */
  gap?: number;
  maxW: number;
  stroke?: string;
  strokeW?: number;
  shadow?: [number, number, string];
  glow?: string;
  align?: "center" | "left";
}

/** One line of text centred at (cx, cy), condensed to fit `maxW`. */
function drawLine(g: CanvasRenderingContext2D, text: string, cx: number, cy: number, o: TextOpts) {
  const [weight, family, thick] = FONTS[o.font % FONTS.length];
  g.save();
  g.font = `${weight} ${o.size}px ${family}`;
  g.textBaseline = "middle";
  g.textAlign = "left";
  const n = [...text].length;
  const gap = o.gap ?? (n <= 2 ? 0.28 : n === 3 ? 0.06 : 0.0);
  const adv = advances(g, text, gap, o.size);
  const total = adv.reduce((a, b) => a + b, 0) - gap * o.size;
  const k = total > o.maxW ? o.maxW / total : 1;
  g.translate(cx, cy);
  g.scale(k, 1);
  let x = o.align === "left" ? 0 : -total / 2;
  if (o.shadow) {
    g.fillStyle = o.shadow[2];
    let sx = x;
    [...text].forEach((c, i) => {
      g.fillText(c, sx + o.shadow![0], o.shadow![1]);
      sx += adv[i];
    });
  }
  if (o.glow) {
    g.shadowColor = o.glow;
    g.shadowBlur = o.size * 0.25;
  }
  g.lineJoin = "round";
  [...text].forEach((c, i) => {
    if (o.stroke && o.strokeW) {
      g.lineWidth = o.strokeW;
      g.strokeStyle = o.stroke;
      g.strokeText(c, x, 0);
    }
    g.fillStyle = o.fill;
    g.fillText(c, x, 0);
    if (thick > 0) {
      g.lineWidth = o.size * thick;
      g.strokeStyle = o.fill;
      g.strokeText(c, x, 0);
    }
    x += adv[i];
  });
  g.restore();
}

function roundRect(g: CanvasRenderingContext2D, x: number, y: number, w: number, h: number, r: number) {
  g.beginPath();
  g.moveTo(x + r, y);
  g.arcTo(x + w, y, x + w, y + h, r);
  g.arcTo(x + w, y + h, x, y + h, r);
  g.arcTo(x, y + h, x, y, r);
  g.arcTo(x, y, x + w, y, r);
  g.closePath();
}

/** Simple geometric marks, centred at (x, y), fitting a circle of radius r. */
function drawMark(
  g: CanvasRenderingContext2D,
  kind: number,
  x: number,
  y: number,
  r: number,
  fg: string,
  bg: string,
  acc: string,
) {
  g.save();
  g.translate(x, y);
  g.lineCap = "round";
  g.lineJoin = "round";
  const disc = (cx: number, cy: number, rr: number, c: string) => {
    g.fillStyle = c;
    g.beginPath();
    g.arc(cx, cy, rr, 0, Math.PI * 2);
    g.fill();
  };
  switch (kind) {
    case 1: {
      // Snowman: two stacked discs, a crown, eyes and a scarf.
      disc(0, r * 0.42, r * 0.58, fg);
      disc(0, -r * 0.3, r * 0.42, fg);
      g.fillStyle = acc;
      g.beginPath();
      g.moveTo(-r * 0.36, -r * 0.55);
      g.lineTo(-r * 0.3, -r * 0.95);
      g.lineTo(-r * 0.1, -r * 0.7);
      g.lineTo(0, -r);
      g.lineTo(r * 0.1, -r * 0.7);
      g.lineTo(r * 0.3, -r * 0.95);
      g.lineTo(r * 0.36, -r * 0.55);
      g.closePath();
      g.fill();
      disc(-r * 0.14, -r * 0.32, r * 0.06, bg);
      disc(r * 0.14, -r * 0.32, r * 0.06, bg);
      g.fillStyle = bg;
      g.fillRect(-r * 0.4, r * 0.0, r * 0.8, r * 0.1);
      break;
    }
    case 2: {
      // Two arches meeting as an M.
      g.strokeStyle = fg;
      g.lineWidth = r * 0.4;
      g.beginPath();
      g.moveTo(-r * 0.88, r * 0.85);
      g.bezierCurveTo(-r * 0.88, -r * 1.1, -r * 0.02, -r * 1.1, -r * 0.02, r * 0.3);
      g.moveTo(r * 0.88, r * 0.85);
      g.bezierCurveTo(r * 0.88, -r * 1.1, r * 0.02, -r * 1.1, r * 0.02, r * 0.3);
      g.stroke();
      break;
    }
    case 3: {
      // Striped-bucket badge.
      roundRect(g, -r * 0.85, -r * 0.85, r * 1.7, r * 1.7, r * 0.3);
      g.fillStyle = fg;
      g.fill();
      g.fillStyle = acc;
      for (let i = -2; i <= 2; i++) g.fillRect(i * r * 0.3 - r * 0.07, -r * 0.5, r * 0.14, r * 1.1);
      g.fillRect(-r * 0.6, -r * 0.7, r * 1.2, r * 0.16);
      break;
    }
    case 4: {
      // Ringed disc with a star.
      disc(0, 0, r, fg);
      disc(0, 0, r * 0.84, acc);
      disc(0, 0, r * 0.74, fg);
      disc(0, 0, r * 0.66, acc);
      g.fillStyle = fg;
      g.beginPath();
      for (let i = 0; i < 10; i++) {
        const a = -Math.PI / 2 + (i * Math.PI) / 5;
        const rr = i % 2 === 0 ? r * 0.5 : r * 0.2;
        g.lineTo(Math.cos(a) * rr, Math.sin(a) * rr);
      }
      g.closePath();
      g.fill();
      break;
    }
    case 5: {
      // Round head with two antlers.
      disc(0, r * 0.3, r * 0.62, fg);
      g.strokeStyle = fg;
      g.lineWidth = r * 0.14;
      for (const s of [-1, 1]) {
        g.beginPath();
        g.moveTo(s * r * 0.3, -r * 0.2);
        g.lineTo(s * r * 0.55, -r * 0.8);
        g.moveTo(s * r * 0.45, -r * 0.5);
        g.lineTo(s * r * 0.85, -r * 0.55);
        g.stroke();
      }
      disc(-r * 0.2, r * 0.25, r * 0.07, bg);
      disc(r * 0.2, r * 0.25, r * 0.07, bg);
      break;
    }
    case 6: {
      // Three nested arcs, like a signal.
      g.strokeStyle = fg;
      g.lineWidth = r * 0.17;
      for (let i = 1; i <= 3; i++) {
        g.beginPath();
        g.arc(-r * 0.4, r * 0.5, r * 0.45 * i, -Math.PI * 0.45, 0.0);
        g.stroke();
      }
      disc(-r * 0.4, r * 0.5, r * 0.12, fg);
      break;
    }
    case 7: {
      // Two interlocked loops.
      g.strokeStyle = fg;
      g.lineWidth = r * 0.2;
      for (const s of [-1, 1]) {
        g.beginPath();
        g.arc(s * r * 0.36, 0, r * 0.52, 0, Math.PI * 2);
        g.stroke();
      }
      break;
    }
    case 8: {
      // An open ring with a bar: a stylised e.
      g.strokeStyle = fg;
      g.lineWidth = r * 0.24;
      g.beginPath();
      g.arc(0, 0, r * 0.72, 0.35, Math.PI * 2 - 0.05);
      g.stroke();
      g.fillStyle = acc;
      g.fillRect(-r * 0.72, -r * 0.08, r * 1.2, r * 0.17);
      break;
    }
    case 9: {
      // Coin: disc with a square hole.
      disc(0, 0, r, fg);
      g.fillStyle = bg;
      g.fillRect(-r * 0.36, -r * 0.36, r * 0.72, r * 0.72);
      g.fillStyle = fg;
      g.fillRect(-r * 0.22, -r * 0.22, r * 0.44, r * 0.44);
      g.fillStyle = bg;
      g.fillRect(-r * 0.1, -r * 0.1, r * 0.2, r * 0.2);
      break;
    }
    case 10: {
      // A disc with a Chinese 工 cut out of it.
      disc(0, 0, r, fg);
      g.fillStyle = bg;
      g.fillRect(-r * 0.5, -r * 0.5, r * 1.0, r * 0.18);
      g.fillRect(-r * 0.09, -r * 0.5, r * 0.18, r * 1.0);
      g.fillRect(-r * 0.5, r * 0.32, r * 1.0, r * 0.18);
      break;
    }
    case 11: {
      // Rounded square with a square ring.
      roundRect(g, -r * 0.9, -r * 0.9, r * 1.8, r * 1.8, r * 0.28);
      g.fillStyle = fg;
      g.fill();
      g.strokeStyle = bg;
      g.lineWidth = r * 0.16;
      g.strokeRect(-r * 0.45, -r * 0.45, r * 0.9, r * 0.9);
      g.fillStyle = bg;
      g.fillRect(-r * 0.9, -r * 0.08, r * 1.8, r * 0.16);
      break;
    }
    case 12: {
      // Cross.
      g.fillStyle = fg;
      roundRect(g, -r * 0.3, -r * 0.9, r * 0.6, r * 1.8, r * 0.1);
      g.fill();
      roundRect(g, -r * 0.9, -r * 0.3, r * 1.8, r * 0.6, r * 0.1);
      g.fill();
      break;
    }
    case 13: {
      // Hexagon with a stripe.
      g.fillStyle = fg;
      g.beginPath();
      for (let i = 0; i < 6; i++) {
        const a = Math.PI / 6 + (i * Math.PI) / 3;
        g.lineTo(Math.cos(a) * r * 0.95, Math.sin(a) * r * 0.95);
      }
      g.closePath();
      g.fill();
      g.fillStyle = bg;
      g.fillRect(-r * 0.7, -r * 0.1, r * 1.4, r * 0.2);
      break;
    }
    case 14: {
      // Chevron mark.
      g.fillStyle = fg;
      g.beginPath();
      g.moveTo(-r * 0.9, r * 0.8);
      g.lineTo(0, -r * 0.85);
      g.lineTo(r * 0.9, r * 0.8);
      g.lineTo(r * 0.45, r * 0.8);
      g.lineTo(0, -r * 0.05);
      g.lineTo(-r * 0.45, r * 0.8);
      g.closePath();
      g.fill();
      break;
    }
    case 15: {
      // A sweeping check.
      g.fillStyle = fg;
      g.beginPath();
      g.moveTo(-r * 0.95, -r * 0.1);
      g.lineTo(-r * 0.5, -r * 0.35);
      g.lineTo(-r * 0.12, r * 0.25);
      g.lineTo(r * 0.7, -r * 0.8);
      g.lineTo(r * 0.95, -r * 0.7);
      g.lineTo(-r * 0.1, r * 0.85);
      g.closePath();
      g.fill();
      break;
    }
    case 16: {
      // A milk bottle.
      g.fillStyle = fg;
      roundRect(g, -r * 0.5, -r * 0.1, r * 1.0, r * 1.0, r * 0.2);
      g.fill();
      g.fillRect(-r * 0.26, -r * 0.7, r * 0.52, r * 0.7);
      g.fillStyle = acc;
      g.fillRect(-r * 0.32, -r * 0.9, r * 0.64, r * 0.2);
      g.fillStyle = bg;
      g.fillRect(-r * 0.5, r * 0.2, r * 1.0, r * 0.14);
      break;
    }
    case 17: {
      // A bowl with steam.
      g.fillStyle = fg;
      g.beginPath();
      g.arc(0, 0, r * 0.85, 0, Math.PI);
      g.closePath();
      g.fill();
      g.strokeStyle = fg;
      g.lineWidth = r * 0.1;
      for (const s of [-1, 0, 1]) {
        g.beginPath();
        g.moveTo(s * r * 0.4, -r * 0.1);
        g.bezierCurveTo(s * r * 0.4 - r * 0.2, -r * 0.35, s * r * 0.4 + r * 0.2, -r * 0.5, s * r * 0.4, -r * 0.8);
        g.stroke();
      }
      break;
    }
    case 18: {
      // A hat roof.
      g.fillStyle = fg;
      g.beginPath();
      g.ellipse(0, r * 0.1, r * 0.95, r * 0.5, 0, Math.PI, 0);
      g.fill();
      g.fillRect(-r * 0.9, r * 0.05, r * 1.8, r * 0.28);
      break;
    }
    case 19: {
      // A leaf.
      g.fillStyle = fg;
      g.beginPath();
      g.moveTo(-r * 0.8, r * 0.8);
      g.bezierCurveTo(-r * 0.9, -r * 0.6, r * 0.2, -r * 1.0, r * 0.85, -r * 0.8);
      g.bezierCurveTo(r * 0.9, -r * 0.0, r * 0.3, r * 0.8, -r * 0.8, r * 0.8);
      g.fill();
      g.strokeStyle = bg;
      g.lineWidth = r * 0.08;
      g.beginPath();
      g.moveTo(-r * 0.7, r * 0.7);
      g.lineTo(r * 0.5, -r * 0.5);
      g.stroke();
      break;
    }
    case 22: {
      // A cup.
      g.fillStyle = fg;
      g.beginPath();
      g.moveTo(-r * 0.6, -r * 0.3);
      g.lineTo(r * 0.6, -r * 0.3);
      g.lineTo(r * 0.42, r * 0.9);
      g.lineTo(-r * 0.42, r * 0.9);
      g.closePath();
      g.fill();
      g.fillRect(-r * 0.7, -r * 0.5, r * 1.4, r * 0.2);
      g.strokeStyle = fg;
      g.lineWidth = r * 0.1;
      g.beginPath();
      g.moveTo(r * 0.1, -r * 0.5);
      g.lineTo(r * 0.3, -r * 1.0);
      g.stroke();
      break;
    }
    case 23: {
      // Bars: a gold lozenge.
      g.fillStyle = fg;
      g.beginPath();
      g.moveTo(0, -r * 0.9);
      g.lineTo(r * 0.7, 0);
      g.lineTo(0, r * 0.9);
      g.lineTo(-r * 0.7, 0);
      g.closePath();
      g.fill();
      g.fillStyle = bg;
      g.beginPath();
      g.moveTo(0, -r * 0.45);
      g.lineTo(r * 0.35, 0);
      g.lineTo(0, r * 0.45);
      g.lineTo(-r * 0.35, 0);
      g.closePath();
      g.fill();
      break;
    }
    default:
      break;
  }
  g.restore();
}

function drawBoard(g: CanvasRenderingContext2D, ox: number, oy: number, w: number, h: number, s: BoardSpec) {
  g.save();
  g.translate(ox, oy);
  g.beginPath();
  g.rect(0, 0, w, h);
  g.clip();
  const bg = hex(s.bg);
  const fg = hex(s.fg);
  const acc = hex(s.acc);
  g.fillStyle = bg;
  g.fillRect(0, 0, w, h);
  let textX = w / 2;
  let maxW = w * 0.9;
  let nameY = h * 0.5;
  let nameSize = h * 0.62;
  if (s.sub) {
    nameY = h * 0.42;
    nameSize = h * 0.5;
  }
  const subY = h * 0.84;
  const subSize = h * 0.17;
  const subColour = fg;
  switch (s.style) {
    case 1: {
      // Light box: a slight fall-off, an aluminium frame and an inner rule.
      const grad = g.createLinearGradient(0, 0, 0, h);
      grad.addColorStop(0, "rgba(255,255,255,0.16)");
      grad.addColorStop(0.5, "rgba(255,255,255,0.0)");
      grad.addColorStop(1, "rgba(0,0,0,0.12)");
      g.fillStyle = grad;
      g.fillRect(0, 0, w, h);
      g.strokeStyle = "rgba(200,205,210,0.9)";
      g.lineWidth = 5;
      g.strokeRect(2.5, 2.5, w - 5, h - 5);
      g.strokeStyle = acc;
      g.globalAlpha = 0.7;
      g.lineWidth = 2;
      g.strokeRect(9, 9, w - 18, h - 18);
      g.globalAlpha = 1;
      break;
    }
    case 2: {
      // 3D characters over a low band.
      g.fillStyle = acc;
      g.globalAlpha = 0.9;
      g.fillRect(0, h * 0.8, w, h * 0.2);
      g.globalAlpha = 1;
      nameSize = s.sub ? h * 0.56 : h * 0.72;
      nameY = s.sub ? h * 0.42 : h * 0.46;
      break;
    }
    case 3: {
      // Mark on the left, name on the right.
      const r = h * 0.36;
      const mx = h * 0.62;
      if (s.mark === 4) {
        // nothing: handled by the generic mark
      }
      drawMark(g, s.mark, mx, h * 0.5, r, fg, bg, acc);
      textX = (mx + r + w) / 2 + 2;
      maxW = w - (mx + r) - h * 0.28;
      nameSize = s.sub ? h * 0.46 : h * 0.54;
      nameY = s.sub ? h * 0.4 : h * 0.5;
      break;
    }
    case 5: {
      g.fillStyle = "#101114";
      g.fillRect(0, 0, w, h);
      g.strokeStyle = fg;
      g.globalAlpha = 0.55;
      g.lineWidth = 3;
      g.strokeRect(6, 6, w - 12, h - 12);
      g.globalAlpha = 1;
      break;
    }
    case 6: {
      // Striped fascia: coloured bands top and bottom.
      const palette =
        s.mark === 20 ? ["#f07a1a", "#00833e", "#e03a2c"] : [hex(s.acc), "#1b8a43", hex(s.acc)];
      const band = h * 0.075;
      palette.forEach((c, i) => {
        g.fillStyle = c;
        g.fillRect(0, i * band + 2, w, band * 0.8);
        g.fillRect(0, h - (i + 1) * band - 2, w, band * 0.8);
      });
      nameY = s.sub ? h * 0.46 : h * 0.5;
      nameSize = s.sub ? h * 0.4 : h * 0.46;
      break;
    }
    default: {
      g.strokeStyle = acc;
      g.globalAlpha = 0.55;
      g.lineWidth = 2;
      g.strokeRect(6, 6, w - 12, h - 12);
      g.globalAlpha = 1;
    }
  }
  const outlined = s.style === 2;
  drawLine(g, s.text, textX, nameY, {
    fill: fg,
    size: nameSize,
    font: s.font,
    maxW,
    stroke: outlined ? acc : undefined,
    strokeW: outlined ? nameSize * 0.09 : 0,
    shadow: outlined ? [nameSize * 0.045, nameSize * 0.06, "rgba(0,0,0,0.55)"] : undefined,
    glow: s.style === 5 ? fg : undefined,
  });
  if (s.sub) {
    drawLine(g, s.sub, textX, subY, {
      fill: subColour,
      size: subSize,
      font: 0,
      maxW: maxW * 0.95,
      gap: 0.04,
    });
  }
  g.restore();
}

function drawVert(g: CanvasRenderingContext2D, ox: number, oy: number, w: number, h: number, s: VertSpec) {
  g.save();
  g.translate(ox, oy);
  g.beginPath();
  g.rect(0, 0, w, h);
  g.clip();
  g.fillStyle = hex(s.bg);
  g.fillRect(0, 0, w, h);
  g.strokeStyle = hex(s.acc);
  g.globalAlpha = 0.8;
  g.lineWidth = 3;
  g.strokeRect(8, 8, w - 16, h - 16);
  g.globalAlpha = 1;
  const chars = [...s.text];
  const n = chars.length;
  const size = Math.min(w * 0.62, ((h - 44) / n) * 0.92);
  const step = (h - 44) / n;
  chars.forEach((c, i) => {
    drawLine(g, c, w / 2, 22 + step * (i + 0.5), {
      fill: hex(s.fg),
      size,
      font: s.font,
      maxW: w * 0.8,
      stroke: s.style === 2 ? hex(s.acc) : undefined,
      strokeW: s.style === 2 ? size * 0.08 : 0,
    });
  });
  g.restore();
}

function drawCrown(g: CanvasRenderingContext2D, oy: number, cw: number, ch: number, s: CrownSpec) {
  const pitch = 100;
  const logo = s.mark > 0 ? 120 : 0;
  const used = logo + [...s.name].length * pitch;
  const x0 = (cw - used) / 2;
  const colour = hex(s.colour);
  g.save();
  g.translate(0, oy);
  if (s.mark > 0) {
    const cx = x0 + 50;
    const cy = ch / 2;
    g.fillStyle = "rgba(0,0,0,0.55)";
    g.strokeStyle = "rgba(0,0,0,0.55)";
    g.save();
    g.translate(3, 4);
    logoShape(g, s.mark, cx, cy, 40, "rgba(0,0,0,0.55)");
    g.restore();
    logoShape(g, s.mark, cx, cy, 40, colour);
  }
  const [weight, family] = FONTS[s.font % FONTS.length];
  g.font = `${weight} 92px ${family}`;
  g.textBaseline = "middle";
  g.textAlign = "center";
  g.lineJoin = "round";
  [...s.name].forEach((c, i) => {
    const x = x0 + logo + pitch * (i + 0.5);
    const y = ch / 2 + 2;
    // A dark rim and drop shadow: the letters read on light parapets too.
    g.fillStyle = "rgba(0,0,0,0.5)";
    g.fillText(c, x + 4, y + 5);
    g.lineWidth = 6;
    g.strokeStyle = "rgba(20,22,26,0.75)";
    g.strokeText(c, x, y);
    g.fillStyle = colour;
    g.fillText(c, x, y);
    if (s.style === 1) {
      g.lineWidth = 2;
      g.strokeStyle = "rgba(255,255,255,0.35)";
      g.strokeText(c, x, y);
    }
  });
  g.restore();
}

/** Company logo shapes: ring, diamond, triangle, hexagon, wave, square-in-square. */
function logoShape(g: CanvasRenderingContext2D, kind: number, x: number, y: number, r: number, colour: string) {
  g.save();
  g.translate(x, y);
  g.fillStyle = colour;
  g.strokeStyle = colour;
  g.lineWidth = r * 0.28;
  g.lineJoin = "round";
  switch (kind) {
    case 1:
      g.beginPath();
      g.arc(0, 0, r * 0.85, 0, Math.PI * 2);
      g.stroke();
      g.beginPath();
      g.arc(0, 0, r * 0.3, 0, Math.PI * 2);
      g.fill();
      break;
    case 2:
      g.beginPath();
      g.moveTo(0, -r);
      g.lineTo(r * 0.8, 0);
      g.lineTo(0, r);
      g.lineTo(-r * 0.8, 0);
      g.closePath();
      g.fill();
      break;
    case 3:
      g.beginPath();
      g.moveTo(0, -r * 0.95);
      g.lineTo(r * 0.95, r * 0.8);
      g.lineTo(-r * 0.95, r * 0.8);
      g.closePath();
      g.fill();
      break;
    case 4:
      g.beginPath();
      for (let i = 0; i < 6; i++) {
        const a = (i * Math.PI) / 3;
        g.lineTo(Math.cos(a) * r, Math.sin(a) * r);
      }
      g.closePath();
      g.stroke();
      break;
    case 5:
      g.beginPath();
      g.moveTo(-r, r * 0.2);
      g.bezierCurveTo(-r * 0.5, -r * 0.9, 0, r * 0.9, r * 0.5, -r * 0.2);
      g.lineTo(r, -r * 0.5);
      g.stroke();
      break;
    default:
      g.strokeRect(-r * 0.8, -r * 0.8, r * 1.6, r * 1.6);
      g.fillRect(-r * 0.3, -r * 0.3, r * 0.6, r * 0.6);
  }
  g.restore();
}

function poleTexture(): THREE.Texture | null {
  const size = 128;
  const g = canvas(size, size);
  if (!g) return null;
  const image = g.createImageData(size, size);
  const bands: Array<[number, number, number]> = [
    [200, 28, 36],
    [244, 244, 240],
    [34, 74, 166],
    [244, 244, 240],
  ];
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const t = (((x + y) % size) / size) * bands.length;
      const i = Math.floor(t) % bands.length;
      const f = t - Math.floor(t);
      const j = (i + 1) % bands.length;
      // A soft edge a few texels wide.
      const m = f > 0.94 ? (f - 0.94) / 0.06 : 0;
      const o = (y * size + x) * 4;
      for (let c = 0; c < 3; c++) image.data[o + c] = bands[i][c] * (1 - m) + bands[j][c] * m;
      image.data[o + 3] = 255;
    }
  }
  g.putImageData(image, 0, 0);
  const texture = new THREE.CanvasTexture(g.canvas);
  texture.wrapS = THREE.RepeatWrapping;
  texture.wrapT = THREE.RepeatWrapping;
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.flipY = false;
  texture.anisotropy = 4;
  return texture;
}

function finish(g: CanvasRenderingContext2D): THREE.Texture {
  const texture = new THREE.CanvasTexture(g.canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  // Row 0 is the top, and the geometry counts V downward, as every other bake.
  texture.flipY = false;
  texture.generateMipmaps = true;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  texture.anisotropy = 8;
  texture.needsUpdate = true;
  return texture;
}

/** Rasterise every sheet the manifest describes.  Null without a DOM. */
export function buildSignTextures(manifestBytes: Uint8Array): SignTextures | null {
  if (typeof document === "undefined") return null;
  const text = new TextDecoder("utf-8").decode(manifestBytes).replace(/\0+$/, "");
  const sheets: Record<string, SheetDef> = {};
  const boards: BoardSpec[] = [];
  const verts: VertSpec[] = [];
  const crowns: CrownSpec[] = [];
  for (const line of text.split("\n")) {
    if (!line) continue;
    const p = line.split("\t");
    switch (p[0]) {
      case "SHEET":
        sheets[p[1]] = parseSheet(p);
        break;
      case "B":
        boards.push({
          cell: +p[1],
          bays: +p[2],
          style: +p[3],
          font: +p[4],
          bg: p[5],
          fg: p[6],
          acc: p[7],
          mark: +p[8],
          text: p[9],
          sub: p[10] ?? "",
        });
        break;
      case "V":
        verts.push({
          cell: +p[1],
          style: +p[2],
          font: +p[3],
          bg: p[4],
          fg: p[5],
          acc: p[6],
          text: p[7],
        });
        break;
      case "C":
        crowns.push({
          cell: +p[1],
          style: +p[2],
          font: +p[3],
          colour: p[4],
          mark: +p[5],
          name: p[6],
        });
        break;
    }
  }
  const bs = sheets.board;
  const vs = sheets.vert;
  const cs = sheets.crown;
  if (!bs || !vs || !cs) return null;
  const bg = canvas(bs.cols * bs.cw, bs.rows * bs.ch);
  const vg = canvas(vs.cols * vs.cw, vs.rows * vs.ch);
  const cg = canvas(cs.cw, cs.rows * cs.ch);
  const pole = poleTexture();
  if (!bg || !vg || !cg || !pole) return null;
  for (const b of boards) {
    drawBoard(bg, (b.cell % bs.cols) * bs.cw, Math.floor(b.cell / bs.cols) * bs.ch, bs.cw * b.bays, bs.ch, b);
  }
  for (const v of verts) {
    drawVert(vg, (v.cell % vs.cols) * vs.cw, Math.floor(v.cell / vs.cols) * vs.ch, vs.cw, vs.ch, v);
  }
  for (const c of crowns) drawCrown(cg, c.cell * cs.ch, cs.cw, cs.ch, c);
  return { board: finish(bg), vert: finish(vg), crown: finish(cg), pole };
}
