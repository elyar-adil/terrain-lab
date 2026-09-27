use crate::model::{
    BuildingMass, CityStyle, ModernCity, ModernRoadClass, StreetClass, StreetSegment, UrbanModel,
};

impl ModernCity {
    /// Compatibility projection for older infrastructure consumers. The
    /// rich SD/HD scene remains the canonical representation.
    pub fn urban_model(&self) -> UrbanModel {
        let streets = self
            .hd_roads
            .iter()
            .map(|road| StreetSegment {
                from: *road.centreline.first().unwrap_or(&crate::model::Point {
                    x_km: 0.0,
                    y_km: 0.0,
                }),
                to: *road.centreline.last().unwrap_or(&crate::model::Point {
                    x_km: 0.0,
                    y_km: 0.0,
                }),
                class: match road.class {
                    ModernRoadClass::Expressway | ModernRoadClass::Arterial => {
                        StreetClass::Boulevard
                    }
                    ModernRoadClass::Collector => StreetClass::Avenue,
                    ModernRoadClass::Local => StreetClass::Street,
                },
                width_metres: road.width_metres,
            })
            .collect();
        let buildings = self
            .buildings
            .iter()
            .map(|building| BuildingMass {
                footprint: building.footprint.clone(),
                courtyard: None,
                height_metres: building.height_metres,
                roof: building.roof,
            })
            .collect();
        UrbanModel {
            style: CityStyle::ChineseModern,
            streets,
            blocks: self.blocks.clone(),
            buildings,
        }
    }
}
