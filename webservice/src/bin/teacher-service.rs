use actix_web::{web, App, HttpServer, http};
use std::{io};
// use std::sync::Mutex;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::web::Data;
use sqlx::mysql::MySqlPoolOptions;
// use actix_files as fs;


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
use crate::handlers::articles::structs::Article;

#[actix_rt::main]
async fn main() -> io::Result<()>{
    let str = r#"
        [
            {
                "id":"8218",
                "content_count":21,
                "cover":{
                    "images":["http://localhost:3001/static/1F600.png"],
                    "image_type":3
                },
                "status":1,
                "like_count":0,
                "publish_date":"2023-12-27 21:05:14",
                "read_count":2,
                "title": "Rust入门到再入门",
                "category": "Rust",
                "content":"<p>let i = 10;</p>"
            },
            {
                "id":"8219",
                "content_count":2,
                "cover":{
                    "images":[],
                    "image_type":1
                },
                "status":0,
                "like_count":0,
                "publish_date":"2023-12-27 21:05:14",
                "read_count":2,
                "title": "wkwebview离线加载h5资源结局方案",
                "category": "前端",
                "content":"<p>React</p>"
            },
            {
                "id":"8220",
                "content_count":39,
                "cover":{
                    "images":[],
                    "image_type":0
                },
                "status":-1,
                "like_count":1,
                "publish_date":"2023-12-27 21:05:14",
                "read_count":2,
                "title": "大数据下的裸奔",
                "category": "大数据",
                "content":"<p>Spark</p>"
            }
        ]
    "#;
    let tmp_articles: Vec<Article> = serde_json::from_str(str).unwrap();
    let articles:Data<Mutex<Vec<Article>>> = web::Data::new( Mutex::new(tmp_articles));

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
            .service( actix_files::Files::new("static", "static").show_files_listing())

            // .app_data(web::PayloadConfig::default().limit(1_048_576))
            .app_data( articles.clone())
            .configure(course_routes)
            .configure(user_routes)
            .configure(article_routes)

    };

    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}

