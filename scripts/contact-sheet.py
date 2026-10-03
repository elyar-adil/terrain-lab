#!/usr/bin/env python3
"""Tile a folder of render-matrix PNGs into labelled contact sheets.

    python3 scripts/contact-sheet.py render-out/facades render-out/sheets/facades.jpg --cols 2

One label per tile (file name without extension). Output is JPEG so a sheet of a
dozen 1440x900 frames stays small enough to look at or attach.
"""
import argparse
import pathlib

from PIL import Image, ImageDraw, ImageFont


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("folder", type=pathlib.Path)
    parser.add_argument("out", type=pathlib.Path)
    parser.add_argument("--cols", type=int, default=2)
    parser.add_argument("--tile-width", type=int, default=720)
    parser.add_argument("--match", default="")
    args = parser.parse_args()

    files = sorted(p for p in args.folder.glob("*.png") if args.match in p.stem)
    if not files:
        raise SystemExit(f"no PNGs in {args.folder}")
    try:
        font = ImageFont.truetype("DejaVuSans.ttf", 22)
    except OSError:
        font = ImageFont.load_default()

    tiles = []
    for path in files:
        image = Image.open(path).convert("RGB")
        scale = args.tile_width / image.width
        image = image.resize((args.tile_width, round(image.height * scale)), Image.LANCZOS)
        draw = ImageDraw.Draw(image)
        label = path.stem
        box = draw.textbbox((8, 8), label, font=font)
        draw.rectangle((box[0] - 6, box[1] - 4, box[2] + 6, box[3] + 4), fill=(0, 0, 0))
        draw.text((8, 8), label, fill=(255, 255, 255), font=font)
        tiles.append(image)

    rows = (len(tiles) + args.cols - 1) // args.cols
    cell_h = max(t.height for t in tiles)
    sheet = Image.new("RGB", (args.cols * args.tile_width, rows * cell_h), (24, 24, 24))
    for index, tile in enumerate(tiles):
        sheet.paste(tile, ((index % args.cols) * args.tile_width, (index // args.cols) * cell_h))
    args.out.parent.mkdir(parents=True, exist_ok=True)
    sheet.save(args.out, quality=86)
    print(f"{args.out} ({len(tiles)} tiles, {sheet.width}x{sheet.height})")


if __name__ == "__main__":
    main()
