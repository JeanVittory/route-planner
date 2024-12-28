use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct RequestRoutePlanner {
    origin: Origin,
    destination: Destination,
    #[serde(rename="travelMode")]
    travel_mode: TravelMode,
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
#[serde(rename_all = "UPPERCASE")] 
pub enum TravelMode {
    DRIVE,
    WALK,
    BICYCLE,
    TWOWHEELER,
    TRANSIT
}