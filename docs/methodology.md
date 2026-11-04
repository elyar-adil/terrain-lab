# Methodology: procedural generation from orbit to leaf

This is a research project. Its question is how far procedural generation can go
when every layer is held to physical and observational evidence. It is not a
game, and nothing is tuned merely to look pleasing.

## Target

One continuous world that can be viewed from a satellite and zoomed, without a
break in the model, down to:

- individual leaves on a tree;
- every building enterable, and every room of every building distinct.

"Distinct" means a different design drawn from the same rules, not a recoloured copy.

## Principles

1. **Causal chain.** Each layer is derived from the layer below it. Nothing is
   placed because it looks right.
   - climate and geology -> terrain -> hydrology -> soil and vegetation zone
   - terrain and water -> settlement sites -> road network -> land use
   - land use and era -> building types -> floor plans -> rooms -> furniture
   Any new feature must name its inputs. A parameter with no upstream source is a defect.
2. **Deterministic and unique.** Everything is a pure function of (world seed,
   location, scale). Variation comes from seeded sampling of design rules, never
   from a lookup table of finished assets. The same input always gives the same
   result, so any region can be regenerated on demand and nothing needs to be stored.
3. **One model, many scales.** Levels of detail are the same object refined, not
   different objects swapped in. A far tree is the summary of a near tree (same
   species, size and colour). A far building is the envelope of its interior.
   Detail is generated lazily when the camera needs it, and the coarse levels
   must be consistent with it (no popping in colour, size or count).
4. **Grounded in references.** Each rule cites an observation: a survey, a
   standard (for example GB 50352, GB 50096, JTG roads, CJJ 37), a botanical
   description, or an openly licensed photograph. References are viewed and cited
   in docs; they are not copied into the repository.
5. **Validation as measurement.** For each subsystem we state an observable
   statistic and compare generated values against real ones: street widths and
   block sizes, building height distribution, floor areas per unit, species
   proportions per climate, grade and clearance on ramps. A render is only
   evidence for what it was looked at for; unchecked claims are listed as such.
6. **Honest limits.** What is not modelled is recorded in `docs/city-issues.md`
   instead of being hidden by a plausible-looking default.

7. **Everything must make sense.** A generated feature is acceptable only if a
   person who knows the real thing would find it plausible in place, function
   and relation to its neighbours. Plausibility is checked by rules, not by
   eye: no road or building under water, no object overlapping another, every
   bridge clears the water or road below it, every road connects to the
   network, every building has an entrance reachable from a street, every
   plant fits its climate, every shop sign belongs to a shop on a ground floor.
   These checks run automatically over the generated scene
   (`scripts/sanity-audit`, described in `docs/sanity-audit.md`) and a failure counts as a
   defect in the generator.

8. **The real application is the only acceptance interface.** What `cargo run`
   (the desktop app in `src-tauri`) shows is the product. Harness pages, the
   single-file web build and `dump_scene` renders are development aids; a
   feature counts as done only when it has been seen in the real app. Reports
   must say which interface a check was made in.

## Scale ladder and what each rung needs

| Scale | Must be true | State |
|---|---|---|
| Satellite (50 km to 1 km) | terrain, water, land cover, fields, road net, urban footprint in correct colour and texture | partial: far view is coarse, river and parcel paving in progress |
| District (1 km to 100 m) | block pattern, building mass, street trees, correct roof types | working in the demo city |
| Street (100 m to 2 m) | facades, shops, signs, furniture, markings, vehicles | working, quality still uneven |
| Plant (2 m to 1 cm) | species-correct crown, branches, leaves and bark | in progress: climate-driven species zones |
| Interior | every building enterable; floor plan and rooms by building type | **not started** |
| Leaf and material | per-leaf shape, veins, wear on materials | not started |

## Interiors: the next large piece

Interiors must be generated, not authored.

1. **Program from type.** Building type (rule-based from land use, size, era,
   region) gives a room program: units per floor, rooms per unit, circulation,
   stair and lift cores, services. Residential blocks follow national and
   regional design codes for unit mix and depth.
2. **Plan from envelope.** The footprint and storey count that the exterior
   already uses are the constraint. The plan is solved inside it (core placement,
   corridor or hall, unit subdivision, room partition) so windows, balconies and
   doors on the facade line up with rooms inside. Exterior and interior must be
   the same building.
3. **Furnish from function.** Room function selects furniture sets and layouts
   with clearances, then lighting, finishes and wear from the building's age and
   its household or business.
4. **Distinct by construction.** Each unit draws its own plan variant,
   furnishing and finishes from seeded distributions. Uniqueness is tested: hash
   a plan and furnishing signature per unit and report the collision rate.
5. **Streaming.** Interiors are generated only for buildings near the camera
   (or entered), cached by (seed, building id), and evicted. Entering a building
   must not require a precomputed asset.

Open choices, to be settled by a short design study before code: the
floor-plan solver (constraint-based subdivision vs grammar), how doors and
stairs connect units across floors, and the interior lighting model.

## Leaf scale

Leaf geometry follows the species parameters already needed for crown shape:
blade outline, venation, size distribution, colour variation with age and light,
and attachment angle. Near trees are instanced from per-species leaf and shoot
models; the crown generator places shoots, so coarse billboards can be derived
from the same data and stay consistent with the near model.

## Method for each change

1. State the observable it targets and its upstream inputs.
2. Cite the reference.
3. Implement as a deterministic rule.
4. Add a test for the measurable statistic (not only "does not panic").
5. Render and read the result at the scale it is meant for; record what was seen.
6. Record the gap in the issue list if something is deliberately left out.
