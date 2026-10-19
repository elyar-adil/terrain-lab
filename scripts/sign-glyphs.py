#!/usr/bin/env python3
"""Rasterise the signboard characters into the atlas the shopfront texture reads.

Every character in `SIGN_PHRASES` (crates/city-scene/src/facades/cn_shopfront.rs)
is drawn from a CJK font, averaged down to a 24 by 24 cell, and packed to four
bits a cell.  Per character the atlas holds a little-endian u32 code point and
288 bytes, high nibble first, row 0 at the top of the character.

    python3 scripts/sign-glyphs.py [font.ttc]

The default font is WenQuanYi Zen Hei, which Ubuntu ships as fonts-wqy-zenhei.
"""

import re
import struct
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "crates/city-scene/src/facades/cn_shopfront.rs"
OUT = ROOT / "crates/city-scene/src/facades/sign_glyphs.bin"
CELLS = 24
DRAW = 64


def phrases() -> list[str]:
    text = SOURCE.read_text(encoding="utf-8")
    block = re.search(r"SIGN_PHRASES: &\[&str\] = &\[(.*?)\];", text, re.S)
    if block is None:
        raise SystemExit("SIGN_PHRASES not found in cn_shopfront.rs")
    return re.findall(r'"([^"]*)"', block.group(1))


def main() -> None:
    font_path = sys.argv[1] if len(sys.argv) > 1 else "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc"
    font = ImageFont.truetype(font_path, 60)
    chars = sorted({c for phrase in phrases() for c in phrase})
    out = bytearray()
    for c in chars:
        canvas = Image.new("L", (DRAW, DRAW), 0)
        draw = ImageDraw.Draw(canvas)
        left, top, right, bottom = draw.textbbox((0, 0), c, font=font)
        x = (DRAW - (right - left)) / 2 - left
        y = (DRAW - (bottom - top)) / 2 - top
        draw.text((x, y), c, font=font, fill=255)
        cell = canvas.resize((CELLS, CELLS), Image.Resampling.BOX)
        levels = [round(v * 15 / 255) for v in cell.tobytes()]
        packed = bytes((levels[i] << 4) | levels[i + 1] for i in range(0, len(levels), 2))
        out += struct.pack("<I", ord(c)) + packed
    OUT.write_bytes(bytes(out))
    print(f"{len(chars)} characters, {len(out)} bytes -> {OUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
