mod constants;
mod handlers;
mod models;
mod services;

use std::{env, sync::{Arc, Mutex}};
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use handlers::route_planner::get_route_planner;
use constants::{LOCAL_HOST, PORT, STARTED_SUCCESFULLY, ERROR_RUNNING_SERVER, ERROR_STARTING_SERVER};
use models::state::ApplicationState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let api_key = env::var("API_KEY").expect("You must to provide an API key");

    let server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(ApplicationState {
            api_key: Arc::new(Mutex::new(api_key.clone()))
        }))
        .service(
            web::scope("api")
                .service(get_route_planner)
        )
    })
    .bind((LOCAL_HOST, PORT));

    match server {
        Ok(server) => {
            println!("{}", STARTED_SUCCESFULLY);
            if let Err(e) = server.run().await {
                eprintln!("{} {}", ERROR_RUNNING_SERVER, e);
            }
        }
        Err(e) => {
            eprintln!("{} {}", ERROR_STARTING_SERVER, e);
        }
    }

    Ok(())
}