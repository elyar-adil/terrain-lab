# Building design: from the unit plan to the facade

Status: design study for the Interiors rung of `docs/methodology.md`. The
prototype that implements the first half of it is
`crates/city-scene/src/buildings/plan/`.

## 1. The principle

Real buildings are not drawn from the outside in. An architect fixes the
**program** (how many dwellings, of what size, for whom, in what climate and
decade), draws the **typical floor plan** (core, units, rooms), and the
elevation is the consequence: the window in a facade is a room's window, the
balcony is the living room's balcony, the small high window is the bathroom.

So the generator is plan-first, and exterior and interior are two read-outs of
one data structure:

```
region, latitude, era, use, plot ->  typology
typology + plot + storeys        ->  floor plate (W x D, notches)
floor plate                      ->  core (stairs, lifts, lobby)
floor plate - core               ->  units (or lettable floor)
unit                             ->  rooms (function, rectangle)
rooms                            ->  doors (connectivity) and openings
openings                         ->  facade per side; massing = plate x storeys
```

Everything downstream of the typology is a pure function of
`(seed, typology, era, region)`; variation is seeded sampling of design rules,
never a lookup table of finished plans (methodology principles 1 and 2).

## 2. How far the references were checked

Search from this environment returned few primary texts. The status of every
number below is marked so that nothing is presented as checked when it was not.

* **[V] read this session.** Clause numbers and values of GB 50096-2011 were read
  from two reproductions of the standard's text:
  <https://www.yuanlin8.com/guifan/jianzhu/5da6a833.html> and the mandatory-clause
  extract <https://www.hbtgsj.com/index.php/newYJ/71.html>. They disagree in no
  value; both say their tables are partly truncated, so tables (door openings,
  clause 5.8.7) are not used.
* **[V-news] read, partial.** Fire compartment areas of GB 50016 Table 5.3.1 and
  the 0.80 / 1.10 m residential widths are from the draft-amendment news text
  <https://www.sohu.com/a/360231258_222758> (it does not quote the residential
  distance table).
* **[V-study] read as abstracts.** Office space efficiency and core ratio come
  from published statistics of tall buildings: Shanghai high-rises (mean
  space efficiency 75 %, mean core share 23 % of gross area, ranges 52-93 % and
  5-33 %) <https://researchportal.tuni.fi/en/publications/shanghais-high-rise-buildings-exploring-space-efficiency-structur/>
  and Japanese tall buildings (net area ratio 64-85 %, core share 13-34 %)
  <https://ada.atilim.edu.tr/entities/publication/dedd7811-5f1c-486a-83fb-15d8c7abc7ad>.
  Those papers cover tall buildings; low office blocks run a lower core share.
* **[R] recalled from professional knowledge, not re-read.** Everything marked
  [R] is the author's recollection of the cited standard or of common practice
  and **must be checked against the standard text** before being relied on. The
  prototype keeps every [R] value in one `Rules` struct so correcting one is a
  one-line change, and the validation tests read their limits from it.
* Typology facts about core ratio per household (one lift per 60-80
  households; more than 120 households need 3 lifts with a fire lift) are from
  <https://www.sohu.com/a/221621522_119548> [V-news].

No images were copied; no imagery from map providers was used.

## 3. Standards that constrain the plan

### 3.1 GB 50096-2011 Design code for residential buildings [V]

| Clause | Rule | Used as |
|---|---|---|
| 5.1.2 | unit use area >= 30 m2; smallest (bed-sitting + kitchen + bath) >= 22 m2 | `min_unit_area` |
| 5.2.1 | double bedroom >= 9 m2, single >= 5 m2, bed-sitting >= 12 m2 | `min_double_bed`, `min_single_bed` |
| 5.2.2 | living room >= 10 m2 | `min_living` |
| 5.3.1 | kitchen >= 4.0 m2 (3.5 m2 in the smallest unit) | `min_kitchen` |
| 5.4.1/5.4.2 | bathroom with 3 fittings >= 2.5 m2; WC+basin 1.8, WC+bath 2.0, WC alone 1.1 | `min_bath` |
| 5.5.1 | storey height 2.80 m advised | `floor_height` by era |
| 5.5.2 | bedroom/living clear height >= 2.40 m | floor height - slab >= 2.40 |
| 5.5.4 | kitchen/bath clear height >= 2.20 m | |
| 5.6.2/5.6.3 | balcony guard bars <= 0.11 m apart; guard >= 1.05 m (<= 6 storeys), 1.10 m (>= 7) | balcony openings carry `guard_height` |
| 5.8.1 | window sill < 0.90 m with no balcony needs a guard | sill >= 0.9 or bay-window flag |
| 6.2.3 | >= 19 storeys: at least 2 exits per floor of a unit | tower gets 2 stairs (or a scissor pair) |
| 6.2.4 | two exits >= 5 m apart | office |
| 6.3.1 | stair flight >= 1.10 m clear (1.00 m if <= 6 storeys, one rail) | stair width |
| 6.3.2 | tread >= 0.26, riser <= 0.175 m, handrail >= 0.90 m | stair run length |
| 6.4.1 | lift required for >= 7 storeys or entry floor > 16 m | core contents |
| 6.4.2 | >= 12 storeys: at least 2 lifts per building | |
| 6.4.6 | lift lobby depth >= max car depth and >= 1.50 m | |
| 6.4.7 | lift not adjacent to a bedroom | core is never adjacent to a bedroom |
| 7.1.1 | every unit has at least one habitable room with winter sun | south living room |
| 7.1.3 | bedroom, living room and kitchen have direct daylight | **every habitable room has an exterior window** |
| 7.1.5 | window-to-floor ratio >= 1/7 for those rooms (glass area; openings with sill < 0.5 m do not count, 7.1.7) | daylight test |
| 7.1.6 | stair window-to-floor ratio >= 1/12 | stair window |
| 7.2.4 | kitchen natural vent opening >= 1/10 floor and >= 0.6 m2 | kitchen window sizing |
| 8.5.3 | a bathroom without window needs mechanical ventilation (dark bath is legal) | `Func::Bath { dark }` |
| 5.4.4 | bathroom not directly above bedroom/living/kitchen/dining of the unit below | typical floors repeat so it is satisfied inside a unit; across units by identical stacking |

Not found in the text read, therefore [R]: balcony clear depth (Chinese practice
1.2-1.5 m for old and "ordinary" units, 1.5-2.1 m for 2000s+ living-room
balconies; GB 50096 counts a balcony at half its area in the area calculation
only when unenclosed), corridor widths for the gallery-access type (6.5.1 gives
1.20 m for an external corridor [V]), and the 15 / 10 m dead-end rules of 6.2.1.

### 3.2 GB 50016-2014 (2018 edition) fire code [R unless marked]

* 5.3.1 [V-news]: high-rise (class I, II) compartment 1500 m2; single/multi-storey
  2500 m2; sprinklers double the area. Residential units are separate
  compartments bounded by 1.0-1.5 h walls; the plate is therefore one compartment
  per core in the prototype.
* 5.5.29 residential unit door to nearest exit, straight line [R]: 40 m between
  two exits, 22 m in a dead end for multi-storey and 20 m for high-rise; the
  prototype's `max_evac_unit_door` defaults to **20 m** (the stricter) and every
  residential core is a single stair directly beside the unit doors, so the
  expected distances are 2-8 m, far under. This is a check against error, not a
  tight design driver.
* 5.5.25 [R]: a single stair may serve a residential block up to 18 m (about 6
  storeys) or, in high-rise, up to 33 m with a protected stair and <= 6 units
  per floor and a 650 m2 plate; above 33 m or more units: two stairs or one
  scissor pair. Over 54 m (class I): two separate stairs. Lifts: one fire lift
  per compartment above 33 m. The typology sets (`stairs`, `lifts`) from storeys.
* 5.5.30 [V-news]: residential unit door and exit door clear width >= 0.80 m,
  corridor, stair, ground-floor exterior door >= 1.10 m.
* 5.5.17 [R]: for offices the straight line from a room door to the nearest
  exit is <= 40 m between exits, <= 22 m dead end (class 1-2, no sprinklers);
  an open floor needs any point within 30 m of an exit door.
* 5.5.8 [R]: two exits at least 5 m apart, in an office floor over 200 m2 /
  50 persons.

### 3.3 GB 50352-2019 (partly superseded by GB 55031-2022) and daylight

* GB 50352-2019 Uniform standard for civil buildings: stair, guard, window sill
  and clear-height rules; the mandatory clauses 4.3.1, 6.7.4, 6.8.6, 6.8.9 were
  moved into the mandatory code GB 55031-2022 effective 2023-03-01 [V from
  search listing]. Values used in the prototype are the GB 50096 ones above.
* GB 50033-2013 daylight: living rooms 1/6 for the 'III' light climate [R],
  GB 50096's 1/7 is the residential minimum used here.
* Sun spacing 日照间距 (GB 50180-2018 urban residential area planning, and city
  by-laws) [R]: spacing between slab rows = k x H with k about 1.6-1.8 at 40 N
  (Beijing), 1.2-1.4 at 31 N (Shanghai), 0.9-1.1 at 23 N (Guangzhou); winter
  solstice or "Great Cold" day, 1-2 hours (2 for class I climate). This is a
  site rule that fixes the slab's depth and orientation row by row; the plan
  only inherits `latitude` -> typical orientation within +-15 deg of south and
  balcony glazing.
* Energy codes [R]: JGJ 26-2018 (severe cold / cold), JGJ 134 (hot summer cold
  winter), JGJ 75-2012 (hot summer warm winter) cap the window-to-wall ratio per
  orientation. Typical caps: north 0.25-0.30 (cold) to 0.40 (warm); east/west
  0.30-0.35; south 0.45-0.50. Real measured residential facades are lower than
  the caps (0.25-0.40 overall). The prototype reports WWR per side and tests
  against the widest cap of any zone.

## 4. Typologies

### 4.1 Slab (板式) with a core of two units (一梯两户) [R unless marked]

* 2000s-2020s mid-rise (7-18 storeys) and 6-storey "ordinary" blocks.
* Orientation: long axis east-west, living side south. Plate 11-13 m deep (an
  "ideal" 11.4-12.6 m for 3-bed units), 24-38 m long.
* Units mirror about a central core: stair 2.6-2.8 m wide with a 5.0-5.6 m run
  (riser 0.15-0.17 m for a 2.8-3.0 m floor: 17-19 risers, two flights, about 9
  treads x 0.26 m = 2.34 m a flight plus landings), one or two lifts, a lobby of
  at least 1.5 m (GB 50096 6.4.6 [V]). The core sits on the north side of the
  centre, the south part of the central strip is given to the two living rooms.
* Unit: 南北通透 (through ventilation), living room and master bedroom to the
  south with the balcony; kitchen, bath, second bedroom north; short hall.
  Use areas 70-90 (2 bedrooms), 90-140 m2 (3 bedrooms).
* Reading from outside: balconies cluster as a column of two on each side of the
  core; the stair appears as a taller strip of small windows in the middle of
  the north face, half-floor offset (the stair landing is half a storey up); the
  north face has paired small windows for kitchen/bath.

### 4.2 Slab with 3, 4 or more units (一梯三户 / 一梯四户 / 一梯多户)

Same plate, a longer wall, more units per core; the units at the ends have a
gable; the central units may be narrow through-units or dark-bath units. In
high-rise slabs of 4-6 units per core the lift count rises with units
(60-80 households per lift [V-news]); middle units lose the gable window.

### 4.3 Point tower (点式塔楼), 一梯四户 pinwheel (风车型) [R]

* 19-34 storeys, floor plate 22-30 m square, one central core of about 6-8 m
  square holding stairs (one or a scissor pair), 2 lifts, a lobby and shafts;
  four units arranged in a pinwheel around it so each unit has two exterior
  walls (corner unit). Gross about 450-700 m2 per floor.
* The unit has a primary face (living, master bedroom, balcony), and a gable
  face with a secondary bedroom and often the kitchen. Baths and the foyer are
  commonly dark. Variants: 十字型/蝶形 (butterfly, wings off a core, more units),
  凹口 (notch) corner light wells.
* Reading from outside: balconies at the corners, regularly rotated around the
  tower (the rotational symmetry of the pinwheel is visible in elevations);
  vertical strips of small windows only where the stair is on the perimeter.

### 4.4 Old estate (老旧小区)

* **苏式 (Soviet-style, 1950s-60s)** [R]: 3-5 storeys, brick-masonry walls
  240-370 mm, storey height 3.0-3.3 m, central stair with a window, units either
  in a stair-hall plan (2 per landing) or with a wooden gallery (外廊), large
  pitched roof and decorated cornice. Floor plates: 12-14 m deep, units
  35-60 m2.
* **80s-90s 砖混 6-storey 单元楼 (the dominant stock)** [R]: 6 storeys (no lift,
  GB 50096 6.4.1 [V]), 2.7-2.9 m floors, 240 mm brick cross walls, concrete
  slabs; each unit stair serves 2 flats per floor (一梯两户), 3 (一梯三户) in some
  1990s blocks; plate 10-11 m deep, bays 3.0-3.3 m; unit use area 40-70 m2
  (小户型 1-2 bedrooms); stair in the centre of the north face with a window at
  each half-landing; kitchen north 4-5 m2 with a small window, bath 2-3 m2 with
  a small window, small hall (小厅) 8-12 m2 in the middle, bedrooms south (12-16
  and 9-12 m2); balcony 1.2 m deep and often only the main bedroom/hall; later
  glazed in by the owners.
* Reading from outside: low wide windows in an even grid, one bay of
  stair windows at mid-height up the middle, tiny kitchen and bath windows, steel
  bars and enclosures added to balconies, no lift core.

### 4.5 Townhouse / villa (联排 / 别墅) [R]

Not in the prototype. Party walls 5-6 m (townhouse bay 4.2-6.6 m), 3 storeys,
stair inside the unit, garage; the plan is the unit.

## 5. Dimensions and modules

* Bay (开间) module: 3.0, 3.3, 3.6, 3.9, 4.2, 4.5 m in 300 mm steps; masonry
  era 2.7-3.3 m, frame era 3.3-4.5 m; living room 3.6-5.4 m [R].
* Depth (进深): 10.5-12.6 m for through units; a room deeper than 6.0 m is
  not given a window at the back.
* Room areas by market era (use area, m2) [R, consistent with GB 50096 minima
  [V]]:

| Era | 1-bed | 2-bed | 3-bed | notes |
|---|---|---|---|---|
| 1950-70 Soviet | 28-38 | 40-58 | - | shared kitchens/baths common |
| 1980-95 | 30-42 | 42-62 | 62-75 | masonry, small hall |
| 1996-2010 | 40-55 | 65-90 | 90-130 | 90 m2 policy (2006) pushed the 2-3 bed type |
| 2010- | 40-60 | 70-95 | 95-140 | open plan living/dining, double balconies |

* Room area shares of the use area (median over a typical 3-bed) [R]: living
  and dining 25-30 %, bedrooms 38-45 %, kitchen 7-9 %, baths 6-9 %, hall and
  corridor 6-10 %.
* Balcony depth 1.2-1.5 m (old), 1.5-2.1 m (new); width equal to living or
  master-bedroom bay; service balcony 0.9-1.2 m by the kitchen.
* Window: bedroom 1.5-2.1 m wide, sill 0.9 m (0.45 for a bay window); living
  room balcony door 2.4-3.6 m with a fixed pane; kitchen 1.2-1.5 m, sill 1.0-1.2
  m; bath 0.6-0.9 m, sill 1.5 m; stair 1.2 m, sill at landing + 0.9 m.
* Storey heights: 2.7-2.9 (old), 2.8-3.0 (2000s), 2.9-3.1 (tower, GB 50096 5.5.1
  advises 2.8) [V/R].
* Air-conditioning ledge (空调位): 0.6-0.9 m wide platform beside the living and
  bedroom windows at the sill, louvred; a column of these reads as units on
  the facade.

## 6. Office tower (reference only: no room layout exposed)

* The plan is driven by the core, the lettable depth and the structural grid.
* Grid 8.4 or 9.0 m (parking module 2 cars at 2.4-2.5 m + column) [R], also
  9.0 x 9.0 and 9.0 x 12.0; columns on the perimeter and around the core.
* Lettable depth from curtain wall to core: 9-13.5 m, 12 m ideal (daylight
  penetration 1.5 x clear height) [R].
* Core: central for a square plate, edge core for a slab; core share of gross
  area 13-34 % in tall buildings, mean 23 % in Shanghai [V-study]; efficiency
  64-93 % (mean 75 %).
* Core contents: 2 stairs >= 5 m apart (GB 50096 6.2.4 equivalent for public
  buildings, GB 50016 5.5.8 [R]), lift banks by height zone, toilets, shafts.
* Curtain wall module 1.2-1.5 m (4.2 m storey, 0.9 m spandrel); ribbon windows
  for older (80s-90s) blocks.
* From outside: the grid, module, spandrels and the crown show; the interior is
  an open plan and is divided only by tenant.

## 7. Generation pipeline

1. **Programme**: `PlanSpec { typology, era, region, latitude, storeys, seed,
   plot: Option<(width, depth)> }`. Type from land use and height (7 storeys or
   more: lift core; 19 or more: tower), era from the development (cn_grammar's
   era), region and latitude give climate class (window caps, balcony
   glazing) and the sun azimuth.
2. **Plate**: width and depth drawn on the bay module inside the type's range
   (or fitted to the plot footprint if given).
3. **Core**: stair size from the storey height (riser count), lifts from
   storeys (`GB 50096 6.4`), lobby; placed at the type's position.
4. **Units**: slab: one unit per side of the core; tower: four pinwheel
   rectangles; old estate: two units per landing. Each unit is a rectangle (or
   the union of two) with a known primary face and entry edge.
5. **Rooms**: rows of rooms on bay widths (south row living/bedrooms, hall band,
   north row kitchen/bath/bedroom). Generate-and-test: sample, validate with
   `Rules` (areas, aspect, daylight, reachability) and resample on failure
   (bounded; the failure rate is a measured statistic).
6. **Doors** by connectivity growth from the entry hall (hall > living >
   bedroom).
7. **Openings** per room by function, era and climate; balconies and A/C ledges
   attached to the exterior rooms; stair windows at the half-landing.
8. **Read-outs**: `openings_by_side(side)` for the facade; `floor(n)` for the
   interior; the plate x storeys for the massing. A typical floor repeats; the
   ground floor and roof are the facade code's business (shops, lobby).

## 8. Data structures

```
PlanSpec -> Plan {
  typology, era, plate: Rect, floor_height, storeys,
  rooms:   Vec<Room { func, rect, unit, dark }>,      // incl. Balcony (outside plate)
  doors:   Vec<Door { a, b, at, width, kind }>,       // kind: Entry | Internal | Balcony | Core
  openings:Vec<Opening { side, along, width, sill, head, kind, room }>,
  units:   Vec<UnitInfo { rooms, bedrooms, use_area, entry_door }>,
  core:    CoreInfo { stairs, lifts, lobby rects, stair_door_points },
}
```

Plate frame: x east, y north, so the south face is `y = 0`; `along` is the x
(for S and N faces) or y (for E and W faces) coordinate of the opening centre.
Facade code maps its own span grammar onto `openings_by_side`.

## 9. Measurable statistics to validate

| Statistic | Target | Test |
|---|---|---|
| unit use area by bedrooms | table in section 5 | `unit_areas_in_code_ranges` |
| room area minima | GB 50096 5.2-5.4 | same |
| habitable rooms with exterior window | 100 % | `every_habitable_room_has_daylight` |
| window-to-floor ratio of those rooms | >= 1/7 | same |
| every unit reachable from the core stair | 100 % | `units_reach_the_core` |
| unit door to stair distance | <= 20 m (residential), <= 30 m open office | `evacuation_distance_within_limit` |
| distinct plan signatures over 500 seeds | >= 95 % per typology | `plans_are_unique` |
| living/bedroom share of use area | 25-30 % / 38-45 % | reported by the example |
| window-to-wall ratio by side | N <= 0.40, E/W <= 0.35, S <= 0.50 | `wwr_by_side_within_caps` |
| plate depth (slab) | 10.5-13.5 m | typology tests |
| units per core | 2 (slab), 4 (tower), 2 (walk-up), office 1 core | typology tests |
| office core share | 15-30 % | office test |
| determinism | bit-identical signature for the same spec | `deterministic` |

## 10. Reading units from outside (what the facade must show)

* 南北通透: south face has the balcony and big openings; the north face the
  small ones (kitchen, bath), stair strip and A/C ledges.
* Window width by function is the identification key: >= 2.4 m = living room
  balcony door; 1.5-2.1 m = bedroom; 1.2-1.5 m with a high sill = kitchen;
  0.6-0.9 m with a 1.5 m sill = bath.
* A stair strip of narrow windows offset by half a storey from the unit
  windows.
* Balcony column pairs on either side of the core (slab), corner balconies
  rotating (tower), full-width gallery or balcony row (old estate).
* Dark rooms never appear on the facade: a tower face with a blank bay marks a
  dark bath/foyer.

## 11. What is not done (next phases)

Full interior geometry (wall thickness, structural walls), furniture, finishes,
multi-floor stair connection (flight geometry), duplex/penthouse units, the
ground floor (shops, lobby, garage), townhouses, gallery-access old blocks,
unit-level uniqueness of furnishing, and verification of the [R] numbers
against the standards.

## 12. Prototype status and measurements

Code: `crates/city-scene/src/buildings/plan/` (`geom`, `unit`, `residential`,
`office`, `tests`), example `plan_floor_svg`. All numbers below are produced by
`cargo test --release -p city-scene --lib plan::tests -- --nocapture`.

| Typology, era | distinct signatures / 500 seeds | mean redraws | worst unit-door-to-stair |
|---|---|---|---|
| slab, 2000s | 500 | 1.3 | 3.6 m |
| slab, 2010s | 500 | 2.1 | 5.6 m |
| tower 一梯四户, 2010s | 500 | 3.6 | 4.3 m |
| tower 一梯四户, 2000s | 500 | 7.8 | 4.3 m |
| walk-up 80s-90s | 500 | 5.7 | 1.5 m |
| walk-up Soviet | 500 | 1.5 | 1.7 m |
| office 2010s | 431 | 0.1 | 28.2 m (any point, limit 30) |
| office 80s-90s | 472 | 0.1 | 28.2 m |

* Every generated residential plan passes: room minima (GB 50096 5.2-5.4, on net
  area inside the walls), 1/7 daylight for every living room, bedroom and
  kitchen, reachability of every room from the stair or lobby by the door graph,
  evacuation, no lift against a bedroom, unit use area inside the era window.
  All 9 era x typology combinations the facade uses draw for 200 of 200 seeds.
* Mean window-to-wall ratio by face (slab, 2010s): south 0.40, north 0.18-0.21,
  east and west gables 0.11; tower 0.28-0.31 on every face (the pinwheel is
  rotationally symmetric); walk-up south 0.30-0.36, north 0.18.
* Room mix (share of net use area): living+dining 19-29 %, bedrooms 40-48 %.
  The reference medians of section 5 are 25-30 % and 38-45 %: slabs and walk-ups
  run low on living (the dining room is only a separate room in the 2000s+ north
  row) and high on bedrooms.
* The office plate has a 15-30 % core share by test; its uniqueness is lower
  because a real office floor is a small set of plans (grid, depth, core
  position, lift count).

Known defects seen in the rendered plans (not hidden): units take their slack as
oversized bedrooms (master 26 m2 in a 4-bedroom slab unit), north-row `store`
rooms appear beside kitchens more often than real plans have them, tower halls
are 17-26 m2 (too big), the stair window of a slab is drawn at the plate
centre although the stair sits to one side of the core, office doors between
open bands are drawn like doors, and the fire-code figures are [R].

### Facade integration

`cn_grammar::residential_design` still draws its random elevations (so all other
choices of a design are unchanged) and then replaces the three elevation span
lists by the faces of the block's plan: tile = the first dwelling (living face
south, service face north, gable west or east), openings become spans by kind
(balcony door -> balcony, window with its plan sill and head, bay, A/C ledge ->
niche, stair window half-width at the tile edge where the mirror makes it
whole). The tile is mirrored at its edges by the existing geometry, which gives
the mirrored pair of a two-unit core and, in a tower, the rotated corner
units. Villages and commercial buildings are untouched. Deviations: the
latitude is fixed at 31 N for the plan, the tile is the first dwelling only
(the second is its mirror even when the plan drew two different flats), and the
slab's stair window is at the plate centre.

Not yet done: full interior geometry (wall thickness, structural walls),
furniture, multi-floor stair flights, duplex/penthouse, ground-floor and roof
plans, townhouses, gallery-access old blocks, and checking the [R] numbers.
