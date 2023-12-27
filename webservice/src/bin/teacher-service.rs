use actix_web::{web, App, HttpServer, http};
use std::io;
// use std::sync::Mutex;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;
// use actix_cors::Cors;


#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routers/mod.rs"]
mod routers;

#[path = "../app_data/mod.rs"]
mod state;

#[path= "../models/mod.rs"]
mod models;

#[path="../error_handle/error_handle.rs"]
mod error_handle;

use crate::routers::routers::*;
use state::state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("URL not define in env");
    let db = MySqlPoolOptions::new()
        .connect(&database_url).await.expect("connect failed");

    // web::Data会加上Arc
    let shared_data = web::Data::new(AppState{
        // health_check_response:"I'm ok.".to_string(),
        // visit_count: Mutex::new(0),
        // courses: Mutex::new(Vec::new()),
        db
    });

    let app = move || {
        App::new()
            .wrap(
                Cors::default()
                    // here is cannot fill out *, or else Error: Custom { kind: Other, error: "can not start server service 0" }
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("null")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT"])
                    .allowed_headers(vec![http::header::AUTHORIZATION,
                                          http::header::ACCEPT,
                                          http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                                          // 这个header用来结局antd的upload组件的请求
                                          http::header::HeaderName::from_str("x-requested-with").unwrap()
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)

            )
            .app_data(shared_data.clone())
            .app_data(web::PayloadConfig::default().limit(1_048_576))
            // .configure(general_routes)
            .configure(course_routes)
            .configure(user_routes)
            .configure(article_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}

