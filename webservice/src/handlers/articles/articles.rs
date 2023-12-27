#![allow(unused)]

use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use actix_web::{HttpRequest, HttpResponse, web};
use crate::state::state::AppState;
use actix_multipart::Multipart;
use actix_web::http::header::ContentLength;
use futures_util::StreamExt as _;
use crate::handlers::articles;
use crate::handlers::articles::structs::Article;

pub mod structs;

pub async  fn handle_channels(_request: HttpRequest,_shared_data:web::Data<AppState>) -> HttpResponse{
    return HttpResponse::Ok().json(r#"["大数据", "Rust", "前端", "后端", "机器学习"]"#);
}

pub async  fn handle_create_article(request: HttpRequest
                                    , article: web::Json<structs::Article>
                                    , _shared_data:web::Data<AppState>,
                                    mut articles:  web::Data<Mutex<Vec<Article>>>
)
                                    -> HttpResponse{
    // println!("{:?}",article);
    &articles.lock().unwrap().push(Article::from(article));
    println!("{:?}",articles);
    return HttpResponse::Ok().json(r#"文章保存成功"#);

}

pub async fn handle_upload(mut body: web::Payload, request: HttpRequest)
-> HttpResponse{
    println!("{:?}",request.headers().get("Content-Length").unwrap());
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
        println!("{:?}", bytes)
    }

    return HttpResponse::Ok().json(r#"{"data":"http://localhost:3001/static/filename.txt","msg":"success"}"#);
}



pub async fn handle_get_list(request: HttpRequest,
                             params: web::Query<HashMap<String,String>>,
                             mut articles:web::Data<Mutex<Vec<Article>>>)
-> HttpResponse{

    // println!("{:?}",serde_json::to_string();
    // let a = &articles.lock().unwrap();
    let mut s = String::from("{ \"data\": ");
    s.push_str(&serde_json::to_string(&articles.lock().unwrap().as_slice()).unwrap());
    s.push_str("}");
    return HttpResponse::Ok().json(s) ;
}