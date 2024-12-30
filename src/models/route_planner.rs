use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestRoutePlanner {
    origin: Origin,
    destination: Destination,
    #[serde(rename="travelMode")]
    travel_mode: TravelMode,
    #[serde(rename="routingPreference")]
    routing_preference: RoutingPreference,
    #[serde(rename="computeAlternativeRoutes")]
    compute_alternative_routes: bool,
    #[serde(rename="routeModifiers")]
    route_modifiers: RouteModifiers,
    #[serde(rename="languageCode")]
    language_code: String,
    units: Units
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Origin {
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Destination {
    location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename="latLng")]
    lat_lng: LatLng

}
#[derive(Debug, Serialize, Deserialize)] 
pub struct LatLng {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoutingPreference {
    #[serde(rename="ROUTING_PREFERENCE_UNSPECIFIED")]
    RoutingPreferenceUnspecified,
    #[serde(rename="TRAFIC_UNAWARE")]
    TraficUnaware,
    #[serde(rename="TRAFFIC_AWARE")]
    TraficAware,
    #[serde(rename="TRAFFIC_AWARE_OPTIMAL")]
    TraficAwareOptimal
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TravelMode {
    #[serde(rename="DRIVE")]
    Drive,
    #[serde(rename="WALK")]
    Walk,
    #[serde(rename="BICYLE")]
    Bicyle,
    #[serde(rename="TWO_WHEELER")]
    TwoWheeler,
    #[serde(rename="TRANSIT")]
    Transit,
    #[serde(rename="TRAVEL_MODE_UNSPECIFIED")]
    TravelModeUnspecified
}

#[derive(Debug, Serialize, Deserialize)] 
pub struct RouteModifiers{
    #[serde(rename="avoidTolls")]
    avoid_tolls: Option<bool>,
    #[serde(rename="avoidHighways")]
    avoid_highways: Option<bool>,
    #[serde(rename="avoidFerries")]
    avoid_ferries: Option<bool>,
    #[serde(rename="avoidIndoor")]
    avoid_indoors: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Units{
    IMPERIAL,
    METRIC,
}