mod buildings;
mod core;
pub(crate) mod jurisdiction;
pub(crate) mod morphology;
pub(crate) mod probability;
mod roads;
pub mod scene;
pub(crate) mod traffic;

pub use buildings::{
    BuildingFacade, Compound, ModernBuilding, Parcel, ParcelUse, TreeInstance, TreeSpecies,
};
pub use core::{
    BuildingMass, CitySpec, CityStyle, ModernChinaSpec, ModernRoadClass, Point, RoofStyle,
    StreetClass, StreetSegment, UrbanBlock, UrbanModel,
};
pub use jurisdiction::{
    ArrowStyle, DrivingSide, JurisdictionId, SignalStyle, TrafficRules,
};
pub use morphology::{CityGraph, MorphologyPrior, MorphologyStats, measure};
pub use probability::{
    ActionCandidate, ActionKind, GrowthState, ModelWeights, SplitMix64, sample_action,
};
pub use roads::{
    CityJunction, HdLane, HdRoad, JunctionKind, JunctionPhase, LaneMarking, LaneUse, MarkingKind,
    Movement, RoadConnector, RoadCrossSection, SdNode, SdRoad, SignalHead, TurnArrow, cross_section,
};
pub use traffic::{ApproachSpec, synthesize_junction};
pub use scene::{CityFrameInfo, ModernCity, Tributary};
