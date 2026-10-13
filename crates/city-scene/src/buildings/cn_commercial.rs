//! Commercial buildings of a `ChineseModern` city: the glass office tower on
//! its podium, the stone-grid office or hotel, and the panel-and-ribbon-window
//! 商业楼 that lines every main road, all over a continuous shop storey.
//!
//! They wear the `cncom/*` elevations of [`crate::facades::cn_elevation`], and
//! the relief that sells each type is built, not painted: the fins of a
//! curtain wall, the projecting piers and sills of a stone grid, the spandrel
//! bands of a ribbon block.  Colour comes from the wall's vertex colour — the
//! stone or panel colour on a grid or ribbon facade, the coating colour of the
//! glass on a curtain wall.

use urban::{ModernBuilding, ParcelUse};

use super::cn_residential::{PAINT, TRIM, WallMap, face, ground_shops, parapet, projection};
use super::{hash_u32, level, roofscape};
use crate::facades::GROUND_STOREY_M;
use crate::facades::cn_elevation::{Layout, PAINT_BASE, Part, SPANDREL_M, layout};
use crate::math::{Vec2, Vec3, inset_ring, ring_centroid, signed_area};
use crate::mesh::MeshBuilder;

/// Stone and panel colours, final linear reflectance: beige and grey granite,
/// white and champagne aluminium, and the odd dark-grey panel block.
const CLADDING: [([f32; 3], f32); 6] = [
    ([0.56, 0.51, 0.43], 3.0), // beige stone
    ([0.58, 0.58, 0.56], 2.5), // light grey granite
    ([0.70, 0.70, 0.68], 2.0), // white panel
    ([0.60, 0.54, 0.44], 1.5), // champagne panel
    ([0.48, 0.42, 0.38], 1.0), // warm granite
    ([0.24, 0.25, 0.26], 1.0), // dark grey panel
];

/// Curtain-wall coatings, as the colour the glass reflects: blue-green, blue,
/// silver, dark grey and green — the five every Chinese CBD is built from.
const COATING: [([f32; 3], f32); 5] = [
    ([0.42, 0.66, 0.66], 3.0),
    ([0.38, 0.52, 0.74], 2.5),
    ([0.66, 0.70, 0.72], 2.0),
    ([0.30, 0.33, 0.36], 1.5),
    ([0.44, 0.62, 0.48], 1.0),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Skin {
    Curtain,
    Grid,
    Ribbon,
}

impl Skin {
    fn layout(self) -> &'static Layout {
        layout(match self {
            Skin::Curtain => "cncom/curtain",
            Skin::Grid => "cncom/grid",
            Skin::Ribbon => "cncom/ribbon",
        })
        .expect("commercial layouts are in the table")
    }
}

fn skin_for(id: u32, floors: u16) -> Skin {
    let roll = hash_u32(id, 301, 5);
    if floors >= 12 {
        if roll < 0.55 {
            Skin::Curtain
        } else {
            Skin::Grid
        }
    } else if roll < 0.40 {
        Skin::Ribbon
    } else if roll < 0.70 {
        Skin::Grid
    } else {
        Skin::Curtain
    }
}

/// How a commercial building's storeys are stacked.
struct Stack {
    /// Where the first storey above the shop storey (or the podium roof) is.
    base: f32,
    /// Storeys of its own retail podium, under a stepped-back shaft.
    podium: u32,
    podium_skin: Skin,
    /// Storeys of the shaft.
    shaft: u32,
    skin: Skin,
}

impl Stack {
    fn of(building: &ModernBuilding, area: f32, stands_on: Option<(f32, u16)>) -> Self {
        let id = building.id;
        let floors = building.floors.max(2);
        let skin = skin_for(id, floors);
        let podium_skin = if skin == Skin::Curtain {
            Skin::Grid
        } else {
            Skin::Ribbon
        };
        if let Some((top, below)) = stands_on {
            return Self {
                base: top,
                podium: 0,
                podium_skin,
                shaft: (floors.saturating_sub(below) as u32).max(2),
                skin,
            };
        }
        let upper = (floors - 1) as u32;
        // A tower stands on a podium of two to four retail storeys and steps
        // its shaft back from it, if the plot is big enough to step.
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
        }
    }

    fn podium_top(&self) -> f32 {
        self.base + self.podium as f32 * self.podium_skin.layout().storey_m
    }

    fn top(&self) -> f32 {
        self.podium_top() + self.shaft as f32 * self.skin.layout().storey_m
    }
}

/// The roof deck height of a commercial building standing on the ground.
pub(crate) fn roof_height(building: &ModernBuilding, ring: &[Vec2]) -> f32 {
    let area = signed_area(&inset_ring(ring, 0.3)).abs();
    Stack::of(building, area, None).top()
}

/// Build one commercial building.  `podium` is the roof height and storey
/// count of a larger, lower building it stands on, if any.  Returns the roof
/// deck height.
pub(crate) fn shell(
    building: &ModernBuilding,
    ring: &[Vec2],
    podium: Option<(f32, u16)>,
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
    let stack = Stack::of(building, area, podium);
    let skin = stack.skin;
    let tower = floors >= 12;
    let stone = pick(&CLADDING, hash_u32(id, 303, 5));
    let coating = pick(&COATING, hash_u32(id, 305, 5));

    if podium.is_none() {
        // The shop storey, all round.
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
        let podium_top = dress(
            &plan,
            stack.podium_skin,
            base,
            stack.podium,
            stone,
            coating,
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
    let upper = stack.shaft;
    let top = dress(&shaft_ring, skin, base, upper, stone, coating, id, builder);

    // The roof: a parapet, the plant, and on a tower a louvred plant screen.
    builder.ground_uv("roof", &shaft_ring, top, None);
    let parapet_colour = if skin == Skin::Curtain {
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
    super::details::downpipe(
        &plan,
        podium.map_or(level::GROUND, |(y, _)| y),
        top,
        id,
        builder,
    );
    top
}

/// Wrap a ring in an elevation for `storeys` storeys from `base`, with the
/// relief its skin calls for.  Returns the height of its top.
#[allow(clippy::too_many_arguments)]
fn dress(
    ring: &[Vec2],
    skin: Skin,
    base: f32,
    storeys: u32,
    stone: [f32; 3],
    coating: [f32; 3],
    id: u32,
    builder: &mut MeshBuilder,
) -> f32 {
    let layout = skin.layout();
    let storey_m = layout.storey_m;
    let top = base + storeys as f32 * storey_m;
    let tint = match skin {
        Skin::Curtain => coating,
        _ => [
            (stone[0] / PAINT_BASE).min(1.0),
            (stone[1] / PAINT_BASE).min(1.0),
            (stone[2] / PAINT_BASE).min(1.0),
        ],
    };
    let centre = ring_centroid(ring);
    let fins = skin == Skin::Curtain && hash_u32(id, 311, 5) < 0.6;
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
        let mut out = Vec2::new(along.y, -along.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
        let units = (length / layout.width).round().max(1.0) as u32;
        let map = WallMap {
            a,
            along,
            out,
            u0: u_run,
            u_per_m: units as f32 * layout.width / length,
            units,
            storeys: storeys as f32,
            storey_m,
        };
        u_run += units as f32 * layout.width;
        let u1 = map.u0 + length * map.u_per_m;
        let vb = storeys as f32 / 4.0;
        builder.quad_uv(
            layout.key,
            Vec3::from_plan(a, base),
            Vec3::from_plan(b, base),
            Vec3::from_plan(b, top),
            Vec3::from_plan(a, top),
            [(map.u0, vb), (u1, vb), (u1, 0.0), (map.u0, 0.0)],
            Some(tint),
        );
        match skin {
            Skin::Curtain if fins && length > 6.0 => {
                // A fin on every other mullion, full height: the vertical
                // grain of a curtain-walled tower from across the city.
                let pitch = 3.0;
                let count = (length * map.u_per_m / pitch).floor() as u32;
                for k in 1..count {
                    let p = map.point(map.u0 + k as f32 * pitch);
                    let (material, colour) = if white_fins {
                        (TRIM, None)
                    } else {
                        ("trim.dark", None)
                    };
                    projection(
                        builder,
                        material,
                        colour,
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
            Skin::Grid => {
                // Projecting piers on every pier span, and a sill ledge under
                // every window: the deep grid of a stone office.
                for k in 0..units {
                    let tile_u = map.u0 + k as f32 * layout.width;
                    for span in layout.spans {
                        let p0 = map.point(tile_u + span.x0);
                        let p1 = map.point(tile_u + span.x1);
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
            Skin::Ribbon => {
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
                        y + 1.05,
                        None,
                    );
                }
            }
            Skin::Curtain => {}
        }
        // A curtain wall's top: the glass runs past the roof as a screen.
        if skin == Skin::Curtain {
            let screen = 1.2;
            face(
                builder,
                layout.key,
                [
                    Vec3::from_plan(a, top),
                    Vec3::from_plan(b, top),
                    Vec3::from_plan(b, top + screen),
                    Vec3::from_plan(a, top + screen),
                ],
                Vec3::new(out.x, 0.0, out.y),
                Some([
                    (map.u0, SPANDREL_M / (4.0 * storey_m)),
                    (u1, SPANDREL_M / (4.0 * storey_m)),
                    (u1, 0.0),
                    (map.u0, 0.0),
                ]),
                Some(tint),
            );
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
        let along = (b - a) / length;
        let mut out = Vec2::new(along.y, -along.x);
        if out.dot((a + b) * 0.5 - centre) < 0.0 {
            out = -out;
        }
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
