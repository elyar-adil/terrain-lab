use std::f32::consts::TAU;

use crate::math::{centroid, hash01, scale_polygon};
use crate::{
    BuildingMass, CitySpec, CityStyle, Point, RoofStyle, StreetClass, StreetSegment, UrbanBlock,
    UrbanModel,
};

pub trait UrbanGenerator {
    fn generate(&self, spec: CitySpec) -> UrbanModel;
}

pub struct ParisianGenerator;
pub struct BarcelonaGenerator;
pub struct ManhattanGenerator;

impl UrbanGenerator for BarcelonaGenerator {
    fn generate(&self, spec: CitySpec) -> UrbanModel {
        let spacing = 0.133;
        let half = (spec.radius_km / spacing).floor() as i32;
        let mut streets = Vec::new();
        let mut blocks = Vec::new();
        let mut buildings = Vec::new();
        for index in -half..=half {
            let offset = index as f32 * spacing;
            streets.push(street(
                spec,
                Point {
                    x_km: -spec.radius_km,
                    y_km: offset,
                },
                Point {
                    x_km: spec.radius_km,
                    y_km: offset,
                },
                StreetClass::Street,
                20.0,
            ));
            streets.push(street(
                spec,
                Point {
                    x_km: offset,
                    y_km: -spec.radius_km,
                },
                Point {
                    x_km: offset,
                    y_km: spec.radius_km,
                },
                StreetClass::Street,
                20.0,
            ));
        }
        for y in -half..half {
            for x in -half..half {
                let cx = (x as f32 + 0.5) * spacing;
                let cy = (y as f32 + 0.5) * spacing;
                if cx.hypot(cy) > spec.radius_km * 0.96 {
                    continue;
                }
                let block = chamfered(spec, cx, cy, spacing * 0.82, spacing * 0.82, 0.018);
                let courtyard = scale_polygon(&block, centroid(&block), 0.52);
                blocks.push(UrbanBlock {
                    boundary: block.clone(),
                    courtyard: Some(courtyard.clone()),
                });
                buildings.push(BuildingMass {
                    footprint: block,
                    courtyard: Some(courtyard),
                    height_metres: 18.0 + hash01(spec.seed, x, y) * 9.0,
                    roof: RoofStyle::Terracotta,
                });
            }
        }
        UrbanModel {
            style: CityStyle::BarcelonaEixample,
            streets,
            blocks,
            buildings,
        }
    }
}

impl UrbanGenerator for ManhattanGenerator {
    fn generate(&self, spec: CitySpec) -> UrbanModel {
        let avenue_spacing = 0.285;
        let street_spacing = 0.086;
        let x_count = (spec.radius_km / avenue_spacing).ceil() as i32;
        let y_count = (spec.radius_km / street_spacing).ceil() as i32;
        let mut streets = Vec::new();
        for x in -x_count..=x_count {
            let offset = x as f32 * avenue_spacing;
            streets.push(street(
                spec,
                Point {
                    x_km: offset,
                    y_km: -spec.radius_km,
                },
                Point {
                    x_km: offset,
                    y_km: spec.radius_km,
                },
                StreetClass::Avenue,
                30.0,
            ));
        }
        for y in -y_count..=y_count {
            let offset = y as f32 * street_spacing;
            streets.push(street(
                spec,
                Point {
                    x_km: -spec.radius_km,
                    y_km: offset,
                },
                Point {
                    x_km: spec.radius_km,
                    y_km: offset,
                },
                StreetClass::Street,
                18.0,
            ));
        }
        let mut blocks = Vec::new();
        let mut buildings = Vec::new();
        for y in -y_count..y_count {
            for x in -x_count..x_count {
                let cx = (x as f32 + 0.5) * avenue_spacing;
                let cy = (y as f32 + 0.5) * street_spacing;
                if cx.hypot(cy) > spec.radius_km {
                    continue;
                }
                let boundary =
                    rectangle(spec, cx, cy, avenue_spacing * 0.86, street_spacing * 0.70);
                blocks.push(UrbanBlock {
                    boundary: boundary.clone(),
                    courtyard: None,
                });
                for parcel in 0..4 {
                    let parcel_cx =
                        cx - avenue_spacing * 0.30 + parcel as f32 * avenue_spacing * 0.20;
                    let footprint = rectangle(
                        spec,
                        parcel_cx,
                        cy,
                        avenue_spacing * 0.16,
                        street_spacing * 0.58,
                    );
                    let centrality =
                        1.0 - (cx.hypot(cy) / spec.radius_km.max(0.01)).clamp(0.0, 1.0);
                    buildings.push(BuildingMass {
                        footprint,
                        courtyard: None,
                        height_metres: 18.0
                            + centrality.powf(1.7) * spec.density * 210.0
                            + hash01(spec.seed, x * 7 + parcel, y) * 22.0,
                        roof: if centrality > 0.55 {
                            RoofStyle::SetbackTower
                        } else {
                            RoofStyle::Flat
                        },
                    });
                }
            }
        }
        UrbanModel {
            style: CityStyle::Manhattan,
            streets,
            blocks,
            buildings,
        }
    }
}

impl UrbanGenerator for ParisianGenerator {
    fn generate(&self, spec: CitySpec) -> UrbanModel {
        // Haussmann boulevards are only the primary skeleton. Treating the
        // space between nine rays as one building produced wedges hundreds of
        // metres wide. Secondary streets keep ordinary Parisian blocks in the
        // roughly 40--220 m range while every fourth ray remains a boulevard.
        let ray_count = 36;
        let ring_spacing = 0.18;
        let rings = (spec.radius_km / ring_spacing).ceil() as usize;
        let mut streets = Vec::new();
        for ray in 0..ray_count {
            let angle = ray as f32 / ray_count as f32 * TAU + hash01(spec.seed, ray, 3) * 0.16;
            streets.push(street(
                spec,
                Point {
                    x_km: 0.0,
                    y_km: 0.0,
                },
                Point {
                    x_km: angle.cos() * spec.radius_km,
                    y_km: angle.sin() * spec.radius_km,
                },
                if ray % 4 == 0 {
                    StreetClass::Boulevard
                } else {
                    StreetClass::Street
                },
                if ray % 4 == 0 { 30.0 } else { 14.0 },
            ));
        }
        for ring in 1..=rings {
            let radius = ring as f32 * ring_spacing;
            let segments = 28;
            for segment in 0..segments {
                let a0 = segment as f32 / segments as f32 * TAU;
                let a1 = (segment + 1) as f32 / segments as f32 * TAU;
                streets.push(street(
                    spec,
                    polar(radius, a0),
                    polar(radius, a1),
                    StreetClass::Street,
                    14.0,
                ));
            }
        }
        let mut blocks = Vec::new();
        let mut buildings = Vec::new();
        for ring in 0..rings {
            let inner = ring as f32 * ring_spacing + 0.018;
            let outer = ((ring + 1) as f32 * ring_spacing - 0.018).min(spec.radius_km);
            for sector in 0..ray_count {
                let jitter = (hash01(spec.seed, ring as i32, sector as i32) - 0.5) * 0.012;
                // Angular setbacks correspond to a physical 7 m gap at the
                // block midpoint, rather than expanding with distance.
                let mean_radius = ((inner + outer) * 0.5).max(0.04);
                let angular_gap = (0.007 / mean_radius).min(TAU / ray_count as f32 * 0.28);
                let a0 = sector as f32 / ray_count as f32 * TAU + angular_gap + jitter;
                let a1 = (sector + 1) as f32 / ray_count as f32 * TAU - angular_gap + jitter;
                let boundary = vec![
                    transform(spec, polar(inner, a0)),
                    transform(spec, polar(inner, a1)),
                    transform(spec, polar(outer, a1)),
                    transform(spec, polar(outer, a0)),
                ];
                let courtyard = scale_polygon(&boundary, centroid(&boundary), 0.58);
                blocks.push(UrbanBlock {
                    boundary: boundary.clone(),
                    courtyard: Some(courtyard.clone()),
                });
                buildings.push(BuildingMass {
                    footprint: boundary,
                    courtyard: Some(courtyard),
                    height_metres: 17.0 + hash01(spec.seed, ring as i32, sector as i32) * 10.0,
                    roof: RoofStyle::Mansard,
                });
            }
        }
        UrbanModel {
            style: CityStyle::Parisian,
            streets,
            blocks,
            buildings,
        }
    }
}

/// Generate the compact, hierarchical street morphology used by a modern
/// Chinese city.  The SD graph is intentionally coarse and deterministic;
/// HD roads retain physical widths and centre lines for renderers.  Blocks
/// are then split into parcels and building metadata without relying on a
/// raster resolution, so callers can zoom indefinitely.
fn street(
    spec: CitySpec,
    from: Point,
    to: Point,
    class: StreetClass,
    width_metres: f32,
) -> StreetSegment {
    StreetSegment {
        from: transform(spec, from),
        to: transform(spec, to),
        class,
        width_metres,
    }
}

fn transform(spec: CitySpec, point: Point) -> Point {
    let c = spec.rotation_radians.cos();
    let s = spec.rotation_radians.sin();
    Point {
        x_km: spec.centre.x_km + point.x_km * c - point.y_km * s,
        y_km: spec.centre.y_km + point.x_km * s + point.y_km * c,
    }
}

fn polar(radius: f32, angle: f32) -> Point {
    Point {
        x_km: radius * angle.cos(),
        y_km: radius * angle.sin(),
    }
}

fn rectangle(spec: CitySpec, cx: f32, cy: f32, width: f32, height: f32) -> Vec<Point> {
    [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
        .into_iter()
        .map(|(x, y)| {
            transform(
                spec,
                Point {
                    x_km: cx + x * width,
                    y_km: cy + y * height,
                },
            )
        })
        .collect()
}

fn chamfered(spec: CitySpec, cx: f32, cy: f32, width: f32, height: f32, cut: f32) -> Vec<Point> {
    let hw = width * 0.5;
    let hh = height * 0.5;
    [
        (-hw + cut, -hh),
        (hw - cut, -hh),
        (hw, -hh + cut),
        (hw, hh - cut),
        (hw - cut, hh),
        (-hw + cut, hh),
        (-hw, hh - cut),
        (-hw, -hh + cut),
    ]
    .into_iter()
    .map(|(x, y)| {
        transform(
            spec,
            Point {
                x_km: cx + x,
                y_km: cy + y,
            },
        )
    })
    .collect()
}
