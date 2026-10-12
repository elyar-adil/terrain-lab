#!/usr/bin/env python3
"""Numbers for a render, so a change to a building is judged by value, not by eye.

    python3 scripts/measure-render.py render.png --box 0.3,0.2,0.7,0.6

--box is a fractional rectangle (x0,y0,x1,y1) of the image: the facade region to
measure. Reports the median sRGB colour, median relative luminance, the share of
pixels that are near-white (>0.75 luminance) and near-black (<0.15), and the
horizontal-edge density (how strongly floors read as bands) and vertical-edge
density (how strongly window columns read). Use the same box for the reference
photo and the render when comparing.
"""
import argparse

from PIL import Image
import numpy as np


def srgb_to_linear(c):
    c = c / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def main():
    p = argparse.ArgumentParser()
    p.add_argument("image")
    p.add_argument("--box", default="0.3,0.2,0.7,0.6")
    a = p.parse_args()
    img = np.asarray(Image.open(a.image).convert("RGB")).astype(np.float64)
    h, w, _ = img.shape
    x0, y0, x1, y1 = [float(v) for v in a.box.split(",")]
    crop = img[int(y0 * h):int(y1 * h), int(x0 * w):int(x1 * w)]
    lin = srgb_to_linear(crop)
    lum = 0.2126 * lin[..., 0] + 0.7152 * lin[..., 1] + 0.0722 * lin[..., 2]
    median_rgb = np.median(crop.reshape(-1, 3), axis=0)
    gy, gx = np.gradient(lum)
    horiz = np.mean(np.abs(gy) > 0.04)
    vert = np.mean(np.abs(gx) > 0.04)
    print(f"box {a.box} on {a.image} ({crop.shape[1]}x{crop.shape[0]} px)")
    print(f"median sRGB      {median_rgb.round().astype(int).tolist()}")
    print(f"median luminance {np.median(lum):.3f}  (reference cream ~0.6-0.7, grey-brown ~0.2-0.3)")
    print(f"near-white share {np.mean(lum > 0.75):.3f}   near-black share {np.mean(lum < 0.15):.3f}")
    print(f"edge density  horizontal {horiz:.3f}  vertical {vert:.3f}  (floor bands vs window columns)")


if __name__ == "__main__":
    main()
