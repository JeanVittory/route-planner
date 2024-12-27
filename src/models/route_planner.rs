use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestRoutePlanner {
    origin: Origin,
    destination: Destination,
    travel_mode: TravelMode,
}
#[derive(Deserialize)] 
pub struct Origin {
    location: LatLng,
}
#[derive(Deserialize)] 
pub struct Destination {
    location: LatLng,
}
#[derive(Deserialize)] 
pub struct LatLng {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]  
#[serde(rename_all = "UPPERCASE")] 
pub enum TravelMode {
    DRIVE,
    WALK,
    BICYCLE,
    TWOWHEELER,
    TRANSIT
}