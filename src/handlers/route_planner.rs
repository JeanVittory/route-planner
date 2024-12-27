use actix_web::{Responder, get, HttpResponse};
use crate::models::route_planner;

#[get("/route-planner")]
pub async fn get_route_planner() -> impl Responder{
    HttpResponse::Ok().body("Hello from some route!")
}

