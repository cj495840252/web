#![allow(unused)]

use std::sync::Mutex;
use crate::handlers::handlers::*;
use actix_web::web;
use actix_web::web::Data;
use crate::handlers::user::{handle_login, handle_profile};
use crate::handlers::articles::{handle_channels, handle_create_article, handle_delete_article, handle_get_list, handle_upload};
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
    cfg
        .service(
            web::scope("/articles")
                .route("/category", web::get().to(handle_channels))
                .route("/create",web::post().to(handle_create_article))
                .route("/upload",web::post().to(handle_upload))
                .route("/list",web::get().to(handle_get_list))
                .route("/delete/{id}",web::post().to(handle_delete_article))


        );
}
// #[derive(Deserialize)]
// pub struct PathParam{
//     pub course: usize,
//     pub name: String,
// }

