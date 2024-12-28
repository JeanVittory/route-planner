use crate::constants::GOOGLE_ENDPOINT;
use crate::models::route_planner::RequestRoutePlanner;

pub async fn route_api(payload:&RequestRoutePlanner, api_key: &str ) -> (){
    let client = reqwest::Client::new();
    let response = client.post(GOOGLE_ENDPOINT).header("X-Goog-Api-Key",api_key ).body("").send().await;

    match response {
        Ok(response) => println!("{:?}", response),
        Err(e) => println!("Error: {}", e),
    }
}