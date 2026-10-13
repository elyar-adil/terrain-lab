//! Commercial buildings of a `ChineseModern` city: the glass office tower on
//! its podium, the stone-grid office or hotel, and the panel-and-ribbon-window
//! 商业楼 that lines every main road, all over a continuous shop storey.
//!
//! Their elevations come from the facade grammar (`office_elevation`), so the
//! curtain-wall module, the grid's pier and window, the ribbon's proportions
//! and the glass coating differ from building to building.  The relief that
//! sells each type is built, not drawn: the fins of a curtain wall, the
//! projecting piers and sills of a stone grid, the spandrel bands of a ribbon
//! block.  The wall's vertex colour is the stone or panel colour; a curtain
//! wall's glass coating is in its elevation row.

use urban::{ModernBuilding, ParcelUse};

use super::cn_kit::{
    PAINT, TRIM, WallMap, edge_out, facade_wall, face, ground_shops, parapet, projection, wall_tint,
};
use super::{hash_u32, level, roofscape};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_grammar::{Elevation, FACADE, FacadeTable, OfficeSkin, Part, SPANDREL_M};
use crate::math::{Vec2, Vec3, inset_ring, ring_centroid, signed_area};
use crate::mesh::MeshBuilder;

/// Stone and panel colours, final linear reflectance: beige and grey granite,
/// white and champagne aluminium, and the odd dark-grey panel block.
const CLADDING: [([f32; 3], f32); 6] = [
    ([0.56, 0.51, 0.43], 3.0),
    ([0.58, 0.58, 0.56], 2.5),
    ([0.70, 0.70, 0.68], 2.0),
    ([0.60, 0.54, 0.44], 1.5),
    ([0.48, 0.42, 0.38], 1.0),
    ([0.24, 0.25, 0.26], 1.0),
];

fn pick(table: &[([f32; 3], f32)], roll: f32) -> [f32; 3] {
    let total: f32 = table.iter().map(|(_, w)| w).sum();
    let mut left = roll * total;
    for (colour, weight) in table {
        if left < *weight {
            return *colour;
        }
        left -= weight;
    }
    table[0].0
}

/// Whether this module builds a building.
pub(crate) fn applies(building: &ModernBuilding) -> bool {
    building.use_type == ParcelUse::Commercial && building.floors >= 2
}

fn skin_for(id: u32, floors: u16) -> OfficeSkin {
    let roll = hash_u32(id, 301, 5);
    if floors >= 12 {
        if roll < 0.55 {
            OfficeSkin::Curtain
        } else {
            OfficeSkin::Grid
        }
    } else if roll < 0.40 {
        OfficeSkin::Ribbon
    } else if roll < 0.70 {
        OfficeSkin::Grid
    } else {
        OfficeSkin::Curtain
    }
}

/// How a commercial building's storeys are stacked, with its elevations.
struct Stack {
    /// Where the first storey above the shop storey (or the podium roof) is.
    base: f32,
    /// Its own retail podium under a stepped-back shaft, if any.
    podium: u32,
    podium_skin: (Elevation, u8),
    shaft: u32,
    skin: OfficeSkin,
    shaft_skin: (Elevation, u8),
}

impl Stack {
    fn of(
        building: &ModernBuilding,
        area: f32,
        stands_on: Option<(f32, u16)>,
        cladding: [f32; 3],
        table: &mut FacadeTable,
    ) -> Self {
        let id = building.id;
        let floors = building.floors.max(2);
        let skin = skin_for(id, floors);
        let podium_kind = if skin == OfficeSkin::Curtain {
            OfficeSkin::Grid
        } else {
            OfficeSkin::Ribbon
        };
        let shaft_skin = table.office(id, skin, cladding);
        let podium_skin = table.office(id ^ 0x7777, podium_kind, cladding);
        if let Some((top, below)) = stands_on {
            return Self {
                base: top,
                podium: 0,
                podium_skin,
                shaft: (floors.saturating_sub(below) as u32).max(2),
                skin,
                shaft_skin,
            };
        }
        let upper = (floors - 1) as u32;
        let podium = if floors >= 12 && area > 500.0 {
            (2 + (hash_u32(id, 307, 5) * 3.0) as u32).min(upper.saturating_sub(6))
        } else {
            0
        };
        Self {
            base: level::GROUND + GROUND_STOREY_M,
            podium,
            podium_skin,
            shaft: (upper - podium).max(1),
            skin,
            shaft_skin,
        }
    }

    fn podium_top(&self) -> f32 {
        self.base + self.podium as f32 * self.podium_skin.0.storey_m
    }

    fn top(&self) -> f32 {
        self.podium_top() + self.shaft as f32 * self.shaft_skin.0.storey_m
    }
}

fn cladding_of(id: u32) -> [f32; 3] {
    pick(&CLADDING, hash_u32(id, 303, 5))
}

/// The roof deck height of a commercial building standing on the ground.
pub(crate) fn roof_height(
    building: &ModernBuilding,
    ring: &[Vec2],
    table: &mut FacadeTable,
) -> f32 {
    let area = signed_area(&inset_ring(ring, 0.3)).abs();
    Stack::of(building, area, None, cladding_of(building.id), table).top()
}

/// Build one commercial building.  `podium` is the roof height and storey
/// count of a larger, lower building it stands on, if any.  Returns the roof
/// deck height.
pub(crate) fn shell(
    building: &ModernBuilding,
    ring: &[Vec2],
    podium: Option<(f32, u16)>,
    table: &mut FacadeTable,
    builder: &mut MeshBuilder,
) -> f32 {
    let id = building.id;
    let floors = building.floors.max(2);
    let mut outward = ring.to_vec();
    if signed_area(&outward) > 0.0 {
        outward.reverse();
    }
    let plan = inset_ring(&outward, 0.3);
    let area = signed_area(&plan).abs();
    let stone = cladding_of(id);
    let stack = Stack::of(building, area, podium, stone, table);
    let tower = floors >= 12;

    if podium.is_none() {
        for index in 0..plan.len() {
            let a = plan[index];
            let b = plan[(index + 1) % plan.len()];
            builder.wall("wall.render", a, b, level::PLINTH, level::GROUND, None);
        }
        ground_shops(&plan, building, builder);
    }
    let mut base = stack.base;
    let mut shaft_ring = plan.clone();
    if stack.podium > 0 {
        let kind = if stack.skin == OfficeSkin::Curtain {
            OfficeSkin::Grid
        } else {
            OfficeSkin::Ribbon
        };
        let podium_top = dress(
            &plan,
            kind,
            &stack.podium_skin,
            base,
            stack.podium,
            stone,
            id,
            builder,
        );
        builder.ground_uv("roof", &plan, podium_top, None);
        parapet(&plan, podium_top, 1.1, stone, builder);
        let inset = inset_ring(&plan, 3.0);
        if signed_area(&inset).abs() > 200.0 {
            shaft_ring = inset;
        }
        base = podium_top;
    }
    let top = dress(
        &shaft_ring,
        stack.skin,
        &stack.shaft_skin,
        base,
        stack.shaft,
        stone,
        id,
        builder,
    );

    builder.ground_uv("roof", &shaft_ring, top, None);
    let parapet_colour = if stack.skin == OfficeSkin::Curtain {
        [0.30, 0.31, 0.32]
    } else {
        stone
    };
    parapet(
        &shaft_ring,
        top,
        if tower { 1.6 } else { 1.1 },
        parapet_colour,
        builder,
    );
    if signed_area(&shaft_ring).abs() > 90.0 {
        roofscape::roofscape(building, false, &shaft_ring, top, builder);
    }
    if tower {
        plant_screen(&shaft_ring, top, builder);
    }
    let foot = podium.map_or(level::GROUND, |(y, _)| y);
    super::details::downpipe(&plan, foot, top, id, builder);
    top
}

/// Wrap a ring in an elevation for `storeys` storeys from `base`, with the
/// relief its skin calls for.  Returns the height of its top.
#[allow(clippy::too_many_arguments)]
fn dress(
    ring: &[Vec2],
    skin: OfficeSkin,
    (elevation, row): &(Elevation, u8),
    base: f32,
    storeys: u32,
    stone: [f32; 3],
    id: u32,
    builder: &mut MeshBuilder,
) -> f32 {
    let storey_m = elevation.storey_m;
    let top = base + storeys as f32 * storey_m;
    let tint = wall_tint(stone);
    let centre = ring_centroid(ring);
    let fins = skin == OfficeSkin::Curtain && hash_u32(id, 311, 5) < 0.6;
    let white_fins = hash_u32(id, 313, 5) < 0.5;
    let mut u_run = 0.0;
    for index in 0..ring.len() {
        let a = ring[index];
        let b = ring[(index + 1) % ring.len()];
        let length = a.distance(b);
        if length < 0.3 {
            continue;
        }
        let along = (b - a) / length;
        let out = edge_out(a, b, centre);
        let units = (length / elevation.width).round().max(1.0) as u32;
        let map = WallMap {
            a,
            along,
            out,
            u0: u_run,
            u_per_m: units as f32 * elevation.width / length,
            units,
            storey_m,
        };
        u_run += units as f32 * elevation.width;
        let u1 = map.u0 + length * map.u_per_m;
        facade_wall(builder, &map, b, base, 0, storeys, *row, tint);
        match skin {
            OfficeSkin::Curtain if fins && length > 6.0 => {
                // A fin on every other mullion, full height: the vertical
                // grain of a curtain-walled tower from across the city.
                let module = elevation
                    .spans
                    .iter()
                    .find_map(|span| match span.part {
                        Part::Curtain { module } => Some(module),
                        _ => None,
                    })
                    .unwrap_or(1.5);
                let pitch = module * 2.0;
                let count = (length * map.u_per_m / pitch).floor() as u32;
                for k in 1..count {
                    let p = map.point(map.u0 + k as f32 * pitch);
                    let material = if white_fins { TRIM } else { "trim.dark" };
                    projection(
                        builder,
                        material,
                        None,
                        p - along * 0.06,
                        p + along * 0.06,
                        out,
                        0.38,
                        base,
                        top + 0.6,
                        None,
                    );
                }
            }
            OfficeSkin::Grid => {
                // Projecting piers on every pier span, and a sill ledge under
                // every window: the deep grid of a stone office.
                for k in 0..units {
                    let tile_u = map.u0 + k as f32 * elevation.width;
                    let mirrored = k % 2 == 1;
                    for span in &elevation.spans {
                        let (x0, x1) = if mirrored {
                            (elevation.width - span.x1, elevation.width - span.x0)
                        } else {
                            (span.x0, span.x1)
                        };
                        let p0 = map.point(tile_u + x0);
                        let p1 = map.point(tile_u + x1);
                        match span.part {
                            Part::Pier => projection(
                                builder,
                                PAINT,
                                Some(stone),
                                p0,
                                p1,
                                out,
                                0.30,
                                base,
                                top,
                                None,
                            ),
                            Part::Window { sill, .. } => {
                                for s in 0..storeys {
                                    let y = base + s as f32 * storey_m + sill;
                                    projection(
                                        builder,
                                        PAINT,
                                        Some(stone),
                                        p0,
                                        p1,
                                        out,
                                        0.16,
                                        y - 0.14,
                                        y,
                                        None,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            OfficeSkin::Ribbon => {
                // The spandrel band of every floor stands proud of the glazing.
                for s in 0..storeys {
                    let y = base + s as f32 * storey_m;
                    projection(
                        builder,
                        PAINT,
                        Some(stone),
                        a,
                        b,
                        out,
                        0.14,
                        y - 0.40,
                        y + 1.0,
                        None,
                    );
                }
            }
            OfficeSkin::Curtain => {}
        }
        // A curtain wall's top: the glass runs on past the roof as a screen,
        // drawn as the spandrel band of a storey above the last.
        if skin == OfficeSkin::Curtain {
            let screen = 1.2;
            let v0 = storeys as f32;
            let v1 = v0 + SPANDREL_M.min(screen) / storey_m;
            builder.facade_row = Some(*row);
            face(
                builder,
                FACADE,
                [
                    Vec3::from_plan(a, top),
                    Vec3::from_plan(b, top),
                    Vec3::from_plan(b, top + screen),
                    Vec3::from_plan(a, top + screen),
                ],
                Vec3::new(out.x, 0.0, out.y),
                Some([(map.u0, v0), (u1, v0), (u1, v1), (map.u0, v1)]),
                Some(tint),
            );
            builder.facade_row = None;
        }
    }
    top
}

/// The louvred screen round a tower's roof plant.
fn plant_screen(ring: &[Vec2], top: f32, builder: &mut MeshBuilder) {
    let screen = inset_ring(ring, 3.5);
    if signed_area(&screen).abs() < 60.0 {
        return;
    }
    let h = 3.2;
    let centre = ring_centroid(&screen);
    for index in 0..screen.len() {
        let a = screen[index];
        let b = screen[(index + 1) % screen.len()];
        let length = a.distance(b);
        if length < 1.0 {
            continue;
        }
        let out = edge_out(a, b, centre);
        // Louvre blades every 300 mm: thin boxes, which throw the stripe of
        // shadow a real screen does.
        let blades = (h / 0.3) as u32;
        for k in 0..blades {
            let y = top + 0.2 + k as f32 * 0.3;
            projection(builder, TRIM, None, a, b, out, 0.12, y, y + 0.16, None);
        }
        face(
            builder,
            "trim.dark",
            [
                Vec3::from_plan(a, top),
                Vec3::from_plan(b, top),
                Vec3::from_plan(b, top + h),
                Vec3::from_plan(a, top + h),
            ],
            Vec3::new(out.x, 0.0, out.y),
            None,
            None,
        );
    }
}
