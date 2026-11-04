mod buildings;
mod climate;
mod core;
pub(crate) mod jurisdiction;
pub(crate) mod morphology;
pub(crate) mod probability;
mod roads;
pub mod scene;
pub(crate) mod traffic;
pub mod transport;

pub use buildings::{
    BuildingFacade, Compound, CropKind, Field, ModernBuilding, Parcel, ParcelUse, TreeInstance,
    TreeSpecies,
};
pub use climate::SiteClimate;
pub use core::{
    BuildingMass, CitySpec, CityStyle, ModernChinaSpec, ModernRoadClass, Point, RoofStyle,
    StreetClass, StreetSegment, UrbanBlock, UrbanModel,
};
pub use jurisdiction::{ArrowStyle, DrivingSide, JurisdictionId, SignalStyle, TrafficRules};
pub use morphology::{CityGraph, MorphologyPrior, MorphologyStats, measure};
pub use probability::{
    ActionCandidate, ActionKind, GrowthState, ModelWeights, SplitMix64, sample_action,
};
pub use roads::{
    CityJunction, HdLane, HdRoad, JunctionKind, JunctionPhase, LaneMarking, LaneUse, MarkingKind,
    Movement, RoadConnector, RoadCrossSection, SdNode, SdRoad, SignalHead, TurnArrow,
    cross_section, ramp_cross_section,
};
pub use scene::{CityFrameInfo, ModernCity, RiverSetting, Tributary};
pub use traffic::{ApproachSpec, synthesize_junction};
