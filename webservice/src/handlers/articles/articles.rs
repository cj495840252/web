#![allow(unused)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use actix_web::{HttpRequest, HttpResponse, web};
use crate::state::state::AppState;
use actix_multipart::Multipart;
use actix_web::http::header::ContentLength;
use futures_util::StreamExt as _;

mod structs;

pub async  fn handle_channels(_request: HttpRequest,_shared_data:web::Data<AppState>) -> HttpResponse{
    return HttpResponse::Ok().json(r#"["大数据", "Rust", "前端", "后端", "机器学习"]"#);
}

pub async  fn handle_create_article(request: HttpRequest
                                    , article: web::Json<structs::Article>
                                    ,_shared_data:web::Data<AppState>)
-> HttpResponse{
    println!("{:?}",article);
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

    return HttpResponse::Ok().json(r#"上传成功"#);
}

