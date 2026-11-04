# Sanity audit: does what the generator made make sense?

`docs/methodology.md` principle 7 says a feature is acceptable only if someone who
knows the real thing would find it plausible in place, function and relation to
its neighbours, and that this is checked by rules, not by eye. This is that check.

* Library: `crates/city-scene/src/audit/` (`mod.rs` catalogue and report, `checks.rs`
  the rules, `geom.rs` helpers, `tests.rs` planted-defect tests and the gate).
* CLI: `crates/city-scene/examples/sanity_audit.rs`, wrapped by `scripts/sanity-audit`.
* Regression gate: `cargo test --release -p city-scene audit` (the demo city must
  stay at or under the `gate` of every check in `audit::CHECKS`).

## Running it

```
scripts/sanity-audit                                   # the demo city
scripts/sanity-audit --world 284735                    # a town of a generated world
scripts/sanity-audit --world 7 --landform plateau --json /tmp/audit.json --top 10
scripts/sanity-audit --from plan.json --climate 8,0.4,1500,900
scripts/sanity-audit --no-gate                         # report, always exit 0
```

Inputs are those of `dump_scene` (`--seed`, `--radius-km`, `--block-m`, `--disc`,
`--from`, `--fixture`/`--city`, `--world SEED [--town i] [--landform ..] [--grid ..]`,
`--climate`, `--lite`). It builds the plan, derives the street network, builds
the scene (so trees, furniture and vehicles, which exist only as instances, are
judged too) and prints one line per check, then the worst offenders of each check
that found something, with city-local `(x, z)` metres, the size of the violation
and ids (`building:12`, `road:7`, `junction:3`, `field:9`, `compound:4`, an
instance key). `--json` writes everything (counts, the 40 worst per check).
The exit status is 1 when a check is over its gate, which for a world town is
expected today: the gates are the demo city's numbers, and lowering one is how a
fix is locked in. Never raise a gate.

With `--world` the terrain heights are known, so the slope checks run. Without it
they are skipped (`terrain no` in the first line of the output).

From a test: `audit::audit_city(&city, SceneBudget::default())`, or
`audit::audit(&city, &network, Some(&scene), terrain, max_buildings)`.

## What is checked

Each check has a severity (1 unproven or cosmetic, 2 wrong but local, 3 plainly
broken), a visibility (1 close look, 2 from the street, 3 from afar) and an owner
(the module that most probably produces the defect). The ranking key is
`count x severity x visibility`.

| group | checks |
|---|---|
| ground and water | `water.building`, `water.road`, `water.tree`, `water.furniture`, `water.field`, `ground.level` (floating or sunk into a deck), `ground.slope_building` (over 15 percent), `ground.slope_field` (over 25 degrees) |
| overlaps | `overlap.building_building` (shop-podium towers are exempt), `overlap.building_road` (kerb, junction box, headroom under a viaduct), `overlap.building_field`, `overlap.field_road`, `overlap.tree_road`, `overlap.tree_building`, `overlap.furniture_lane`, `overlap.furniture_building`, `overlap.vehicle_building`, `overlap.vehicle_offroad`, `overlap.fence_road` |
| bridges | `bridge.clear_water` (under 1.5 m), `bridge.clear_road` (under 4.5 m headroom, CJJ 37), `bridge.at_grade_crossing`, `bridge.step`, `bridge.gap`, `bridge.grade`, `network.warning` (what the derivation itself reported) |
| network | `net.orphan`, `net.dead_end`, `net.lane_width`, `net.junction_angle` (under 30 degrees), `net.junction_control` (signalised without heads, priority control between arterials, footways with no crossing) |
| buildings | `bldg.storey` (floor-to-floor by use), `bldg.height_use`, `bldg.street_access`, `bldg.spacing_highrise` (13 m, GB 50016), `bldg.sliver`, `bldg.degenerate` |
| vegetation | `veg.species_climate` (against `trees/zones.rs` for the site climate), `veg.size` |
| land use | `land.industrial_core`, `land.village_water` |
| interchanges | `ramp.grade` (5 percent), `ramp.continuity` |
| level of detail | `lod.roof` (far-mass roof against the plan height), `lod.count` |

## Not checked, and why

* **Sunlight spacing (日照间距).** Needs the north direction of the plan frame; the
  plan carries none. The 13 m fire spacing stands in for it and is only a floor.
* **Piers in carriageways, approach ramps of a bridge as meshes.** Piers exist only
  as mesh triangles; the audit sees the road path (deck height, grade, ends meeting
  junctions), not the pier geometry.
* **Entrances reachable by footway, windows crossing wall edges, signs on
  ground floors only, shop on a street frontage.** The floor plans and shop-sign
  placement are inside the mesh builders and under active rework; the plan only
  says `entrance_count`. `bldg.street_access` is the plan-level proxy.
* **Schools near housing, farmland on slopes in the demo.** The plan has no school
  use; slopes need `--world`.
* **Crown size against age.** Trees carry no age; `veg.size` only bounds height
  and crown against the species' mature range.
* **Markings.** Only whether a junction has signal heads or a crossing is known
  from the network; paint is a mesh.
* **Terrain under roads and buildings.** The plan frame is flat and the renderer
  drapes it; `ground.level` therefore checks an object against the surface it
  stands on in the plan (ground or deck), not against the heightfield.
* The slope probe samples the 80 km world grid (about 310 m cells) over 240 m, so
  it sees valley-scale slope, not a retaining wall.

## Findings

Runs on 4 November 2026: the demo city (`scripts/sanity-audit`) and three worlds,
`--world 284735 --landform plains`, `--world 7 --landform plateau`,
`--world 91 --landform coastal` (town 0 of each, 256 grid, default climate). Other
agents were changing the generators while these were run, so the numbers drift
by a few percent between runs; the order does not. Counts are violations found.
Each run takes about 30 s.

### Counts per check, ranked by count x severity x visibility

| check | owner | sev | vis | demo | 284735 plains | 7 plateau | 91 coastal |
|---|---|---|---|---|---|---|---|
| `lod.roof` far roof height differs from the detail roof | buildings::far | 2 | 3 | 9 | 1032 | 1370 | 1484 |
| `bldg.spacing_highrise` two high-rises closer than 13 m (GB 50016) | urban parcels | 2 | 2 | 0 | 318 | 565 | 2148 |
| `bldg.sliver` gap between two buildings narrower than 4 m but not touching | urban parcels | 1 | 2 | 0 | 495 | 943 | 3896 |
| `veg.species_climate` species not in the planting zone of the site climate | trees::zones / plant | 2 | 2 | 0 | 564 | 626 | 0 |
| `bldg.storey` floor-to-floor height outside the range for the use | urban parcels / buildings | 2 | 2 | 1 | 171 | 129 | 92 |
| `bldg.street_access` no street or footway within reach of the building | urban parcels | 2 | 1 | 0 | 143 | 206 | 300 |
| `overlap.building_road` building inside a carriageway, junction or under a deck | urban parcels vs network | 3 | 3 | 0 | 64 | 34 | 40 |
| `overlap.building_building` two buildings overlapping | urban parcels | 3 | 3 | 0 | 48 | 49 | 35 |
| `net.orphan` road component not connected to the main network | urban graph | 3 | 3 | 0 | 22 | 26 | 24 |
| `water.furniture` lamp, pole or sign standing in the river channel | furniture | 2 | 2 | 1 | 72 | 20 | 23 |
| `water.building` building footprint in the river channel | urban parcels / buildings::river | 3 | 3 | 0 | 22 | 8 | 11 |
| `overlap.tree_road` tree trunk in a motor lane or junction | trees::plant | 3 | 2 | 0 | 4 | 4 | 9 |
| `overlap.fence_road` compound fence or gate on a carriageway | urban parcels (compounds) | 2 | 2 | 0 | 10 | 8 | 5 |
| `net.dead_end` dead end implausible for its class or length | urban graph | 2 | 2 | 2 | 9 | 8 | 4 |
| `water.road` road at grade through the river channel | urban roads + network deck | 3 | 3 | 0 | 4 | 3 | 1 |
| `veg.size` tree height or crown outside the species range | trees::plant | 2 | 2 | 0 | 0 | 0 | 17 |
| `overlap.furniture_lane` lamp, pole, sign, bin or shelter in a motor lane or junction | furniture | 3 | 2 | 0 | 9 | 0 | 0 |
| `overlap.field_road` field ring crossing a carriageway | urban landscape | 2 | 2 | 0 | 5 | 1 | 5 |
| `network.warning` engineering problem the network derivation itself reported | network | 2 | 2 | 0 | 6 | 3 | 1 |
| `water.tree` tree standing in the river channel | trees::plant | 2 | 2 | 0 | 0 | 4 | 0 |
| `net.junction_angle` junction arms closer than 30 degrees | urban graph / tidy | 2 | 2 | 0 | 2 | 0 | 2 |
| `lod.count` buildings dropped from the detail but present in the far mass | scene budget | 2 | 3 | 0 | 0 | 0 | 1 |

checks with no finding in any run: water.field, ground.level, ground.slope_building, ground.slope_field, overlap.building_field, overlap.tree_building, overlap.furniture_building, overlap.vehicle_building, overlap.vehicle_offroad, bridge.clear_water, bridge.clear_road, bridge.at_grade_crossing, bridge.step, bridge.gap, bridge.grade, net.lane_width, net.junction_control, bldg.height_use, bldg.degenerate, land.industrial_core, land.village_water, ramp.grade, ramp.continuity

The demo city (25 buildings) is too small to exercise most checks, which is why
its gate is a floor and the worlds are the real test. Checks that found nothing in
any run are listed above; for the bridge, ramp and grade checks that is a result
(68, 42 and 57 bridges, 11 to 24 ramps were audited and all clear the water and
each other, meet their junctions without a step or gap, and stay within grade),
but they have been proven to fire only through the planted-defect tests for the
plan-level checks (`audit/tests.rs`), not for the bridge family.

### Defects by owner

**Plan: parcels, buildings, compounds (`crates/urban/src/modern/parcels.rs`, `tidy.rs`)**

* Buildings in the carriageway, `overlap.building_road`, 34 to 64 per world. Ten
  and 13 m buildings stand on an arterial's centreline (16.5 m of intrusion):
  `building:620`, `621`, `622` on `road:661` at (622, 710), (613, 672), (598, 629)
  in world 284735; `building:3304` to `3306` on collector `road:583` at (693, 67) in
  world 91. Also towers under viaducts with no headroom: a 77 m tower beneath the
  expressway `road:1118` at (679, 356), 64 m into the 4.5 m headroom (world
  284735); a 60 m tower under `road:748` at (-248, -230) (world 91).
* Buildings overlapping, `overlap.building_building`, 35 to 49 per world, up to
  1049 m2 shared: `building:296` (18 m) and `297` (122 m) at (61, 511) in world 91;
  `building:723` and `724` at (299, -129) in world 284735. Some may be shop-podium
  towers the podium detector does not recognise (the detector in
  `buildings::far::podiums_of` wants the tower's centre inside a lower, larger
  building; the audit also accepts a tower whose footprint is 80 percent inside a
  lower building that is not smaller than it).
* Buildings in the river, `water.building`, 8 to 22 per world, up to 18 m into the
  channel: `building:1095` at (-656, -915) in 284735; `building:2394` at
  (1867, -250) in the plateau world; `building:1386`, a 52 m tower, at (-81, 860) in
  the coastal one. Shared with the river work (`buildings::river`).
* Plan height against floors, `bldg.storey`, 92 to 171 per world: residential
  buildings with 2 floors and a height of 20.7 m (10.35 m per floor) in every
  world, e.g. `building:646` at (1192, 2144). The plan's `height_metres` also
  disagrees with the height the detail builds for it (`lod.roof`, below), so the
  height field is not the truth for any consumer.
* Gaps between buildings, `bldg.sliver` (under 4 m, 495 to 3896 per world) and
  `bldg.spacing_highrise` (towers closer than 13 m, 318 to 2148 per world). The
  0.6 to 1.5 m slits between the 14-storey handshake blocks of the urban village
  (`parcels.rs`, village rows) are the cause of most of them: real 城中村 blocks
  are 6 to 9 storeys, rarely 15, and fire codes still ask 6 m. They are cheap to
  see from the street and cannot be right.
* No street in reach, `bldg.street_access`, 143 to 300 per world: industrial and
  commercial blocks 200 to 370 m from any street, e.g. `building:3737` at
  (-1019, -2164) in the coastal world. These are blocks whose access road was
  not generated.
* Fences on the road, `overlap.fence_road`, 5 to 10 per world, up to 12 m into the
  motor lanes: `compound:19` and `18` on `road:234` at (165, 1082) in the coastal
  world; `compound:37` on `road:947` at (942, 1372) in 284735.
* Fields over roads, `overlap.field_road`, 1 to 5 per world (`field:632` on the
  648 m collector dead end `road:739` at (-2747, 1492)).

**Road graph (`crates/urban/src/modern/graph.rs`, `expressway.rs`)**

* Road crossing a river at grade with no bridge, `water.road`, 1 to 4 per world:
  `road:608` runs 22.8 m into the channel at (-2615, 1400) in 284735; `road:60`
  33.7 m at (-731, -1837) in the plateau world. Street furniture is then built on
  the water (`water.furniture`, 20 to 72 bollards: 35.8 m into the channel at
  (-2617, 1415)), which is the same defect seen a second time, not a furniture bug.
* Road fragments not joined to the network, `net.orphan`: 22 to 26 per world, almost
  all nodes that no road uses (`node:87` at (2334, 756)); one real fragment, 4
  roads at (3024, 1158) in the plateau world.
* Dead ends, `net.dead_end`, 4 to 9 per world: collectors and arterials that stop
  in the fields, a 648 m collector (`road:739`, (-3262, 1693)), a 238 m collector
  (`road:127`, (-1293, -3440)), an arterial after 72 to 136 m.
* Junction arms 0 degrees apart (`junction:104` at (1846, -591)) and 19 degrees
  (`junction:45` at (-284, -824), five arms).

**Network and street layer (`network.rs`, `street/`)**

* `network.warning`: "road 56 crosses a road on a 0.0 m deck, below the 4.5 m
  clearance it needs" (also road 67 in the plateau world): a bridge the span cannot
  lift. The derivation already reports it; the planner should lengthen or drop it.
* Signalised junctions without signal heads: 7 to 22 per world before the fix
  below (a third of four-arm crossings were skipped at random, and collector
  crossings were never given heads).
* Bollards standing in the carriageway, `overlap.furniture_lane`, 9 in 284735
  (`road:976` at (1819, -560), 9.4 m in).

**Trees (`trees/plant.rs`, `trees/zones.rs`) - another agent's**

* Shelterbelts are always poplar, whatever the planting zone, so 564 and 626
  `yang-shu` stand in the coast-warm zone where the zone tables do not list it
  (`veg.species_climate`, worlds 284735 and 7; `plant.rs` `POPLAR`).
* Shelterbelt rows run onto collector roads, `overlap.tree_road`, 4 to 9 per world
  (3.3 m into `road:127` at (-1199, -3397)).
* Waterfront trees in the channel, `water.tree` (4, plateau world, 5.2 m in at
  (11, 2820)); tree heights 30 to 60 percent above the species' mature maximum,
  `veg.size` (17, coastal world).

**Building LOD (`buildings/far.rs`, `cn_residential.rs`, `cn_commercial.rs`) - another agent's**

* `lod.roof`: 1032 to 1484 buildings per world whose plan height differs from the
  roof the detail builds (and the far mass draws) by more than 1 m, by up to 30 m:
  `building:3332`, plan 109.4 m, detail roof 81.3 m at (-327, -197). The detail is
  always lower for towers. Anything that reads `height_metres` (shadows, the
  satellite view for kinds with no far look, flight clearance) sees a different
  city from the one that is drawn.
* `lod.count`: 384 of 4384 buildings in the coastal world have a far form but no
  detail shell, because the scene budget cuts at 4000.

**Rail (`rail.rs`)** - the scene now reports "rail: line 2.8 m over a street at 158 m,
below the 5.0 m clearance" (plateau world) through `network.warning`; new, not mine.

### What was fixed

* **Trees and street furniture inside buildings** (`overlap.tree_building`, 9 to 39
  per world, and `overlap.furniture_building`, 31 to 81, bins, lamps and
  bollards on forecourts the lots had taken). `scene.rs` now drops, right after
  planting and before the buildings are built, every tree and every piece of
  furniture (railings excepted) whose foot is inside a building footprint
  (`drop_inside_buildings`, `MeshBuilder::retain_keyed_instances_since`), and
  corrects the scene statistics. After the fix both checks are 0 in all worlds
  and in the demo.
* **Signalised junctions with no signal heads** (`net.junction_control`, 7 to 22
  per world, 3 in the demo). `street/signals.rs` skipped 35 percent of the
  four-arm signalised junctions at random and every one with no arterial arm,
  while markings, crossings and the signal plan treat them as signalised. Every
  junction the network classes as signalised now gets heads on all its footway
  arms. 0 in all worlds and the demo.
* Roundabout islands are not carriageway for the audit's own checks (a false
  positive of the first version of `overlap.furniture_lane`).

Both fixes have regression tests (`audit::tests::nothing_planted_stands_inside_a_building`,
`every_signalised_junction_has_signal_heads`), and the demo city gate runs in
the normal test suite (`audit::tests::the_demo_city_is_within_the_gate`).

### Handing back

The remaining defects are in the plan generators (`urban`), the river and the tree
and building-LOD work that other agents are changing. Nothing was changed there.
Suggested assignment: parcels overlap and road-intrusion (one cause: lots are cut
before the road net is final), urban-village form (floors and slits), the
at-grade river crossings and dead ends (graph), shelterbelt species by zone and
off the roadway (trees), plan height from the detail roof (building LOD).
