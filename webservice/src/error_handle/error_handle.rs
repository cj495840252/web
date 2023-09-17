#![allow(unused)]
// 1.创建一个自定义错误类型
// 2.实现From trait， 用于把其它类型转为该类型
// 3.为自定义错误类型实现ResponseError trait
// 4.在handler里返回自定义错误类型
// 5.Actix会把错误转化为HTTP 响应
use sqlx::error::Error as SQLxError;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use actix_web::{error, Error, http::StatusCode, HttpResponse, Result};
use actix_web::body::BoxBody;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String)
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse{
    error_message: String
}

impl MyError {
    fn  error_response(&self) -> String{
        match self {
            MyError::DBError(msg) => {
                format!("Database error occurred: {:?}", msg).into()
            },
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error: 500".into()
            },
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            },
        }


    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse{error_message:self.error_response()})
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(value: Error) -> Self {
        MyError::ActixError(value.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(value: SQLxError) -> Self {
        MyError::DBError(value.to_string())
    }
}