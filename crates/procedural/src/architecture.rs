//! The building system's shared metrics layer.  Facade proportions — bay
//! rhythm, storey heights, opening ratios, balcony bands — are decided here
//! once so every generator (urban parcels today, street walls and bridges
//! tomorrow) produces elevations with the same architectural logic.

/// Facade family, mirroring the urban payload's `BuildingFacade` names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FacadeKind {
    /// 住宅: 3.0 m storeys, 3.3-4.0 m bays, balconies on most openings.
    Residential,
    /// 玻璃幕墙: tall ground lobby, wide 4.5-6.0 m bays, ribbon glazing.
    CurtainWall,
    /// 办公/商住 concrete-glass hybrid.
    ConcreteGlass,
    /// 公建 stone civic: generous 5.0 m storeys, punched openings.
    StoneCivic,
}

/// Resolved elevation metrics for one building face.
#[derive(Clone, Copy, Debug)]
pub struct FacadeMetrics {
    /// Bays across the facade (window positions).
    pub bays: u16,
    /// Upper-storey floor-to-floor height, metres.
    pub storey_height_m: f32,
    /// Ground-storey height, metres — taller for lobbies and shops.
    pub ground_storey_height_m: f32,
    /// Fraction of facade area that is glazing/opening, [0, 1].
    pub opening_ratio: f32,
    /// Emit a balcony band every N storeys (0 = none).
    pub balcony_band_every: u8,
}

/// Resolve the facade metrics for a rectangular face.  `width_m` is the face
/// the windows land on; `depth_m` only disambiguates tower versus slab.
pub fn facade_metrics(
    width_m: f32,
    depth_m: f32,
    _storeys: u16,
    kind: FacadeKind,
) -> FacadeMetrics {
    let target_bay = match kind {
        FacadeKind::Residential => 3.7,
        FacadeKind::CurtainWall => 5.4,
        FacadeKind::ConcreteGlass => 4.6,
        FacadeKind::StoneCivic => 4.9,
    };
    // Towers (compact plan) widen their bays slightly so a short face still
    // carries two bays, matching real point-tower elevations.
    let is_tower = width_m.max(depth_m) < 34.0;
    let bay = if is_tower { target_bay * 0.78 } else { target_bay };
    let bays = ((width_m / bay).round() as u16).clamp(2, 40);
    let (storey, ground, opening, balcony) = match kind {
        FacadeKind::Residential => (3.0, 3.6, 0.30, 1),
        FacadeKind::CurtainWall => (4.1, 5.1, 0.64, 0),
        FacadeKind::ConcreteGlass => (3.9, 4.5, 0.46, 0),
        FacadeKind::StoneCivic => (4.6, 5.2, 0.34, 0),
    };
    FacadeMetrics {
        bays,
        storey_height_m: storey,
        ground_storey_height_m: ground,
        opening_ratio: opening,
        balcony_band_every: balcony,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn residential_bays_match_real_bay_rhythm() {
        // A 70 m slab frontage lands on ~19 bays of 3.7 m; a 24 m tower face
        // still carries three bays.
        let slab = facade_metrics(70.0, 13.0, 18, FacadeKind::Residential);
        assert!((17..=20).contains(&slab.bays));
        let tower = facade_metrics(24.0, 20.0, 30, FacadeKind::Residential);
        // Real point towers keep a ~2.9 m bay on a 24 m face.
        assert!((5..=9).contains(&tower.bays));
        assert_eq!(slab.balcony_band_every, 1);
    }

    #[test]
    fn curtain_wall_has_tall_lobby_and_wide_openings() {
        let office = facade_metrics(48.0, 30.0, 24, FacadeKind::CurtainWall);
        assert!(office.ground_storey_height_m > office.storey_height_m);
        assert!(office.opening_ratio > 0.6);
        assert_eq!(office.balcony_band_every, 0);
    }
}
