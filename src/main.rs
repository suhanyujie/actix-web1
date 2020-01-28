extern crate actix_rt;
extern crate actix_web;

use actix_web::{web, get, HttpServer, App, HttpResponse, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web app"),
            })
            .route("/", web::get().to(index))
            .route("again", web::get().to(index2))
            .service(index3)
    })
    .bind("0.0.0.0:9101")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("hello world index2")
}

#[get("hello")]
async fn index3(data: web::Data<AppState>) -> String {
    format!("hello {}", "hey this is index".to_string() + &data.app_name)
}
