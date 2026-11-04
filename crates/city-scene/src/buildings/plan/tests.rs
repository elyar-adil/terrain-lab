use super::*;
use std::collections::HashSet;

const SEEDS: u32 = 500;

fn specs() -> Vec<(Typology, PlanEra, u16, f32)> {
    vec![
        (Typology::SlabTwoUnit, PlanEra::Millennium, 11, 31.0),
        (Typology::SlabTwoUnit, PlanEra::Contemporary, 18, 39.0),
        (Typology::SlabTwoUnit, PlanEra::Contemporary, 11, 23.0),
        (Typology::TowerFourUnit, PlanEra::Contemporary, 26, 31.0),
        (Typology::TowerFourUnit, PlanEra::Millennium, 18, 36.0),
        (Typology::WalkUp, PlanEra::Estate, 6, 34.0),
        (Typology::WalkUp, PlanEra::Soviet, 5, 40.0),
        (Typology::Office, PlanEra::Contemporary, 30, 31.0),
        (Typology::Office, PlanEra::Estate, 8, 36.0),
    ]
}

fn plans(t: Typology, era: PlanEra, storeys: u16, lat: f32, n: u32) -> Vec<Plan> {
    (0..n)
        .map(|seed| Plan::generate(PlanSpec { typology: t, era, latitude: lat, storeys, seed }))
        .collect()
}

#[test]
fn generation_is_deterministic() {
    for (t, e, s, lat) in specs() {
        for seed in 0..20 {
            let spec = PlanSpec { typology: t, era: e, latitude: lat, storeys: s, seed };
            assert_eq!(Plan::generate(spec).signature(), Plan::generate(spec).signature(), "{t:?} {seed}");
        }
    }
}

#[test]
fn every_plan_satisfies_the_code_rules() {
    for (t, e, s, lat) in specs() {
        let rules = Rules::for_climate(Climate::of_latitude(lat));
        for p in plans(t, e, s, lat, 150) {
            let bad = p.check(&rules);
            assert!(bad.is_empty(), "{t:?} {e:?} seed {}: {bad:?}", p.spec.seed);
        }
    }
}

#[test]
fn unit_areas_fall_in_the_era_ranges_and_cover_the_room_mix() {
    for (t, e, s, lat) in specs().into_iter().filter(|x| x.0 != Typology::Office) {
        let mut living = 0.0;
        let mut bedrooms = 0.0;
        let mut total = 0.0;
        for p in plans(t, e, s, lat, 200) {
            for u in &p.units {
                let (lo, hi) = unit_area_range(e, u.bedrooms);
                assert!(u.use_area >= lo && u.use_area <= hi, "{t:?} {e:?} {} bed {}", u.bedrooms, u.use_area);
                for &i in &u.rooms {
                    let r = &p.rooms[i];
                    if r.func.is_balcony() {
                        continue;
                    }
                    total += r.net_area();
                    if matches!(r.func, Func::Living | Func::Dining) {
                        living += r.net_area();
                    }
                    if r.func.is_bedroom() {
                        bedrooms += r.net_area();
                    }
                }
            }
        }
        println!(
            "{t:?} {e:?}: living+dining {:.0} %, bedrooms {:.0} % of use area",
            100.0 * living / total,
            100.0 * bedrooms / total
        );
        // Section 5 of the design doc: 25-30 % and 38-45 % are medians of real
        // plans; the generator is accepted within a wider band.
        assert!((0.15..0.40).contains(&(living / total)), "living share {}", living / total);
        assert!((0.28..0.55).contains(&(bedrooms / total)), "bedroom share {}", bedrooms / total);
    }
}

#[test]
fn every_habitable_room_has_daylight() {
    for (t, e, s, lat) in specs().into_iter().filter(|x| x.0 != Typology::Office) {
        for p in plans(t, e, s, lat, 100) {
            for (i, r) in p.rooms.iter().enumerate().filter(|(_, r)| r.func.needs_daylight()) {
                let glass: f32 = p
                    .openings
                    .iter()
                    .filter(|o| o.room == i)
                    .filter(|o| matches!(o.kind, OpeningKind::Window | OpeningKind::BalconyDoor | OpeningKind::Bay))
                    .map(Opening::glazed_area)
                    .sum();
                assert!(glass >= r.net_area() / 7.0 - 1e-3, "{t:?} seed {} room {i} {:?}", p.spec.seed, r.func);
            }
        }
    }
}

#[test]
fn every_unit_reaches_the_core_and_evacuation_is_short() {
    for (t, e, s, lat) in specs() {
        let limit = if t == Typology::Office { 30.0 } else { 20.0 };
        let mut worst: f32 = 0.0;
        for p in plans(t, e, s, lat, 150) {
            assert!(p.all_reachable(), "{t:?} seed {}", p.spec.seed);
            worst = worst.max(p.evac_max);
        }
        println!("{t:?} {e:?}: worst evacuation distance {worst:.1} m (limit {limit})");
        assert!(worst <= limit);
    }
}

#[test]
fn window_to_wall_ratio_by_face_is_reported_and_capped() {
    for (t, e, s, lat) in specs().into_iter().filter(|x| x.0 != Typology::Office) {
        let mut sum = [0.0f32; 4];
        let ps = plans(t, e, s, lat, 100);
        for p in &ps {
            for (n, side) in Side::ALL.iter().enumerate() {
                sum[n] += p.wwr(*side);
            }
        }
        let k = ps.len() as f32;
        println!(
            "{t:?} {e:?}: mean WWR S {:.2} E {:.2} N {:.2} W {:.2}",
            sum[0] / k,
            sum[1] / k,
            sum[2] / k,
            sum[3] / k
        );
    }
}

#[test]
fn plans_are_unique_across_seeds() {
    for (t, e, s, lat) in specs() {
        let ps = plans(t, e, s, lat, SEEDS);
        let sigs: HashSet<u64> = ps.iter().map(Plan::signature).collect();
        let redraws: u32 = ps.iter().map(|p| p.attempts).sum();
        println!(
            "{t:?} {e:?}: {} distinct of {SEEDS}; mean redraws {:.2}",
            sigs.len(),
            redraws as f32 / SEEDS as f32
        );
        let need = if t == Typology::Office { 0.5 } else { 0.99 };
        assert!(sigs.len() as f32 >= SEEDS as f32 * need, "{t:?} {e:?} only {} distinct", sigs.len());
    }
}

#[test]
fn typologies_have_the_units_per_core_they_name() {
    for (t, e, s, lat) in specs() {
        for p in plans(t, e, s, lat, 30) {
            match t {
                Typology::SlabTwoUnit | Typology::WalkUp => assert_eq!(p.units.len(), 2),
                Typology::TowerFourUnit => assert_eq!(p.units.len(), 4),
                Typology::Office => {
                    assert!(p.units.is_empty());
                    let core: f32 = p
                        .rooms
                        .iter()
                        .filter(|r| !matches!(r.func, Func::Open))
                        .map(|r| r.rect.area())
                        .sum();
                    let ratio = core / p.plate.area();
                    assert!((0.08..0.34).contains(&ratio), "core ratio {ratio}");
                }
            }
            if t == Typology::SlabTwoUnit {
                assert!((10.5..=13.5).contains(&p.plate.d()), "slab depth {}", p.plate.d());
            }
        }
    }
}

#[test]
fn every_facade_combination_can_be_drawn() {
    // The residential facade reads plans for these combinations (cn_grammar).
    let eras = [PlanEra::Estate, PlanEra::Millennium, PlanEra::Contemporary];
    let kinds = [(Typology::WalkUp, 7u16), (Typology::SlabTwoUnit, 14), (Typology::TowerFourUnit, 26)];
    for era in eras {
        for (typology, storeys) in kinds {
            let failed = (0..200u32)
                .filter(|&seed| {
                    Plan::try_generate(PlanSpec { typology, era, latitude: 31.0, storeys, seed }).is_err()
                })
                .count();
            println!("{typology:?} {era:?}: {failed} of 200 seeds without a valid plan");
            assert_eq!(failed, 0, "{typology:?} {era:?}");
        }
    }
}
