use actix_web::{Responder, post, HttpResponse, web};
use crate::models::route_planner::RequestRoutePlanner;
use crate::models::state::ApplicationState;
use crate::services::route_api::route_api;

#[post("/route-planner")]
pub async fn get_route_planner(state:web::Data<ApplicationState>, payload:web::Json<RequestRoutePlanner>) -> impl Responder{
    let api_key = state.api_key.lock().unwrap();
    route_api(&payload, &api_key).await;
    HttpResponse::Ok().json(payload.into_inner())
}

