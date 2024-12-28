use actix_web::{Responder, post, HttpResponse, web};
use crate::models::route_planner::RequestRoutePlanner;

#[post("/route-planner")]
pub async fn get_route_planner(payload:web::Json<RequestRoutePlanner>) -> impl Responder{
    HttpResponse::Ok().json(payload.into_inner())
}

