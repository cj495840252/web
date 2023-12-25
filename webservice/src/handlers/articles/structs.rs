use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use actix_multipart::{Field, Multipart};
use actix_web::{FromRequest, HttpRequest};
use actix_web::web::{Bytes, Payload};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Article{
    title: String,
    content: String,
    cover: Option<Cover>,
    category: String
}
#[derive(Debug,Default,Deserialize)]
pub struct Cover{
    article_type: i32,
    images: Vec<HashMap<String,String>>
}





