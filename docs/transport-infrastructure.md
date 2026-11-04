# Transport infrastructure: railways and airports

Design study for the rail and air layers of the world planner. It follows
`docs/methodology.md`: every rule names its terrain inputs, cites a standard or
observation, is deterministic, and has a statistic we can measure on the output.

Implementation: `crates/infrastructure/src/transport/` (`rail.rs`, `airport.rs`,
`wind.rs`, `geometry.rs`), `crates/urban/src/model/transport.rs` (shared types,
corridor clearing, airfield layout and obstacle surfaces),
`crates/city-scene/src/rail.rs` (track, catenary, stations, trains, crossings),
`src/components/transport3d.ts` (terrain-view railways and airport).

Status of numbers. Values marked **[checked]** were confirmed against a source
fetched while writing this note; values marked **[std]** are the published
figures of the named standard as recalled from its text and must be re-checked
against the current edition before they are used for anything but this
simulation; values marked **[sim]** are our own modelling choices.

## 1. Sources

| Tag | Source | Used for |
|---|---|---|
| TB10098 | 铁路线路设计规范 TB 10098-2017 (applies to standard-gauge HSR, intercity, Class I/II mixed passenger-freight and heavy-haul lines) — catalogue entry at codeofchina.com/standard/TB10098-2017.html **[checked: scope and status only]** | alignment limits, ruling grade, curve radius by speed |
| TB10621 | 高速铁路设计规范 TB 10621-2014 **[std]** | HSR radius, track spacing, grade, platform length |
| TB10002 | 铁路桥涵设计规范 TB 10002 **[std]** | girder spans, clearances |
| 铁路安全管理条例 | 国务院令第639号 (2013), art. 27 **[std]** | safety protection zone beside the line |
| GB 50091 | 铁路车站及枢纽设计规范 **[std]** | platform dimensions, yard throat, station spacing |
| TB10009 | 铁路电力牵引供电设计规范 **[std]** | 25 kV AC, contact-wire height, mast spacing |
| CJJ 37 | 城市道路工程设计规范 **[std]** | road clearance under structures (4.5 m motor vehicles) |
| HSR-WP | Wikipedia, *High-speed rail in China* (fetched) **[checked]** | design speeds 200–380 km/h, 250 and 350 km/h classes, standard-gauge double track, electrified |
| ICAO A14 | ICAO Annex 14 Vol. I, Aerodrome Design and Operations **[std]** | runway width, strip, OLS, wind coverage |
| MH5001 | 民用机场飞行区技术标准 MH 5001-2013 **[std]** | Chinese codes 3C–4F, runway/taxiway widths, separation |
| MH/T 7003 | 民用机场鸟害防治 / ICAO Doc 9137 Pt 3 **[std]** | bird-attractant (water, wetland) distance from the aerodrome |
| FAA-WIND | FAA AC 150/5300-13 appendix on wind analysis (search result excerpt) **[checked]** | 95 % wind coverage target, 13 kt example with 97.28 % |
| ICAO-XW | ICAO crosswind table as summarised in a conference paper (search result) **[checked, secondary]** | allowable crosswind 10 / 13 kt by reference field length; the same table gives 20 kt (37 km/h) for 1 500 m and over in the current edition, 16 kt in older ones |

Photographs of real stations and airfields were consulted only as images (open
satellite and Wikimedia Commons views) to compare layouts; none is copied into
the repository and no map-service imagery is used.

## 2. Railways

### 2.1 Line classes the planner generates

| Class | Design speed | Where | Min radius (sim uses) | Ruling grade | Track spacing | Electrification |
|---|---|---|---|---|---|---|
| Passenger-dedicated HSR | 250–350 km/h | between regional centre and the largest town | 5 000 m (std recommends 7 000 m at 350 km/h, 4 500 m min at 300) | 20 ‰ (std limit 30 ‰) | 5.0 m [std TB10621] | 25 kV AC |
| Conventional mixed line (Class I) | 120–160 km/h | towns and regional centre | 1 200 m (std: 800 m at 120 km/h, 1 200 m at 160 km/h) | 6 ‰ flat country, 12 ‰ hilly (std ruling grade for Class I) | 4.0 m [std] | 25 kV AC |
| Branch line (Class II) | 100–120 km/h | town to a smaller town | 800 m | 12 ‰ | 4.0 m | 25 kV AC |

Common to all: track gauge 1 435 mm [std; HSR-WP checked standard gauge];
60 kg/m rail; sleepers 2.6 m long at 0.6 m spacing (1 667 per km) on ballast,
or slab track on viaducts; ballast bed 0.35 m deep (0.45 m conventional) with
shoulder slope 1:1.75 and bed width 3.4 m (HSR 3.6 m); earthwork side slope
1:1.5 [std]. Vertical curves of radius 10 000 m (HSR 15 000 m) wherever the
grade changes by more than 3 ‰ [std]. Contact wire 5.3 m above rail (HSR),
5.7–6.2 m (conventional, container clearance); masts every 50 m or less
[std TB10009]; mast offset 3.1–3.5 m from track centre.

The `Railway` record stores the numbers actually used, and the tests check the
generated line against the limits of its own class, not against a fixed global
number.

### 2.2 Why lines are smoother than roads

A road route in this repository is a cost-weighted shortest path on the
travel-cost grid with a turning penalty. A railway differs in three ways, all
of which become code:

1. **Hard curvature floor.** The plan corner is filleted by a circular arc of at
   least `R_min`; where two corners are too close for that, the corner is
   removed (the line takes the chord) until the fillet fits. Result: no
   segment of the finished centreline has radius below `R_min` [sim].
   Transition spirals are not modelled yet (recorded in the issue list).
2. **Hard gradient ceiling on the design profile, not on the ground.** Real
   alignments do not follow the ground profile; they follow a grade line that
   is cut into hills and raised on fill, with viaducts and tunnels where the
   difference is too large. We route on the ground (cost rises with the ground
   gradient along the line and with cross-slope) and then fit a design grade
   line that satisfies the ruling grade by construction. Cut/fill is the
   difference.
3. **Structures from the difference, not from decoration.**
   - Embankment where the design is above ground by up to 9 m; viaduct when
     the fill would exceed 9 m, over rivers (river order ≥ 2 or open water),
     over wetland and floodplain (soft ground: HSR practice is to bridge it),
     and through the built-up area of the regional centre (grade separation).
   - Cutting where the design is below ground by up to 14 m; tunnel when the
     cut would exceed that over more than 300 m [sim thresholds].
   - Typical simply supported box girder span 32.6 m (24 m, 32 m
     standard spans); piers at spans, so viaduct geometry is a function of
     chainage.

### 2.3 Siting and routing inputs

| Input (terrain-core / infrastructure) | Effect |
|---|---|
| `height` along the line | design grade line, cut/fill, structures |
| local slope (`slope_ratio`) and cross-slope | route cost; cross-slope above 25 % is side-hill cut and is penalised |
| `water`, `river_order`, `lake` | bridge or viaduct; no embankment across a lake; ocean cells forbidden with a buffer |
| `wetland`, `floodplain` | soft ground: cost up, viaduct when the line cannot avoid it |
| `urban_land` | the line may not use the built-up area except at a station; avoidance cost |
| existing roads | a rail crossing a road is a recorded crossing (below); running along an existing road corridor is not rewarded (a railway needs its own formation) |
| settlement class | which places the net connects (below) |

Network topology. Passenger and freight lines connect the regional centre and
towns (Class `Town` and above), not villages: a minimum spanning tree on
straight-line distance, as for roads, plus a through trunk that leaves the map
edge on the far side of the regional centre (a railway does not end in a field;
it is a piece of a larger network). Villages are reached by road only.

### 2.4 Stations

Placement by city size [sim, calibrated on Chinese practice]: a county-level
regional centre gets one passenger station on its edge, at the point of the
line nearest the centre but not closer than 0.5 km (the old-town station
position) and not farther than 1.6 km; a town gets a stop at 0.4–1.0 km; villages
none. HSR stations are typically 5–10 km from the old centre in new
developments; the planner keeps the 1.6 km maximum so the station falls inside
the generated town window and can be built at street level.

Station layout (GB 50091 / TB10621):

- Platforms: 450 m effective length for HSR (16 cars of 25 m plus margin), 650 m
  on conventional main lines (and 850 m for freight-capable yards).
- Platform height 1.25 m (HSR), 1.10 m (conventional high platform);
  island platform width 12 m, side platform 8 m; platform edge 1.75 m from
  track centre; canopy over the whole platform length.
- Track count: 2 through + 2 loops (4 tracks, 2 island platforms) at a
  regional-centre station; 2 through tracks and 2 side platforms at a town stop.
- Yard throat: ladder of No. 12 turnouts (No. 18 for main tracks) at both
  ends; turnout length about 37 m (No. 12), 70 m (No. 18). Throat length is
  therefore proportional to the number of tracks.
- Station building on the side of the town, at right angles to the tracks or
  parallel (side-flank type), with a forecourt plaza that the road net can
  reach.
- Freight yard / depot: only at the regional centre, on the side away from the
  centre, four to six parallel sidings (the planner draws the yard footprint
  and tracks, not wagons).

### 2.5 Crossings with roads

- No level crossings on HSR (fully fenced). On a conventional line, level
  crossings are allowed only with roads of the collector class or lower; every
  arterial or motorway crossing is grade separated [std practice; 铁路安全管理条例 also
  restricts new level crossings].
- Grade separation is a rail-over-road structure: the road keeps its level and
  its clearance (4.5 m for urban motor roads, CJJ 37; 5.0 m used here, which
  includes the structure allowance), so the rail design grade at the crossing
  is at least `road + 5.0 m + 2.6 m` girder depth.
- Inside a town the line is either elevated on a continuous viaduct (regional
  centre: every street passes under) or at grade with fences and protected
  level crossings (towns): barriers, bells and warning lights, crossing panel
  across the rails.
- The safety protection zone beside the line (铁路安全管理条例 art. 27)
  measured from the toe of the embankment / top of cutting / outside of the
  bridge: urban 8 m (HSR 10 m); suburban residential 10 m (HSR 12 m); village
  residential 12 m (HSR 15 m); elsewhere 15 m (HSR 20 m). The planner clears
  buildings and parcels inside this zone.

### 2.6 Validation statistics (railways)

1. Max design grade along every line ≤ class limit (equals, within 1e-4, by
   construction; the test checks the stored profile).
2. Min radius along every line ≥ class `R_min` (computed from the stored
   centreline by three-point circles at 100 m spacing).
3. Fraction of line length on structure, by kind (a real HSR on plains is 40–
   80 % viaduct; hill lines are 10–30 % bridge and tunnel): reported, not
   asserted, because it depends on terrain.
4. Every road crossing is either grade separated or protected, and none is
   level on HSR.
5. Line length / straight distance (circuity), expected 1.05–1.25 for lines
   built to the limits above.
6. Determinism: same seed gives byte-identical serialised lines.
7. No station closer to a road than the station plaza, no platform shorter than
   its class.

## 3. Airports

### 3.1 Aerodrome class the planner generates

One airport per world at most, selected only if a site exists. Class is chosen
from the settlement it serves (county-level regional centre → code 4C regional
airport; the planner has no larger city, a larger world would raise it to 4D/4E):

| Code | Runway length | Width | Shoulders | Taxiway width | Strip width |
|---|---|---|---|---|---|
| 3C | 1 800 m | 30 m | — | 15 m | 150 m |
| 4C | 2 800 m (2 200–3 200 typical) | 45 m | to 60 m overall | 15 m | 300 m (ICAO 280) |
| 4D | 3 200 m | 45 m | to 60 m | 23 m | 300 m |
| 4E | 3 400–3 600 m | 45 m (60 m for 4F) | to 75 m | 23 m | 300 m |

(MH5001 / ICAO Annex 14 Table 3-1; **[std]**.) Strip extends 60 m beyond each
runway end. Runway longitudinal slope ≤ 1 %, transverse 1.0–1.5 % crown [std].
Parallel taxiway centreline separation from an instrument runway of code 3–4:
168 m (code 3: 152 m) [std, Annex 14 Table 3-1], widened to 190 m in China
for 4E. Exit taxiways at 30° high-speed exits every 600–900 m past the touchdown
zone. Apron stands for code C: 45 m by 45 m clear, nose-in with a service road
behind.

### 3.2 Runway markings (ICAO A14 §5.2, MH5001) **[std]**

- Designator: magnetic bearing / 10, two digits, plus L/C/R for parallel runways.
- Threshold stripes: 12 stripes for 45 m runway (16 for 60 m), 30 m long.
- Centreline: 30 m dashes with 20 m gaps, width 0.9 m (instrument 0.45–0.9).
- Aiming point: two bars 400 m from the threshold on runways of 2 400 m and
  over (300 m for 1 800–2 399 m), 45–60 m long and 10 m wide.
- Touchdown-zone marking: pairs of bars every 150 m for 900 m (three, two, one pairs).
- Edge lines at the pavement edge, continuous.
- Approach lighting for a precision runway Category I: 900 m centreline with
  bars every 30 m and a crossbar at 300 m.

### 3.3 Orientation from wind

Runway heading is chosen so that the usability factor (the share of the time
for which the crosswind component is below the permitted value) is at least
95 % [ICAO A14 Att. A, FAA-WIND checked]. Permitted crosswind for reference
field length ≥ 1 500 m: 20 kt (37 km/h) in the current edition (older 16 kt;
13 kt for 1 200–1 500 m; 10 kt below 1 200 m).

The simulation has a `wind_direction` (degrees) and `wind_speed` (m/s) in its
configuration. They define the *prevailing* wind; we build a deterministic wind
rose from them: a von Mises distribution of direction around the prevailing
direction with concentration κ = 0.8 + 0.25 · speed(m/s), and a Weibull speed
distribution (k = 2) whose mean is half the configured flow speed (surface wind
at aerodrome height is weaker than the flow aloft the terrain model uses) [sim].
In a light-wind climate nearly every heading reaches 95 % at the 20 kt limit, so
candidate headings are ranked on the stricter 13 kt limit (reference field length
under 1 500 m, or poor braking); both figures are reported. The
usability of a candidate heading is the integral of that rose over the
(direction, speed) pairs whose crosswind is below the limit; a runway may be
used in either direction, so headings are modulo 180°. Annual climate
statistics from a real station would replace this rose without changing the
code.

### 3.4 Siting inputs and rules

| Variable | Rule |
|---|---|
| `height` footprint under the airfield | runway strip + 60 m ends + 300 m width: mean slope ≤ 1.5 %, relief (max−min) ≤ 8 m so grading is a bounded earthwork, not a mountain |
| `water`, `lake`, `river_order`, `wetland`, `floodplain` | none inside the footprint; and the footprint plus a buffer is kept 2 km from them: standing water and wetland are bird attractants (MH/T 7003 recommends 8 km for large water; we use the simulation scale and the buffer is a lower bound), and floodplain land floods |
| ocean | none |
| `urban_land` / settlement positions | no overlap with any settlement window; 3 km minimum from any town centre (noise, and the OLS keeps buildings low) |
| distance from the regional centre | 10–40 km, score peaks at 22 km (Chinese airports are 10–40 km from the city centre: Beijing Capital 25 km, Shanghai Pudong 30 km, Chongqing Jiangbei 20 km, a regional airport is closer) |
| wind | candidate headings scored by usability; best must reach 95 % |
| roads | the expressway access runs from the airport to the regional centre along the least-cost path; its class is Motorway |
| terrain beyond the strip | the approach surfaces must clear the terrain: terrain rising above 1:50 from the threshold along the approach axis for 3 000 m is a rejection |

Runway designators: the number painted at a threshold is the compass bearing
of the landing direction over ten, so the end an aircraft lands toward +x carries
the designator of that bearing and the far end the reciprocal (18 apart).

The airport is placed by maximising `score = flatness · distance_term ·
wind_term · (1 − hazard)` over a coarse grid of positions and headings; ties
break on position index, so the result is a pure function of the terrain and
configuration. If no site passes, the world has no airport and says so.

### 3.5 Obstacle limitation surfaces (净空) **[std Annex 14 Ch. 4, Table 4-1]**

For a code 3/4 instrument runway (precision approach Category I):

| Surface | Parameters |
|---|---|
| Approach | inner edge 280 m wide at 60 m before the threshold; divergence 15 % each side; first section 3 000 m at 2 % (1:50); second section 3 600 m at 2.5 %; then horizontal section to 15 000 m total |
| Inner horizontal | 45 m above the aerodrome elevation; radius 4 000 m round the strip |
| Conical | slope 5 % (1:20) outward from the inner-horizontal edge, to 100 m above it |
| Transitional | slope 14.3 % (1:7) from the strip edge up to the inner horizontal |
| Take-off climb | 12 000 m long, 2 % slope, divergence 12.5 % |

This is the source of the "1:50 approach" and "1:20" figures: the 1:20 slope is
the conical surface, not the inner horizontal (which is flat at 45 m). The
planner exposes `surface_height_limit(x, y)`: the lowest limit of any of
these surfaces over a point, relative to the aerodrome elevation. City
generation uses it to cap building heights near an airport; the number of
buildings it lowers is reported.

### 3.6 Layout

From the runway outward: runway, parallel taxiway at the separation above, one
or two rapid exits, apron on the terminal side with stands along the terminal
frontage (code C stand pitch 45 m), terminal building with its landside
forecourt and car park, control tower beside the apron near the runway midpoint,
hangars and maintenance apron at one end, fuel farm and fire station at
the other, perimeter fence around everything, perimeter road, and the
access expressway to the terminal forecourt. Navigation: approach lighting on
the extended centreline of both ends, PAPI at each touchdown zone, localizer
antenna beyond the far end.

### 3.7 Validation statistics (airports)

1. Runway length and width are the table value of the chosen code.
2. Wind usability of the chosen heading ≥ 95 % and ≥ the usability of the
   perpendicular heading.
3. Footprint slope and relief within the limits above; no water or wetland in
   the footprint.
4. Distance to the regional centre in 10–40 km.
5. Parallel taxiway separation = table value.
6. No building of a city generated near the airport exceeds
   `surface_height_limit`.
7. Determinism.

## 4. Limits recorded

Not modelled: transition spirals and cant; real timetable and train
movements (trains are placed as static consists); signals as interlocked
systems (signals are placed, not logic-driven); metro, subway stations, ports,
HSR stations in new towns away from the old one, and freight spurs to
industry; rotating beacons, ILS critical areas; terminal interior.
