"""Roundabouts and approach widening in a city's OSM data.

python3 -I features.py NAME lat0 lon0 radius_km FILE_OR_DIR...

Roundabouts: every closed ring of junction=roundabout ways inside the radius,
with its size, its arms (bundled, so a divided road counts once), their classes,
whether the ring carries signals, and how far the first pedestrian crossing on
each arm stands from the ring.

Approach widening (进口道展宽): every road that comes into a junction with a
lane count tagged both on its last way and on the way before it, and whether
the lanes grow towards the junction, by how many, and over how long a stretch.
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
        self.tags = {}
        self.ways = {}

    def node(self, n):
        self.nodes[n.id] = (n.location.lat, n.location.lon)
        hw = n.tags.get("highway")
        if hw in ("crossing", "traffic_signals", "give_way", "stop"):
            self.tags[n.id] = hw
        elif n.tags.get("crossing"):
            self.tags[n.id] = "crossing"

    def way(self, w):
        hw = w.tags.get("highway")
        if hw not in RANK and hw not in LINKS:
            return
        if w.tags.get("area") == "yes":
            return
        def num(key):
            v = w.tags.get(key)
            try:
                return int(v) if v is not None else None
            except ValueError:
                return None
        self.ways[w.id] = {
            "hw": hw,
            "nodes": [n.ref for n in w.nodes],
            "oneway": w.tags.get("oneway") in ("yes", "1", "-1") or w.tags.get("junction") in ("roundabout", "circular")
            or hw in ("motorway", "motorway_link"),
            "reverse": w.tags.get("oneway") == "-1",
            "lanes": num("lanes"),
            "fwd": num("lanes:forward"),
            "bwd": num("lanes:backward"),
            "turn": w.tags.get("turn:lanes"),
            "roundabout": w.tags.get("junction") in ("roundabout", "circular"),
        }


def pct(xs, p):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(p * len(xs)))] if xs else float("nan")


def base(hw):
    return LINKS.get(hw, hw)


def main():
    name = sys.argv[1]
    lat0, lon0, radius = float(sys.argv[2]), float(sys.argv[3]), float(sys.argv[4]) * 1000
    paths = []
    for a in sys.argv[5:]:
        paths += sorted(glob.glob(os.path.join(a, "*.osm"))) if os.path.isdir(a) else [a]
    c = Collect()
    for p in paths:
        c.apply_file(p, locations=False)
    nodes, ntags = c.nodes, c.tags
    ways = {k: w for k, w in c.ways.items() if all(n in nodes for n in w["nodes"]) and len(w["nodes"]) >= 2}
    kx, ky = 111320 * math.cos(math.radians(lat0)), 110540

    def xy(n):
        la, lo = nodes[n]
        return ((lo - lon0) * kx, (la - lat0) * ky)

    def dist(a, b):
        (ax, ay), (bx, by) = xy(a), xy(b)
        return math.hypot(ax - bx, ay - by)

    adj = collections.defaultdict(list)
    for wid, w in ways.items():
        for a, b in zip(w["nodes"], w["nodes"][1:]):
            adj[a].append((b, wid))
            adj[b].append((a, wid))

    # ---- roundabouts ----
    ring_ways = [wid for wid, w in ways.items() if w["roundabout"]]
    parent = {w: w for w in ring_ways}

    def root(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    by_node = collections.defaultdict(list)
    for wid in ring_ways:
        for n in ways[wid]["nodes"]:
            by_node[n].append(wid)
    for lst in by_node.values():
        for w in lst[1:]:
            parent[root(w)] = root(lst[0])
    rings = collections.defaultdict(set)
    for wid in ring_ways:
        rings[root(wid)].add(wid)
    out_rings = []
    for members in rings.values():
        rnodes = {n for w in members for n in ways[w]["nodes"]}
        pts = [xy(n) for n in rnodes]
        cx, cy = statistics.fmean(p[0] for p in pts), statistics.fmean(p[1] for p in pts)
        if math.hypot(cx, cy) > radius:
            continue
        rs = [math.hypot(p[0] - cx, p[1] - cy) for p in pts]
        r = statistics.fmean(rs)
        if r < 3:
            continue
        arms = []
        for n in rnodes:
            for o, wid in adj[n]:
                if wid in members or o in rnodes:
                    continue
                w = ways[wid]
                # Walk out along the arm for the first crossing, up to 120 m.
                prev, cur, crossing, signal = n, o, None, None
                while True:
                    d = math.hypot(xy(cur)[0] - cx, xy(cur)[1] - cy) - r
                    t = ntags.get(cur)
                    if t == "crossing" and crossing is None:
                        crossing = d
                    if t == "traffic_signals" and signal is None:
                        signal = d
                    if d > 120:
                        break
                    nxt = [(q, ww) for q, ww in adj[cur] if q != prev]
                    if len(nxt) != 1:
                        break
                    prev, cur = cur, nxt[0][0]
                ox, oy = xy(o)
                bearing = math.degrees(math.atan2(oy - cy, ox - cx)) % 360
                arms.append({"bearing": bearing, "hw": w["hw"], "lanes": w["lanes"], "crossing": crossing,
                             "signal": signal})
        arms.sort(key=lambda a: a["bearing"])
        bundles = []
        for a in arms:
            if bundles and abs((a["bearing"] - bundles[-1][-1]["bearing"] + 180) % 360 - 180) < 35:
                bundles[-1].append(a)
            else:
                bundles.append([a])
        if len(bundles) > 1 and abs((bundles[0][0]["bearing"] - bundles[-1][-1]["bearing"] + 180) % 360 - 180) < 35:
            bundles[0] = bundles.pop() + bundles[0]
        lanes = [ways[w]["lanes"] for w in members if ways[w]["lanes"]]
        out_rings.append({
            "x": cx, "y": cy, "r": r, "rmin": min(rs), "rmax": max(rs),
            "arms": len(bundles),
            "classes": sorted({base(a["hw"]) for b in bundles for a in b}, key=lambda c_: RANK.get(c_, 9)),
            "arm_classes": [min((base(a["hw"]) for a in b), key=lambda c_: RANK.get(c_, 9)) for b in bundles],
            "divided": [len(b) > 1 for b in bundles],
            "crossings": [min((a["crossing"] for a in b if a["crossing"] is not None), default=None) for b in bundles],
            "signals_on_ring": sum(1 for n in rnodes if ntags.get(n) == "traffic_signals"),
            "signals_on_arms": sum(1 for b in bundles if any(a["signal"] is not None and a["signal"] < 40 for a in b)),
            "ring_lanes": lanes,
        })

    # ---- approach widening ----
    # A way's lane count in the direction of travel towards node `end`.
    def lanes_towards(wid, end):
        w = ways[wid]
        if w["oneway"]:
            heading_end = (w["nodes"][-1] == end) != w["reverse"]
            return w["lanes"] if heading_end else None
        if w["nodes"][-1] == end:
            return w["fwd"] if w["fwd"] else (w["lanes"] // 2 if w["lanes"] and w["lanes"] % 2 == 0 else None)
        if w["nodes"][0] == end:
            return w["bwd"] if w["bwd"] else (w["lanes"] // 2 if w["lanes"] and w["lanes"] % 2 == 0 else None)
        return None

    junction = {n for n, lst in adj.items() if len(lst) >= 3 and math.hypot(*xy(n)) < radius}
    approaches = []
    for j in junction:
        for o, wid in adj[j]:
            w = ways[wid]
            if base(w["hw"]) in ("motorway",) or w["roundabout"] or RANK.get(base(w["hw"]), 9) > 4:
                continue
            ns = w["nodes"]
            if ns[0] != j and ns[-1] != j:
                continue  # the junction is mid-way; the way runs through
            near = lanes_towards(wid, j)
            if near is None:
                continue
            # The way before this one, joined at its far end.
            far_end = ns[0] if ns[-1] == j else ns[-1]
            length = sum(dist(a, b) for a, b in zip(ns, ns[1:]))
            if far_end in junction or length > 400:
                continue
            prev = [ww for q, ww in adj[far_end] if ww != wid]
            if len(prev) != 1 or ways[prev[0]]["hw"] != w["hw"]:
                continue
            up = lanes_towards(prev[0], far_end)
            if up is None:
                continue
            approaches.append({"class": base(w["hw"]), "near": near, "up": up, "length": length,
                               "turn": w["turn"], "signal": any(ntags.get(n) == "traffic_signals" for n in ns[-3:] + ns[:3])})

    json.dump({"rings": out_rings, "approaches": approaches}, open(f"{name}_features.json", "w"))
    print(f"=== {name}")
    big = [r for r in out_rings if r["r"] >= 8]
    print(f"  roundabouts r>=8 m: {len(big)} (all rings {len(out_rings)})")
    if big:
        print(f"  ring radius (centreline) p10 {pct([r['r'] for r in big], .1):.0f} p50 {pct([r['r'] for r in big], .5):.0f} p90 {pct([r['r'] for r in big], .9):.0f} m")
        print("  arms:", dict(sorted(collections.Counter(r["arms"] for r in big).items())))
        print("  top class:", dict(collections.Counter(r["classes"][0] if r["classes"] else "-" for r in big)))
        print("  arm classes:", dict(collections.Counter(c_ for r in big for c_ in r["arm_classes"])))
        print(f"  divided arms: {sum(sum(r['divided']) for r in big)}/{sum(r['arms'] for r in big)}")
        cr = [c_ for r in big for c_ in r["crossings"] if c_ is not None]
        print(f"  arms with a mapped crossing within 120 m: {len(cr)}/{sum(r['arms'] for r in big)}; distance from ring p25 {pct(cr, .25):.0f} p50 {pct(cr, .5):.0f} p75 {pct(cr, .75):.0f} m")
        print(f"  rings with signals on the ring: {sum(1 for r in big if r['signals_on_ring'])}; with signals on arms near the ring: {sum(1 for r in big if r['signals_on_arms'])}")
        rl = [l for r in big for l in r["ring_lanes"]]
        print(f"  ring lanes (tagged ways {len(rl)}):", dict(sorted(collections.Counter(rl).items())))
    print(f"  approaches with lanes tagged on both ways: {len(approaches)}")
    for cls in ["trunk", "primary", "secondary", "tertiary"]:
        a = [x for x in approaches if x["class"] == cls]
        if not a:
            continue
        grow = [x for x in a if x["near"] > x["up"]]
        print(f"   {cls}: n={len(a)} widened {len(grow)} ({100*len(grow)/len(a):.0f}%), added lanes {dict(collections.Counter(x['near']-x['up'] for x in grow))}, "
              f"widened length p25 {pct([x['length'] for x in grow], .25):.0f} p50 {pct([x['length'] for x in grow], .5):.0f} p75 {pct([x['length'] for x in grow], .75):.0f} m; "
              f"narrowed {sum(1 for x in a if x['near'] < x['up'])}")
    turns = collections.Counter(x["turn"] for x in approaches if x["turn"] and x["near"] > x["up"])
    print("  commonest turn:lanes on widened approaches:", turns.most_common(8))


main()
