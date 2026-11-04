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

FINDINGS_PLACEHOLDER
