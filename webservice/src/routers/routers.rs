#![allow(unused)]

use std::sync::Mutex;
use crate::handlers::handlers::*;
use actix_web::web;
use actix_web::web::Data;
use crate::handlers::user::{handle_login, handle_profile};
use crate::handlers::articles::{handle_channels, handle_create_article, handle_get_list, handle_upload};
use crate::handlers::articles::structs::Article;

// pub fn general_routes(cfg: &mut web::ServiceConfig) {
//     cfg.route("/health", web::get().to(health_check_handler));
// }

pub fn course_routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/courses")
                .route("/new",web::post().to(new_course))
                // .route("/index/{val1}_{val2}/{val3}",web::get().to(index))
                .route("/find", web::get().to(get_courses))
                .route("/delete", web::delete().to(delete_courses))
                .route("/update", web::post().to(update_course))
        );
}


pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(web::scope("/user")
        .route("/login", web::post().to(handle_login))
        .route("/profile", web::get().to(handle_profile))
    );
}

pub  fn  article_routes(cfg: &mut web::ServiceConfig){
    let str = r#"
        [
            {
                "id":"8218",
                "content_count":21,
                "cover":{
                    "images":[],
                    "image_type":2
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
    let mut articles:Data<Mutex<Vec<Article>>> = web::Data::new( Mutex::new(tmp_articles));
    cfg
        .service(
            web::scope("/articles")
                .route("/category", web::get().to(handle_channels))
                .route("/create",web::post().to(handle_create_article))
                .route("/upload",web::post().to(handle_upload))
                .route("/list",web::get().to(handle_get_list))


        ).app_data( articles.clone());
}
// #[derive(Deserialize)]
// pub struct PathParam{
//     pub course: usize,
//     pub name: String,
// }

