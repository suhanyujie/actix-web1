extern crate actix_web;
extern crate bson;
extern crate lazy_static;
extern crate log;
extern crate mongodb;
extern crate serde;

use crate::app::common::response;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;
use bson::{decode_document, encode_document, Bson, Document, doc};
use lazy_static::lazy_static;
use log::*;
use mongodb::{Client, Collection, Cursor};
use serde::{Deserialize, Serialize, Serializer};

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
    #[serde(serialize_with = "serialize_object_id")]
    _id: Option<ObjectId>,
    title: String,
    author: String,
    content: String,
}

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error> 
where
    S: Serializer 
{
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) =>s.serialize_str(&v),
        None => s.serialize_none(),
    }
}

impl Article {
    pub const TABLE_NAME: &'static str = "article";
}

type SimpleResp = Result<HttpResponse, BusinessError>;

pub async fn save_article(article: web::Json<Article>) -> SimpleResp {
    let article: Article = article.into_inner();

    info!("save article, {:?}", &article);
    // write into db
    let mut d = bson::to_bson(&article)
            .map(|x| x.as_document().unwrap().to_owned())
            .unwrap();
    info!("{}", &d);
    d.remove("_id");
    info!("{}", &d);
    let result = collection(Article::TABLE_NAME).insert_one(d, None);
    info!("{:?}", &result);
    match result {
        Ok(rs) => {
            let new_id: String = rs.inserted_id.as_object_id().unwrap().to_hex();
            info!("save article, id={}", new_id);
            response::MyRes::ok(new_id).to_json()
        }
        Err(e) => {
            error!("save_article error, {}", e);
            Err(BusinessError::InternalError)
        }
    }
}

pub trait CursorVec {
    fn to_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T>;
}

impl CursorVec for Cursor {
    fn to_vec<'a, T: Serialize + Deserialize<'a>>(&mut self) -> Vec<T> {
        self.map(|item|{
            let doc: Document = item.unwrap();
            let bson = bson::Bson::Document(doc);
            return bson::from_bson(bson).unwrap();
        }).collect()
    }
}

pub async fn list_article() -> SimpleResp {
    let coll = collection("article");
    let cursor = coll.find(Some(doc!{}), None);
    let result = cursor.map(|mut x| x.to_vec::<Article>());
    match result {
        Ok(list) => {
            let _a = list.iter().map(|article| {
                let id = article._id.as_ref().unwrap();
                dbg!(&id);
                id
            });
            
            response::MyRes::ok(list).to_json()
        },
        Err(err) => {
            error!("list article error, {}", err);
            return Err(BusinessError::InternalError);
        }
    }
}
