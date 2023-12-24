#![allow(unused)]

use std::collections::HashMap;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use crate::state::state::AppState;

pub async fn handle_login(request: HttpRequest, data: web::Json<HashMap<String,String>>, share_data:web::Data<AppState>)
-> HttpResponse
{
    println!("{:?}",data.get("password"));
    if let Some(username) = data.get("username") {
        if let Some(password) =data.get("password") {
            if username=="13403085904" && password=="123456" {
                return HttpResponse::Ok().json("{ \"token\": \"928u4\", \"status_code\":200}")

            }
        }
    }
    HttpResponse::Ok().json("{\"status_code\":201,\"msg\":\"Failed\"}")
}

pub async fn handle_profile(request: HttpRequest,
                            share_data:web::Data<AppState>)
                          -> HttpResponse
{
    let default_auth = HeaderValue::from_str("").unwrap();
    // println!("{:?}", request.headers());
    if request.headers().get("Authorization").unwrap_or(&default_auth) == "Bearer 928u4" {
        HttpResponse::Ok().json(r#"{
        "name":"Zack J Chen",
        "portrait":"/touxiang.jpeg",
        "describe":"浪漫，就是浪费时间慢慢吃饭，浪费时间慢慢喝茶，浪费时间慢慢走..."}
    "#)
    }else {
        HttpResponse::Ok().status(StatusCode::from_u16(401).unwrap()).json("登陆失效")
    }

}

// pub async fn handle_profile(request: HttpRequest,
//                             share_data:web::Data<AppState>)
//                             -> HttpResponse
// {
//
//     println!("{:?}", request.headers());
//     HttpResponse::Ok().json("Request success")
// }