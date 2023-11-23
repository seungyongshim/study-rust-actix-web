extern crate actix_web;
use actix_web::{App, HttpServer, Responder, web};
use std::io::{*};

mod hello_world;
mod health_check;


#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world::hello_world))
            .route("/healthz", web::get().to(health_check::health_check))
    }).bind("127.0.0.1:8800")?
      .run().await
}


