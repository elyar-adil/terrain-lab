import json, math, sys, collections
def pct(xs,p):
    xs=sorted(xs); return xs[min(len(xs)-1,int(p*len(xs)))] if xs else float('nan')
M={'arterial':'primary','collector':'secondary','local':'tertiary','expressway':'trunk'}
for path in sys.argv[1:]:
    d=json.load(open(path))
    P={n['id']:(n['point']['x_km']*1000,n['point']['y_km']*1000) for n in d['nodes']}
    hd={h['sdRoad']:h for h in d['hdRoads']}
    roads=[]
    for s in d['sdRoads']:
        h=hd.get(s['id'])
        if not h or h['layer']!=0 or h['oneway']: continue
        line=[(p['x_km']*1000,p['y_km']*1000) for p in h['centreline']]
        roads.append((s['from'],s['to'],M[s['class']],line))
    inc=collections.defaultdict(list)
    for i,(a,b,c,l) in enumerate(roads):
        inc[a].append((i,True)); inc[b].append((i,False))
    def bearing(i,start):
        a,b,c,l=roads[i]; pts=l if start else l[::-1]
        acc=0; t=pts[1]
        for p,q in zip(pts,pts[1:]):
            acc+=math.dist(p,q); t=q
            if acc>35: break
        return math.degrees(math.atan2(t[1]-pts[0][1],t[0]-pts[0][0]))%360
    arms=collections.Counter(); dev4=[]; small3=[]; tee=0; n3=0
    for n,lst in inc.items():
        if len(lst)<3: continue
        bs=sorted(bearing(i,s) for i,s in lst)
        gaps=[(bs[(k+1)%len(bs)]-b)%360 or 360 for k,b in enumerate(bs)]
        arms[min(len(lst),6)]+=1
        if len(lst)==4: dev4.append(max(abs(g-90) for g in gaps))
        if len(lst)==3:
            n3+=1; small3.append(min(gaps)); tee+= max(gaps)>=160
    # spacing: walk degree-2 chains between junctions
    spacing=collections.defaultdict(list); seen=set()
    deg={n:len(l) for n,l in inc.items()}
    for n,lst in inc.items():
        if deg[n]<3: continue
        for i,s in lst:
            length=0; cur=n; j,st=i,s; cls=roads[i][2]
            while True:
                a,b,c,l=roads[j]; length+=sum(math.dist(p,q) for p,q in zip(l,l[1:]))
                cur=b if st else a
                if deg.get(cur,0)!=2 or length>3000: break
                nxt=[(k,t) for k,t in inc[cur] if k!=j]
                j,st=nxt[0]
            if deg.get(cur,0)>=3:
                key=(min(n,cur),max(n,cur),round(length))
                if key not in seen: seen.add(key); spacing[cls].append(length)
    # ramps
    rl=[]; ang=[]
    for s in d['sdRoads']:
        h=hd.get(s['id'])
        if h and h['oneway']:
            l=[(p['x_km']*1000,p['y_km']*1000) for p in h['centreline']]
            rl.append(sum(math.dist(p,q) for p,q in zip(l,l[1:])))
    print(f"=== ours {path.split('/')[-1]}: junctions {sum(arms.values())} arms {dict(sorted(arms.items()))}")
    print(f"  4-arm worst off 90: p50 {pct(dev4,.5):.0f} p90 {pct(dev4,.9):.0f}; 3-arm T {tee}/{n3} smallest p10 {pct(small3,.1):.0f}")
    for c in ['trunk','primary','secondary','tertiary']:
        v=spacing.get(c,[])
        if v: print(f"  spacing {c}: n={len(v)} p25 {pct(v,.25):.0f} p50 {pct(v,.5):.0f} p75 {pct(v,.75):.0f}")
    print(f"  ramp pieces: n={len(rl)} (ramps are cut into <=60 m pieces)")
