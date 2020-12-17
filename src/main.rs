#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod database;
mod errors;
mod routes;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    database::init();
    env_logger::init();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8088".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath)
            .service(routes::init())
            .default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
