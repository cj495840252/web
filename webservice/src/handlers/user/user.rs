#![allow(unused)]

use std::collections::HashMap;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header::HeaderValue;
use actix_web::http::StatusCode;
use sqlx::{Error, Executor};
use crate::error_handle::MyError;
use crate::state::state::AppState;
use crate::models::user::User;

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








pub async fn get_user_info(request: HttpRequest, share_data:web::Data<AppState>)
-> Result<HttpResponse,MyError> {
    let user = sqlx::query!("select * from user.user")
        .fetch_one(&share_data.db).await
        .map(|row| {
            User {
                username: row.username.ok_or_else(||Error::RowNotFound).expect("user表没有username"),
                portrait: row.portrait.ok_or_else(||Error::RowNotFound).expect("user表没有portrait")
            }
        })
        .map_err(|_e|MyError::NotFound("Not find user data".into()))?;


    Ok(HttpResponse::Ok().json(user))
}