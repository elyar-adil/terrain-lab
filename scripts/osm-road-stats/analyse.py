"""Road-geometry statistics for a city from OSM data.

python3 -I analyse.py NAME lat0 lon0 radius_km FILE_OR_DIR...

Writes NAME.json (raw measurements) and prints a summary.
"""
import collections
import glob
import json
import math
import os
import statistics
import sys

import osmium

CLASSES = ["motorway", "trunk", "primary", "secondary", "tertiary", "unclassified", "residential"]
RANK = {c: i for i, c in enumerate(CLASSES)}
LINKS = {c + "_link": c for c in ["motorway", "trunk", "primary", "secondary", "tertiary"]}


class Collect(osmium.SimpleHandler):
    def __init__(self):
        super().__init__()
        self.nodes = {}
        self.ways = {}

    def node(self, n):
        self.nodes[n.id] = (n.location.lat, n.location.lon)

    def way(self, w):
        hw = w.tags.get("highway")
        if hw not in RANK and hw not in LINKS:
            return
        if w.tags.get("area") == "yes":
            return
        self.ways[w.id] = {
            "hw": hw,
            "nodes": [n.ref for n in w.nodes],
            "oneway": w.tags.get("oneway") in ("yes", "1", "-1") or w.tags.get("junction") == "roundabout"
            or hw in ("motorway", "motorway_link"),
            "lanes": w.tags.get("lanes"),
            "bridge": w.tags.get("bridge") not in (None, "no"),
            "layer": w.tags.get("layer"),
            "tunnel": w.tags.get("tunnel") not in (None, "no"),
            "name": w.tags.get("name"),
            "roundabout": w.tags.get("junction") in ("roundabout", "circular"),
            "turn_lanes": w.tags.get("turn:lanes"),
        }


def load(paths):
    c = Collect()
    for p in paths:
        c.apply_file(p, locations=False)
    return c.nodes, c.ways


def base(hw):
    return LINKS.get(hw, hw)


def main():
    name = sys.argv[1]
    lat0, lon0, radius = float(sys.argv[2]), float(sys.argv[3]), float(sys.argv[4]) * 1000
    paths = []
    for a in sys.argv[5:]:
        paths += sorted(glob.glob(os.path.join(a, "*.osm"))) if os.path.isdir(a) else [a]
    nodes, ways = load(paths)
    kx = 111320 * math.cos(math.radians(lat0))
    ky = 110540

    def xy(nid):
        la, lo = nodes[nid]
        return ((lo - lon0) * kx, (la - lat0) * ky)

    ways = {k: w for k, w in ways.items() if all(n in nodes for n in w["nodes"]) and len(w["nodes"]) >= 2}
    # Segment graph.
    adj = collections.defaultdict(list)  # node -> [(other, way)]
    for wid, w in ways.items():
        ns = w["nodes"]
        for a, b in zip(ns, ns[1:]):
            adj[a].append((b, wid))
            adj[b].append((a, wid))
    inside = lambda nid: math.hypot(*xy(nid)) < radius
    junction = {n for n, lst in adj.items() if len(lst) >= 3 and inside(n)}

    def dist(a, b):
        (ax, ay), (bx, by) = xy(a), xy(b)
        return math.hypot(ax - bx, ay - by)

    # Cluster junction nodes joined by a short road chain (a divided road's two
    # carriageways, the inner box of a big junction).
    parent = {n: n for n in junction}

    def root(n):
        while parent[n] != n:
            parent[n] = parent[parent[n]]
            n = parent[n]
        return n

    def walk(start, nxt, wid, limit):
        """Follow the chain from start through nxt until a junction; return (end, length, way, path)."""
        prev, cur, length, path = start, nxt, dist(start, nxt), [start, nxt]
        while cur not in junction and length < limit:
            options = [(o, w) for o, w in adj[cur] if o != prev]
            if len(options) != 1:
                break
            prev, (cur, wid) = cur, options[0]
            length += dist(prev, cur)
            path.append(cur)
        return cur, length, wid, path

    for n in junction:
        for o, wid in adj[n]:
            if base(ways[wid]["hw"]) in ("motorway",):
                continue
            end, length, _, _ = walk(n, o, wid, 60)
            if end in junction and end != n:
                cls = base(ways[wid]["hw"])
                limit = 45 if RANK.get(cls, 6) <= 2 else 30
                if length <= limit:
                    parent[root(n)] = root(end)
    clusters = collections.defaultdict(set)
    for n in junction:
        clusters[root(n)].add(n)

    records = []
    for members in clusters.values():
        cx = statistics.fmean(xy(n)[0] for n in members)
        cy = statistics.fmean(xy(n)[1] for n in members)
        arms = []
        seen_paths = set()
        for n in members:
            for o, wid in adj[n]:
                if o in members:
                    continue
                end, length, _, path = walk(n, o, wid, 1500)
                if end in members:
                    continue  # an internal loop
                key = (n, o)
                if key in seen_paths:
                    continue
                seen_paths.add(key)
                # Bearing of the arm 25-40 m out from the cluster centre.
                target = None
                acc = 0.0
                for p, q in zip(path, path[1:]):
                    acc += dist(p, q)
                    target = q
                    if acc > 35:
                        break
                tx, ty = xy(target)
                bearing = math.degrees(math.atan2(ty - cy, tx - cx)) % 360
                w = ways[wid]
                arms.append({"bearing": bearing, "hw": w["hw"], "oneway": w["oneway"],
                             "name": w["name"], "lanes": w["lanes"], "length": length, "way": wid,
                             "to": root(end) if end in junction else None})
        # Bundle arms leaving within 22 degrees of each other: the two
        # carriageways of one road, or a main road and the side road (辅路)
        # running beside it.  A bundle is one arm of the junction.
        main = [a for a in arms if a["hw"] not in LINKS]
        links = [a for a in arms if a["hw"] in LINKS]
        main.sort(key=lambda a: a["bearing"])
        bundles = []
        for a in main:
            if bundles:
                m = bundles[-1]
                d = abs((a["bearing"] - m[-1]["bearing"] + 180) % 360 - 180)
                if d < 22:
                    m.append(a)
                    continue
            bundles.append([a])
        if len(bundles) > 1:
            d = abs((bundles[0][0]["bearing"] - bundles[-1][-1]["bearing"] + 180) % 360 - 180)
            if d < 22:
                bundles[0] = bundles.pop() + bundles[0]
        merged = []
        for b in bundles:
            best = min(b, key=lambda a: RANK.get(base(a["hw"]), 9))
            xs = sum(math.cos(math.radians(a["bearing"])) for a in b)
            ys = sum(math.sin(math.radians(a["bearing"])) for a in b)
            classes = {base(a["hw"]) for a in b}
            merged.append({**best, "bearing": math.degrees(math.atan2(ys, xs)) % 360,
                           "pair": len(b) > 1, "mixed": len(classes) > 1, "size": len(b)})
        bearings = sorted(a["bearing"] for a in merged)
        gaps = [((bearings[(i + 1) % len(bearings)] - b) % 360) or 360 for i, b in enumerate(bearings)] if len(bearings) > 1 else []
        span = max(math.hypot(xy(a)[0] - xy(b)[0], xy(a)[1] - xy(b)[1]) for a in members for b in members)
        records.append({
            "id": root(next(iter(members))),
            "next": [(a["to"], base(a["hw"]), a["length"]) for a in arms if a["to"] is not None and a["hw"] not in LINKS],
            "x": cx, "y": cy, "members": len(members), "span": span,
            "arms": len(merged),
            "classes": sorted((base(a["hw"]) for a in merged), key=lambda c: RANK.get(c, 9)),
            "divided": [bool(a.get("pair")) for a in merged],
            "mixed": [bool(a.get("mixed")) for a in merged],
            "gaps": gaps,
            "links": len(links),
            "link_classes": [a["hw"] for a in links],
            "roundabout": any(ways[w]["roundabout"] for n in members for _, w in adj[n]),
        })

    # Junctions under a viaduct: an elevated expressway or ramp segment passes
    # within 20 m of the junction's centre.
    elevated = []
    for w in ways.values():
        if w["bridge"] and base(w["hw"]) in ("motorway", "trunk"):
            pts = [xy(n) for n in w["nodes"]]
            elevated += list(zip(pts, pts[1:]))
    def seg_dist(p, a, b):
        dx, dy = b[0] - a[0], b[1] - a[1]
        l2 = dx * dx + dy * dy or 1e-9
        t = max(0.0, min(1.0, ((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / l2))
        return math.hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy)
    for r in records:
        r["under_viaduct"] = any(seg_dist((r["x"], r["y"]), a, b) < 20 for a, b in elevated)

    # Links (ramps and slip roads): length, end angles against the road they meet.
    link_records = []
    for wid, w in ways.items():
        if w["hw"] not in LINKS:
            continue
        ns = w["nodes"]
        if not inside(ns[0]):
            continue
        length = sum(dist(a, b) for a, b in zip(ns, ns[1:]))
        ends = []
        for end, nxt in ((ns[0], ns[1]), (ns[-1], ns[-2])):
            # Direction of the link leaving this end, and of the other roads there.
            ex, ey = xy(end)
            lx, ly = xy(nxt)
            lb = math.degrees(math.atan2(ly - ey, lx - ex))
            others = []
            for o, ow in adj[end]:
                if ow == wid:
                    continue
                ox, oy = xy(o)
                ob = math.degrees(math.atan2(oy - ey, ox - ex))
                d = abs((ob - lb + 180) % 360 - 180)
                others.append((d, ways[ow]["hw"], ways[ow]["bridge"]))
            ends.append({"meets": [o[1] for o in others],
                         "angle": min((o[0] for o in others), default=None)})
        link_records.append({"hw": w["hw"], "length": length, "bridge": w["bridge"],
                             "lanes": w["lanes"], "oneway": w["oneway"], "ends": ends})

    # Lanes by class.
    lanes = collections.defaultdict(list)
    for w in ways.values():
        if w["lanes"] and w["lanes"].isdigit() and inside(w["nodes"][0]):
            lanes[(w["hw"], w["oneway"])].append(int(w["lanes"]))

    json.dump({"junctions": records, "links": link_records,
               "lanes": {f"{k[0]}|{'oneway' if k[1] else 'twoway'}": v for k, v in lanes.items()}},
              open(f"{name}.json", "w"))
    summary(name, records, link_records, lanes)
    if os.environ.get("SAMPLES"):
        draw_samples(name, records, ways, xy)


def draw_samples(name, records, ways, xy):
    """A contact sheet of random junctions per category, with their labels."""
    import random
    from PIL import Image, ImageDraw
    random.seed(1)
    width = {"motorway": 7, "trunk": 6, "primary": 5, "secondary": 4, "tertiary": 3}
    segs = []
    for w in ways.values():
        pts = [xy(n) for n in w["nodes"]]
        segs.append((pts, w["hw"]))
    cats = {
        "4-arm": [r for r in records if r["arms"] == 4 and RANK.get(r["classes"][0], 9) <= 4],
        "3-arm": [r for r in records if r["arms"] == 3 and RANK.get(r["classes"][0], 9) <= 4],
        "split": [r for r in records if r["arms"] == 2 and any(r["mixed"])],
        "5+": [r for r in records if r["arms"] >= 5],
    }
    tile, half = 220, 90.0
    sheet = Image.new("RGB", (tile * 6, tile * len(cats)), "white")
    for row, (cat, rs) in enumerate(cats.items()):
        for col, r in enumerate(random.sample(rs, min(6, len(rs)))):
            im = Image.new("RGB", (tile, tile), (235, 235, 230))
            d = ImageDraw.Draw(im)
            px = lambda p: ((p[0] - r["x"] + half) / (2 * half) * tile, (r["y"] - p[1] + half) / (2 * half) * tile)
            for pts, hw in segs:
                if all(abs(p[0] - r["x"]) > half * 1.5 or abs(p[1] - r["y"]) > half * 1.5 for p in pts):
                    continue
                col_ = (200, 60, 60) if hw.endswith("_link") else (60, 60, 60)
                d.line([px(p) for p in pts], fill=col_, width=width.get(base(hw), 2))
            d.ellipse([tile / 2 - 3, tile / 2 - 3, tile / 2 + 3, tile / 2 + 3], outline=(0, 120, 255), width=2)
            label = f"{cat} {','.join(c[:3] for c in r['classes'])} " + ",".join(str(round(g)) for g in r["gaps"])
            d.text((3, 3), label, fill=(0, 0, 200))
            sheet.paste(im, (col * tile, row * tile))
    sheet.save(f"{name}_samples.png")
    print("wrote", f"{name}_samples.png")


def pct(xs, p):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(p * len(xs)))] if xs else float("nan")


def summary(name, records, link_records, lanes):
    print(f"=== {name}: {len(records)} junction clusters")
    major = [r for r in records if r["classes"] and RANK.get(r["classes"][0], 9) <= 4]
    print(f"  with a tertiary or bigger road: {len(major)}")
    splits = [r for r in major if r["arms"] == 2 and any(r["mixed"])]
    print(f"  main/side-road split or merge points (two bundles, a side road in one): {len(splits)}")
    major = [r for r in major if r["arms"] >= 3]
    arms = collections.Counter(min(r["arms"], 6) for r in major)
    print("  arms:", dict(sorted(arms.items())))
    print("  roundabouts:", sum(r["roundabout"] for r in major))
    # Angles.
    four = [r for r in major if r["arms"] == 4]
    dev4 = [max(abs(g - 90) for g in r["gaps"]) for r in four]
    three = [r for r in major if r["arms"] == 3]
    straight = [max(r["gaps"]) for r in three]
    print(f"  4-arm: worst corner off 90°: p25 {pct(dev4,.25):.0f} p50 {pct(dev4,.5):.0f} p75 {pct(dev4,.75):.0f} p90 {pct(dev4,.9):.0f}")
    smallest4 = [min(r["gaps"]) for r in four]
    print(f"  4-arm: smallest corner: p10 {pct(smallest4,.1):.0f} p50 {pct(smallest4,.5):.0f}")
    tee = sum(1 for s in straight if s >= 160)
    print(f"  3-arm: T (one side >=160° open) {tee}/{len(three)}; smallest corner p10 {pct([min(r['gaps']) for r in three],.1):.0f} p50 {pct([min(r['gaps']) for r in three],.5):.0f}")
    print(f"  junctions under a viaduct: {sum(r['under_viaduct'] for r in major)} of {len(major)}")
    by_id = {r["id"]: r for r in records}
    spacing = collections.defaultdict(list)
    seen = set()
    for r in records:
        if r["arms"] < 3:
            continue
        for to, cls, length in r["next"]:
            o = by_id.get(to)
            if o is None or o["arms"] < 3 or to == r["id"]:
                continue
            key = (min(r["id"], to), max(r["id"], to), cls)
            if key in seen:
                continue
            seen.add(key)
            spacing[cls].append(length)
    for cls in CLASSES[:5]:
        v = spacing.get(cls, [])
        if len(v) >= 10:
            print(f"  spacing along {cls} between junctions: n={len(v)} p25 {pct(v,.25):.0f} p50 {pct(v,.5):.0f} p75 {pct(v,.75):.0f} m")
    spans = [r["span"] for r in major if r["members"] > 1]
    print(f"  cluster span (divided-road boxes): p50 {pct(spans,.5):.0f} m p90 {pct(spans,.9):.0f} m")
    # Slip roads at at-grade junctions.
    slip = [r for r in major if r["links"] and not any(l.startswith(("motorway", "trunk")) for l in r["link_classes"])]
    print(f"  junctions with slip roads (non-motorway links): {len(slip)} of {len(major)}")
    # Class pairs.
    pairs = collections.Counter(f"{r['classes'][0]}x{r['classes'][1] if len(r['classes'])>1 else '-'}" for r in major)
    print("  top class pairs:", pairs.most_common(8))
    # Links.
    for kind in ["motorway_link", "trunk_link", "primary_link", "secondary_link", "tertiary_link"]:
        ls = [l for l in link_records if l["hw"] == kind]
        if not ls:
            continue
        lens = [l["length"] for l in ls]
        ang = [e["angle"] for l in ls for e in l["ends"] if e["angle"] is not None and any(m in ("motorway", "trunk") for m in e["meets"])]
        print(f"  {kind}: n={len(ls)} length p25 {pct(lens,.25):.0f} p50 {pct(lens,.5):.0f} p75 {pct(lens,.75):.0f} m; bridge {sum(l['bridge'] for l in ls)}; angle to motorway/trunk at its end p25 {pct(ang,.25):.0f}° p50 {pct(ang,.5):.0f}° p75 {pct(ang,.75):.0f}°")
    for k in sorted(lanes, key=lambda k: RANK.get(k[0], 9)):
        v = lanes[k]
        if len(v) >= 10:
            print(f"  lanes {k[0]} {'oneway' if k[1] else 'twoway'}: n={len(v)} {dict(sorted(collections.Counter(v).items()))}")


main()
