use reqwest::{Response, Error};
use crate::constants::GOOGLE_ENDPOINT;
use crate::models::route_planner::RequestRoutePlanner;
use crate::utilities::json_converter::get_json_format;

pub async fn route_api(request_payload: &RequestRoutePlanner, api_key: &str ) -> Result<Response, Error>{
    let client = reqwest::Client::new();
    let payload = get_json_format(request_payload);
    client.post(GOOGLE_ENDPOINT)
        .header("X-Goog-Api-Key",api_key)
        .header("X-Goog-FieldMask", "routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline")
        .body(payload)
        .send()
        .await
}