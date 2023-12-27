use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use actix_multipart::{Field, Multipart};
use actix_web::{FromRequest, HttpRequest, web};
use actix_web::web::{Bytes, Json, Payload};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug,Deserialize,Serialize)]
pub struct Article{
    //                 "id":"8218",
    //                 "content_count":21,
    //                 "cover":{
    //                     "images":[]
    //                 },
    //                 "status":1,
    //                 "like_count":0,
    //                 "publish_date":"2023-12-27 21:05:14",
    //                 "read_count":2,
    //                 "title": "Rust入门到再入门"

    id: Option<String>,
    title: String,
    content_count: Option<u32>,
    like_count: Option<u32>,
    read_count: Option<u32>,
    status: Option<i8>,
    content: String,
    cover: Cover,
    category: String
}


impl From<web::Json<Article>> for  Article {
    fn from(value: Json<Article>) -> Self {
        let id = Uuid::new_v4();
        Article{
            id: Some(id.to_string()),
            title: value.title.to_owned(),
            content_count: Some(value.content_count.unwrap_or(0)),
            like_count: Some(value.like_count.unwrap_or(0)),
            read_count: Some(value.read_count.unwrap_or(0)),
            status: Some(value.status.unwrap_or(0)),
            content: value.content.to_owned(),
            cover: value.cover.clone(),
            category: value.category.to_owned(),
        }
    }
}



#[derive(Debug,Default,Deserialize,Serialize,Clone)]
pub struct Cover{
    image_type: i32,
    images: Vec<String>
}





