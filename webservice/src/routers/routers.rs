#![allow(unused)]
use crate::handlers::handlers::*;
use actix_web::web;
use crate::handlers::user::{handle_login, handle_profile};
use crate::handlers::articles::{handle_channels, handle_create_article};


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
            web::scope("/article")
                .route("/category", web::get().to(handle_channels))
                .route("/create",web::post().to(handle_create_article))
        );
}
// #[derive(Deserialize)]
// pub struct PathParam{
//     pub course: usize,
//     pub name: String,
// }

