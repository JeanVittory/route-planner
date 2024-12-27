mod constants;
mod handlers;
mod models;

use std::io::{self, Write};
use actix_web::{web, App, HttpServer};
use handlers::route_planner::get_route_planner;
use constants::{LOCAL_HOST, PORT, STARTED_SUCCESFULLY, ERROR_RUNNING_SERVER, ERROR_STARTING_SERVER};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(
                web::scope("api")
                    .service(get_route_planner)
            )
    })
    .bind((LOCAL_HOST, PORT));

    match server {
        Ok(server) => {
            print!("{}", STARTED_SUCCESFULLY);
            io::stdout().flush()?;
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