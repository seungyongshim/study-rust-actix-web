extern crate actix_web;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::{*};



#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(hello_world))
    }).bind("127.0.0.1:8800")?
      .run().await
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}