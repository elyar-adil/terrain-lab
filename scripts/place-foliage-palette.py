"""Place the species palette on **physically plausible reflectance**, not on hue.

This exists because the first attempt at the palette picked saturated, bright
colours by eye. Those are not leaf colours. A real leaf reflects roughly 5-20%
in the green band and 2-5% in red and blue; bark reflects 10-25%; blossom petals
reflect 40-60%. A canopy built from 0.74-albedo "green" is not a tree, it is
highlighter paint, and it is the single clearest signature of a cartoon render.

So the palette is authored as **albedo**, and the lighting does the rest of the
work. Under a physically-lit sun this lands in the right place on screen; under
a cartoon ambient it does not, which is the point.

The one deliberate exception is documented inline where it occurs.
"""

import colorsys
import math
import pathlib
import re

# key: (hue_deg, saturation, value) in HSV, where `value` is the *peak*
# reflectance of the most reflective channel. Values are chosen per species so
# the result lands in a real material's range, not a paint chip's.
FOLIAGE = {
    # Deep blue-green; a tallow's summer foliage is the darkest in the palette,
    # which is what makes its scarlet autumn read as violent.
    "wu-jiu": (108, 0.62, 0.155),
    # Conifer: almost black-green, very low chroma. A fir in shade is nearly
    # silhouette, and the sky-lit rim is the only thing that gives it form.
    "cong-shu": (168, 0.40, 0.105),
    # A yellowed, slightly dusty olive.
    "zi-hua-feng-jiao-mu": (78, 0.46, 0.215),
    # Muted, low-chroma: a plum's foliage is a soft grey-green, not a lawn.
    "xiao-ye-ying-ren": (62, 0.34, 0.200),
    # The brightest green in the palette, but still only 27% reflectance.
    "huang-hua-feng-jiao-mu": (48, 0.40, 0.270),
    # Chinaberry: a flat, slightly dusty mid green with low chroma.
    "ma-lian": (96, 0.44, 0.195),
    # Crape myrtle: a strong, saturated green — the most vivid deciduous.
    "xiao-ye-zi-wei": (114, 0.66, 0.195),
    # Crabapple: cool and soft, with a blue cast that separates it from the
    # plum without being a different hue family.
    "hai-tang": (136, 0.30, 0.180),
    # Pomelo: a very dark, very saturated glossy green.
    "you-zi": (156, 0.72, 0.125),
    # Frangipani: a dark olive, the leaves held on bare grey branches.
    "hong-hua-ji-dan-hua": (70, 0.56, 0.150),
    # Dawn redwood: cool and soft, and it turns before it dies.
    "luo-yu-shan": (146, 0.40, 0.170),
    # Wampee: a pale warm green, among the lightest deciduous foliage.
    "huang-pi": (54, 0.50, 0.245),
    # Magnolia: a soft mid green, low chroma, and it goes bronze rather than red.
    "hong-hua-yu-lan": (124, 0.34, 0.215),
    # Metasequoia: a grey-green. Conifers in a temperate city are grey, not
    # emerald, and this is most of why the last port's evergreens looked fake.
    "shui-shan": (88, 0.20, 0.155),
    # Hibiscus: the brightest foliage here, a yellow-green at 28% reflectance.
    "huang-jin": (44, 0.68, 0.285),
    # The one deliberate non-green: a purple-leaf plum is wine-coloured all
    # summer. It is the reason this species is planted, and it is the single
    # colour that has to be unmistakable.
    "jin-ye-ying-ren": (352, 0.60, 0.190),
}

# Autumn: real autumn foliage is not a hue shift on the summer colour, it is a
# loss of chlorophyll, so it goes *warmer and more saturated* while *darkening*
# or holding value. Scarlet tallow and orange maple are the loud ones; a
# dawn redwood's bronze is quiet.
AUTUMN = {
    "wu-jiu": (8, 0.78, 0.235),          # scarlet — the tallow's whole point
    "cong-shu": None,                    # evergreen
    "zi-hua-feng-jiao-mu": (38, 0.60, 0.180),
    "xiao-ye-ying-ren": (28, 0.66, 0.205),
    "huang-hua-feng-jiao-mu": None,
    "ma-lian": (44, 0.72, 0.290),        # yellow
    "xiao-ye-zi-wei": (14, 0.72, 0.250),  # wine red
    "hai-tang": (32, 0.60, 0.230),
    "you-zi": None,                      # evergreen
    "hong-hua-ji-dan-hua": None,         # evergreen, but semi-deciduous
    "luo-yu-shan": (26, 0.66, 0.235),    # bronze
    "huang-pi": None,                    # evergreen
    "hong-hua-yu-lan": (30, 0.44, 0.165),  # bronze, quiet
    "jin-ye-ying-ren": (352, 0.66, 0.165),
    "huang-jin": (40, 0.74, 0.265),      # yellow
    "shui-shan": (30, 0.62, 0.200),      # bronze
}

# Blossom: petals are the most reflective thing on a tree, and a petal is close
# to white, tinted. This is why blossom reads as light and leaves read as dark,
# and a renderer that gets it backwards produces a tree that glows.
BLOOM = {
    "wu-jiu": None,
    "cong-shu": None,
    "zi-hua-feng-jiao-mu": (278, 0.34, 0.520),  # violet
    "xiao-ye-ying-ren": (340, 0.14, 0.660),      # near-white pink
    "huang-hua-feng-jiao-mu": (48, 0.82, 0.640),  # yellow
    "ma-lian": None,
    "xiao-ye-zi-wei": (330, 0.62, 0.500),       # pink
    "hai-tang": (348, 0.16, 0.680),             # white-pink
    "you-zi": (52, 0.10, 0.720),                # white
    "hong-hua-ji-dan-hua": (6, 0.70, 0.520),     # salmon red
    "luo-yu-shan": None,
    "huang-pi": (56, 0.22, 0.660),              # cream
    "hong-hua-yu-lan": (352, 0.66, 0.480),       # deep red
    "jin-ye-ying-ren": (340, 0.16, 0.600),      # pale pink, sparse
    "huang-jin": (52, 0.86, 0.700),             # bright yellow
    "shui-shan": None,
}

# Bark: dry bark is 10-25% reflectance and strongly desaturated. Trunks are
# darker than the leaves they hold, which is the opposite of what a stylised
# render does.
BARK = {
    "wu-jiu": (38, 0.10, 0.185),
    "cong-shu": (24, 0.22, 0.135),
    "zi-hua-feng-jiao-mu": (36, 0.06, 0.235),
    "xiao-ye-ying-ren": (18, 0.16, 0.150),
    "huang-hua-feng-jiao-mu": (40, 0.05, 0.245),
    "ma-lian": (42, 0.08, 0.215),
    "xiao-ye-zi-wei": (32, 0.10, 0.265),
    "hai-tang": (18, 0.14, 0.180),
    "you-zi": (38, 0.08, 0.205),
    "hong-hua-ji-dan-hua": (34, 0.10, 0.230),
    "luo-yu-shan": (20, 0.30, 0.185),
    "huang-pi": (40, 0.08, 0.200),
    "hong-hua-yu-lan": (36, 0.04, 0.225),
    "jin-ye-ying-ren": (12, 0.18, 0.155),
    "huang-jin": (40, 0.07, 0.215),
    "shui-shan": (22, 0.20, 0.175),
}


def rgb(hue: int, sat: float, val: float) -> str:
    r, g, b = colorsys.hsv_to_rgb(hue / 360.0, sat, val)
    return f"[{r:.3f}, {g:.3f}, {b:.3f}]"


def sep(a: tuple, b: tuple) -> float:
    """Scale-invariant separability: chromaticity plus *relative* luma.

    Absolute distance is the wrong metric for reflectance, because a real
    palette is uniformly dark and two real materials are legitimately close in
    absolute terms while being obviously different hues. What a viewer reads is
    the direction of the colour and where it sits on the dark-to-light axis
    relative to its own brightness, so that is what is measured.
    """
    def chroma(c):
        total = c[0] + c[1] + c[2]
        return (c[0] / total, c[1] / total, c[2] / total)

    ca, cb = chroma(a), chroma(b)
    peak_a, peak_b = max(a), max(b)
    la = (0.2126 * a[0] + 0.7152 * a[1] + 0.0722 * a[2]) / peak_a
    lb = (0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]) / peak_b
    return math.sqrt(
        sum((x - y) ** 2 for x, y in zip(ca, cb)) + (la - lb) ** 2
    )


path = pathlib.Path("crates/city-scene/src/species.rs")
text = path.read_text(encoding="utf-8")

for key in FOLIAGE:
    start = text.index(f'key: "{key}"')
    end = text.index("    },", start)
    block = text[start:end]

    block, count = re.subn(
        r"foliage: \[[^\]]*\],",
        f"foliage: {rgb(*FOLIAGE[key])},",
        block,
        count=1,
    )
    assert count == 1, f"{key}: no foliage"

    autumn = AUTUMN[key]
    if autumn is None:
        block, count = re.subn(
            r"autumn: (?:Some\(\[[^\]]*\]\)|None),", "autumn: None,", block, count=1
        )
        assert count == 1, f"{key}: no autumn"
    else:
        block, count = re.subn(
            r"autumn: (?:None|Some\(\[[^\]]*\]\)),",
            f"autumn: Some({rgb(*autumn)}),",
            block,
            count=1,
        )
        assert count == 1, f"{key}: no autumn"

    bloom = BLOOM[key]
    if bloom is None:
        block, count = re.subn(
            r"bloom: (?:Bloom::of\(\[[^\]]*\], [0-9.]+, [0-9.]+\)|Bloom::NONE),",
            "bloom: Bloom::NONE,",
            block,
            count=1,
        )
        assert count == 1, f"{key}: no bloom"
    else:
        block, count = re.subn(
            r"bloom: (?:Bloom::of\(\[[^\]]*\],|Bloom::NONE,) ?([0-9.]+)?,? ?([0-9.]+)?\),?",
            lambda m: f"bloom: Bloom::of({rgb(*bloom)}, {m.group(1)}, {m.group(2)}),",
            block,
            count=1,
        )
        if count != 1:
            block, count = re.subn(
                r"bloom: Bloom::NONE,",
                lambda m: f"bloom: Bloom::of({rgb(*bloom)}, 0.70, 0.24),",
                block,
                count=1,
            )
        assert count == 1, f"{key}: no bloom"

    hue, sat, _ = BARK[key]
    block, count = re.subn(
        r"bark: bark\(\[[^\]]*\], ",
        f"bark: bark({rgb(*BARK[key])}, ",
        block,
        count=1,
    )
    assert count == 1, f"{key}: no bark"

    text = text[:start] + block + text[end:]

path.write_text(text, encoding="utf-8")

values = [colorsys.hsv_to_rgb(h / 360, s, v) for h, s, v in FOLIAGE.values()]
worst = min(
    (sep(values[i], values[j]), k, key)
    for i, k in enumerate(FOLIAGE)
    for j, key in enumerate(FOLIAGE)
    if i < j
)
print(f"rewrote {len(FOLIAGE)} species")
print(f"foliage peak reflectance {min(max(v) for v in values):.3f}"
      f"..{max(max(v) for v in values):.3f}")
print(f"worst-separable pair: {worst[1]} / {worst[2]} at {worst[0]:.4f}")
