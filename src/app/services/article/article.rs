extern crate actix_web;
extern crate bson;
extern crate lazy_static;
extern crate log;
extern crate mongodb;
extern crate serde;

use crate::app::common::response;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use lazy_static::lazy_static;
use log::*;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

type BusinessError = response::BusinessError;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    // Client::connect("127.0.0.1", 27017).expect("Failed to initialize standalone client.")
    Client::with_uri_str("mongodb://127.0.0.1:27017/")
        .expect("Failed to initialize standalone client.")
}

fn collection(coll_name: &str) -> Collection {
    MONGO.database("myblog").collection(coll_name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    _id: Option<ObjectId>,
    title: String,
    author: String,
    content: String,
}

impl Article {
    pub const TABLE_NAME: &'static str = "article";
}

type SimpleResp = Result<HttpResponse, BusinessError>;

pub async fn save_article(article: web::Json<Article>) -> SimpleResp {
    let article: Article = article.into_inner();

    info!("save article, {:?}", article);
    response::MyRes::ok(article.title).to_json()
}
