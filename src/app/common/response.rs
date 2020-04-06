extern crate actix_web;
extern crate failure;
extern crate log;
extern crate serde;

use actix_web::{error, get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready, FutureExt};
use failure::Fail;
use serde::Serialize;

// 自定义响应类型
#[derive(Serialize)]
pub struct MyRes<T>
where
    T: Serialize,
{
    pub err_code: i32,
    pub err_msg: String,
    pub results: Option<T>,
}

impl<T> Responder for MyRes<T>
where
    T: Serialize,
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

impl<T: Serialize> MyRes<T> {
    pub fn ok(data: T) -> Self {
        MyRes {
            err_code: 0,
            err_msg: "".to_owned(),
            results: Some(data),
        }
    }

    pub fn to_json(&self) -> Result<HttpResponse, BusinessError> {
        Ok(HttpResponse::Ok().json(self))
    }
}

impl MyRes<()> {
    pub fn err(err_no: i32, msg: &str) -> Self {
        MyRes {
            err_code: err_no,
            err_msg: msg.to_owned(),
            results: None,
        }
    }
}

impl error::ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            BusinessError::ValidationError { .. } => {
                let resp = MyRes::err(-1, &self.to_string());
                HttpResponse::BadRequest().json(resp)
            }
            _ => {
                let resp = MyRes::err(-1, &self.to_string());
                HttpResponse::InternalServerError().json(resp)
            }
        }
    }
}

#[derive(Fail, Debug)]
enum BusinessError {
    #[fail(display = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[fail(display = "An internal error occured. Please try again later.")]
    InternalError,
}
