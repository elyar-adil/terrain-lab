//! The far form of a building: what its facade grammar averages to.
//!
//! The city is drawn twice: as metre-scale detail within a few hundred metres
//! and, beyond that, as one merged mass built in the browser from the plan
//! (`src/city/cityMass.ts`).  If the two were drawn from different rules the
//! building would change colour, height and window rhythm at the hand-off.  So
//! the far form is not drawn from its own palette: [`FarLook`] is read off the
//! very `Design` or office `Elevation` the detail is built from, with the same
//! seeds, and travels in the plan payload.  The mass then draws one window per
//! bay of that elevation, in that elevation's wall colour, to that roof height.
//!
//! Heights are measured from the ground datum, as the detail's roof deck is.

use serde::Serialize;
use urban::ModernBuilding;

use crate::facades::cn_grammar::{Crown, Elevation, Glass, Part, Skin};

/// How the window of one bay is laid out, as the mass draws it.
pub const KIND_PUNCHED: u8 = 0;
/// A curtain wall: mullions every `module` metres, a spandrel at every slab.
pub const KIND_CURTAIN: u8 = 1;
/// A stone grid: deep piers and a window between.
pub const KIND_GRID: u8 = 2;
/// A ribbon: a long window under a panel band.
pub const KIND_RIBBON: u8 = 3;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FarLook {
    /// Roof deck height above the ground datum, metres: the detail's own.
    pub roof_m: f32,
    /// Where the building's first storey starts above the ground, metres: 0, or
    /// the roof of the lower building it stands on (商住楼: shop podium and tower).
    pub start_m: f32,
    /// Storey height, metres.
    pub storey_m: f32,
    /// Height of the ground storey, metres; 0 when it is a storey like the rest.
    pub shop_m: f32,
    /// Body colour, linear reflectance.
    pub wall: [f32; 3],
    /// A darker base of `base_n` storeys and a different top of `top_n`.
    pub base_band: [f32; 3],
    pub base_n: u32,
    pub top_band: [f32; 3],
    pub top_n: u32,
    /// Width of one repeating flat, metres (the wall is cut into whole flats).
    pub unit_w: f32,
    /// One of the `KIND_*` codes.
    pub kind: u8,
    /// The principal window: x0, x1 across the flat, sill and head above the floor.
    pub window: [f32; 4],
    /// The balcony or loggia door, if the flat has one: x0, x1, head. x1 = 0 for none.
    pub balcony: [f32; 3],
    /// Curtain module, or the pier width of a grid, metres.
    pub module_m: f32,
    /// Glass colour as the facade tints it, linear.
    pub glass: [f32; 3],
    /// 0 clear, 1 tinted, 2 office, 3 curtain.
    pub glass_kind: u8,
    pub frame: [f32; 3],
    pub trim: [f32; 3],
    /// The wall skin code the facade shader uses.
    pub skin: u8,
    /// Bit 0 moulded surrounds, bit 1 a band at every slab.
    pub flags: u8,
    pub weather: f32,
    /// 0 flat parapet, 1 frame, 2 hipped clay, 3 mansard, 4 new-Chinese tile.
    pub crown: u8,
    /// Clay roof (red) rather than grey.
    pub terracotta: bool,
    /// Solar water heaters on the roof.
    pub solar: bool,
    /// Storeys of a retail podium under a set-back shaft; 0 for none.
    pub podium_floors: u32,
    pub podium_storey_m: f32,
    /// How far the shaft is set back from the podium, metres.
    pub setback_m: f32,
    /// The podium's own bay: the mass draws it with `podium_*`.
    pub podium_unit_w: f32,
    pub podium_kind: u8,
    pub podium_window: [f32; 4],
}

/// The far form of `building`, or `None` for a kind the detail does not draw
/// from the facade grammar (sheds, villas, civic blocks): the mass keeps its
/// own plan-derived look for those. `podium` is the roof height and storey
/// count of the larger building it stands on, if any (see [`podiums_of`]).
pub fn far_look(building: &ModernBuilding, podium: Option<(f32, u16)>) -> Option<FarLook> {
    use super::{cn_commercial, cn_residential};
    if cn_residential::applies(building) {
        Some(cn_residential::far_look(building, podium))
    } else if cn_commercial::applies(building) {
        Some(cn_commercial::far_look(building, area_m2(&building.footprint), podium))
    } else {
        None
    }
}

/// Footprint area in square metres, from the plan's kilometre points.
fn area_m2(ring: &[urban::Point]) -> f32 {
    let mut twice = 0.0_f64;
    for i in 0..ring.len() {
        let a = &ring[i];
        let b = &ring[(i + 1) % ring.len()];
        twice += a.x_km as f64 * b.y_km as f64 - b.x_km as f64 * a.y_km as f64;
    }
    (twice.abs() * 0.5 * 1.0e6) as f32
}

fn inside(ring: &[urban::Point], x: f64, y: f64) -> bool {
    let mut inside = false;
    let mut j = ring.len().wrapping_sub(1);
    for i in 0..ring.len() {
        let (a, b) = (&ring[i], &ring[j]);
        if (a.y_km as f64 > y) != (b.y_km as f64 > y)
            && x < (b.x_km as f64 - a.x_km as f64) * (y - a.y_km as f64) / (b.y_km as f64 - a.y_km as f64)
                + a.x_km as f64
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// For each building, the podium it stands on, as the detail finds it
/// (`buildings::podiums_of`): the roof height and storey count of a larger,
/// lower building whose footprint contains its centre.
pub fn podiums_of(buildings: &[ModernBuilding]) -> Vec<Option<(f32, u16)>> {
    use super::{cn_commercial, cn_residential};
    let areas: Vec<f32> = buildings.iter().map(|b| area_m2(&b.footprint)).collect();
    let boxes: Vec<[f64; 4]> = buildings
        .iter()
        .map(|b| {
            let mut e = [f64::MAX, f64::MAX, f64::MIN, f64::MIN];
            for p in &b.footprint {
                e[0] = e[0].min(p.x_km as f64);
                e[1] = e[1].min(p.y_km as f64);
                e[2] = e[2].max(p.x_km as f64);
                e[3] = e[3].max(p.y_km as f64);
            }
            e
        })
        .collect();
    let mut table = crate::facades::cn_grammar::FacadeTable::new();
    buildings
        .iter()
        .enumerate()
        .map(|(index, building)| {
            if building.footprint.len() < 3 {
                return None;
            }
            let n = building.footprint.len() as f64;
            let cx = building.footprint.iter().map(|p| p.x_km as f64).sum::<f64>() / n;
            let cy = building.footprint.iter().map(|p| p.y_km as f64).sum::<f64>() / n;
            let mut best: Option<(f32, u16)> = None;
            for (other, under) in buildings.iter().enumerate() {
                let e = boxes[other];
                let contains = other != index
                    && under.floors < building.floors
                    && areas[other] > areas[index] * 1.15
                    && cx >= e[0]
                    && cx <= e[2]
                    && cy >= e[1]
                    && cy <= e[3]
                    && inside(&under.footprint, cx, cy);
                if !contains {
                    continue;
                }
                let top = if cn_residential::applies(under) {
                    cn_residential::roof_height(under, &mut table)
                } else if cn_commercial::applies(under) {
                    cn_commercial::roof_height_of_area(under, areas[other])
                } else {
                    continue;
                };
                if best.is_none_or(|(y, _)| top > y) {
                    best = Some((top, under.floors));
                }
            }
            best
        })
        .collect()
}

/// Add a `farLook` to every building of `payload` (the serialised
/// `Vec<ModernCity>` the world payload carries) that has one.
pub fn attach_far_looks(payload: &mut serde_json::Value, cities: &[urban::ModernCity]) {
    let Some(list) = payload.as_array_mut() else {
        return;
    };
    for (value, city) in list.iter_mut().zip(cities) {
        let Some(buildings) = value.get_mut("buildings").and_then(|b| b.as_array_mut()) else {
            continue;
        };
        let podiums = podiums_of(&city.buildings);
        for ((value, building), podium) in buildings.iter_mut().zip(&city.buildings).zip(podiums) {
            let (Some(object), Some(look)) = (value.as_object_mut(), far_look(building, podium)) else {
                continue;
            };
            if let Ok(look) = serde_json::to_value(look) {
                object.insert("farLook".to_string(), look);
            }
        }
    }
}

pub(crate) fn skin_code(skin: Skin) -> u8 {
    match skin {
        Skin::Paint => 0,
        Skin::Tile => 1,
        Skin::Stone => 2,
        Skin::Brick => 3,
        Skin::Panel => 4,
        Skin::Render => 5,
    }
}

pub(crate) fn glass_code(glass: Glass) -> u8 {
    match glass {
        Glass::Residential => 0,
        Glass::Tinted => 1,
        Glass::Office => 2,
        Glass::Curtain => 3,
    }
}

pub(crate) fn crown_code(crown: Crown) -> u8 {
    match crown {
        Crown::Flat => 0,
        Crown::Frame => 1,
        Crown::Hip => 2,
        Crown::Mansard => 3,
        Crown::NewChinese => 4,
    }
}

/// The principal window, the balcony and the curtain module of one elevation.
pub(crate) struct Bay {
    pub window: [f32; 4],
    pub balcony: [f32; 3],
    pub module: f32,
    pub kind: u8,
}

/// Read the layout of one flat off an elevation.
pub(crate) fn bay_of(elevation: &Elevation) -> Bay {
    let mut window = [0.0_f32; 4];
    let mut widest = 0.0_f32;
    let mut balcony = [0.0_f32; 3];
    let mut module = 0.0_f32;
    let mut kind = KIND_PUNCHED;
    let mut pier = 0.0_f32;
    for span in &elevation.spans {
        let w = span.x1 - span.x0;
        match span.part {
            Part::Window { sill, head, .. } if w > widest => {
                widest = w;
                window = [span.x0, span.x1, sill, head];
            }
            Part::Bay { .. } if w > widest => {
                widest = w;
                window = [
                    span.x0,
                    span.x1,
                    crate::facades::cn_grammar::BAY_SILL_M,
                    crate::facades::cn_grammar::bay_head(elevation.storey_m),
                ];
            }
            Part::Balcony { .. } | Part::Loggia { .. } if balcony[1] == 0.0 => {
                balcony = [
                    span.x0,
                    span.x1,
                    crate::facades::cn_grammar::door_head(elevation.storey_m),
                ];
            }
            Part::Curtain { module: m } => {
                module = m;
                kind = KIND_CURTAIN;
                window = [span.x0, span.x1, crate::facades::cn_grammar::SPANDREL_M, elevation.storey_m];
            }
            Part::Pier => pier = pier.max(w),
            _ => {}
        }
    }
    if kind != KIND_CURTAIN {
        if elevation.era == crate::facades::cn_grammar::Era::Office {
            if pier > 0.0 {
                kind = KIND_GRID;
                module = pier;
            } else {
                kind = KIND_RIBBON;
            }
        }
    }
    Bay {
        window,
        balcony,
        module,
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::facades::cn_grammar::{Era, OfficeSkin, office_elevation, residential_design};

    #[test]
    fn a_residential_flat_has_a_window_inside_its_width() {
        for seed in 0..40_u32 {
            let design = residential_design(seed * 977, Era::Modern, 18, false);
            let bay = bay_of(&design.front);
            assert!(bay.window[1] > bay.window[0], "no window found for seed {seed}");
            assert!(bay.window[1] <= design.front.width + 1e-3);
            assert!(bay.window[3] > bay.window[2]);
            assert!(bay.window[3] <= design.front.storey_m + 1e-3);
        }
    }

    #[test]
    fn office_skins_map_to_their_kinds() {
        let cladding = [0.5, 0.5, 0.5];
        let curtain = bay_of(&office_elevation(5, OfficeSkin::Curtain, cladding));
        assert_eq!(curtain.kind, KIND_CURTAIN);
        assert!(curtain.module >= 1.2 && curtain.module <= 1.8);
        let grid = bay_of(&office_elevation(5, OfficeSkin::Grid, cladding));
        assert_eq!(grid.kind, KIND_GRID);
        let ribbon = bay_of(&office_elevation(5, OfficeSkin::Ribbon, cladding));
        assert_eq!(ribbon.kind, KIND_RIBBON);
    }
}
