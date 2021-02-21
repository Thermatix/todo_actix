use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;

use std::io;

mod models;
mod app_config;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(models::Status::new("UP"))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = app_config::Config::from_env().unwrap();

    println!("Starting server at \"http://{}:{}/\"...", config.server.host, config.server.port);

    HttpServer::new(|| {

        App::new()
            .route("/", web::get().to(status))

    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
