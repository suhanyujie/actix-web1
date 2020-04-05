extern crate actix_rt;
extern crate actix_web;
extern crate listenfd;
extern crate serde;
extern crate futures;
extern crate serde_json;
extern crate actix_service;

mod app;

use actix_web::{web, get, HttpServer, App, HttpRequest, HttpResponse, Responder, Error};
use std::sync::Mutex;
use listenfd::ListenFd;
use serde::{Serialize};
use std::collections::HashMap;
use futures::future::{ready, Ready, FutureExt};
use actix_service::Service;
use app::middleware::simple_middleware;
use app::libs::logger;

struct AppState {
    app_name: String,
}

// 自定义响应类型
#[derive(Serialize)]
struct MyRes<T> 
where
    T: Serialize
{
    err_code: i32,
    err_msg: String,
    results: Option<T>,
}

impl<T> Responder for MyRes<T>
where
    T: Serialize
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    logger::init_logger();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start 1. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .data(AppState {
                app_name: String::from("Actix-web app"),
            })
            .route("/", web::get().to(index))
            .route("again", web::get().to(index2))
            .service(index3)
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:9101")?
    };

    server.run().await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

async fn index2() -> impl Responder {
    let mut hash1 = HashMap::new();
    hash1.insert("id".to_string(), "1".to_string());
    MyRes {
        err_code: 0,
        err_msg: "ok".to_string(),
        results: Some(hash1),
    }
}

#[get("hello")]
async fn index3(data: web::Data<AppState>) -> String {
    format!("hello {}", "hey this is index".to_string() + &data.app_name)
}
