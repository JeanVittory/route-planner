use actix_web::{Responder, post, HttpResponse, web};
use crate::models::route_planner::RequestRoutePlanner;
use crate::models::state::ApplicationState;
use crate::services::route_api::route_api;
use serde_json::Value;

#[post("/route-planner")]
pub async fn get_route_planner(state:web::Data<ApplicationState>, payload:web::Json<RequestRoutePlanner>) -> impl Responder{
    let api_key = state.api_key.lock().unwrap();
    let response = route_api(&payload, &api_key).await;
    match response {
        Ok(res) =>{
            if let Ok(res) = res.json::<Value>().await{
                HttpResponse::Ok().json(res)
            }else{
                HttpResponse::InternalServerError().body("Error: Error reading response body")
            }
        },
        Err(e) => {
            let error_message = format!("Error sending request to external service: {}", e);
            HttpResponse::InternalServerError().body(error_message)
        },
    }
}

