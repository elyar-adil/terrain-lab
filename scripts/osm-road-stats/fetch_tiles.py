"""Fetch the OSM data of a bounding box from the main API in small tiles,
splitting a tile that is refused as too dense, one request at a time.

python3 -I fetch_tiles.py NAME south west north east OUTDIR
"""
import os
import sys
import time
import urllib.request
import urllib.error

API = "https://api.openstreetmap.org/api/0.6/map?bbox={w},{s},{e},{n}"
UA = "terrain-lab-research/0.1 (one-off road geometry statistics)"


def fetch(s, w, n, e, out, depth=0):
    path = os.path.join(out, f"{s:.4f}_{w:.4f}_{n:.4f}_{e:.4f}.osm")
    if os.path.exists(path) and os.path.getsize(path) > 0:
        return
    url = API.format(s=s, w=w, n=n, e=e)
    for attempt in range(4):
        try:
            req = urllib.request.Request(url, headers={"User-Agent": UA})
            with urllib.request.urlopen(req, timeout=180) as r:
                data = r.read()
            with open(path, "wb") as f:
                f.write(data)
            print(f"ok {path} {len(data)//1024} KB", flush=True)
            time.sleep(1.0)
            return
        except urllib.error.HTTPError as err:
            if err.code == 400 and depth < 6:
                # Too many nodes: split in four.
                ms, mw = (s + n) / 2, (w + e) / 2
                for box in [(s, w, ms, mw), (s, mw, ms, e), (ms, w, n, mw), (ms, mw, n, e)]:
                    fetch(*box, out, depth + 1)
                return
            print(f"http {err.code} {url}", flush=True)
            time.sleep(5 * (attempt + 1))
        except Exception as err:  # network
            print(f"err {err} {url}", flush=True)
            time.sleep(5 * (attempt + 1))


def main():
    name, s, w, n, e, out = sys.argv[1], *map(float, sys.argv[2:6]), sys.argv[6]
    out = os.path.join(out, name)
    os.makedirs(out, exist_ok=True)
    step = 0.02
    lat = s
    while lat < n - 1e-9:
        lon = w
        while lon < e - 1e-9:
            fetch(lat, lon, min(lat + step, n), min(lon + step, e), out)
            lon += step
        lat += step
    print("done", name, flush=True)


main()
