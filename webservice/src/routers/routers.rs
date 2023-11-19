use crate::handlers::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/courses")
                .route("/new",web::post().to(new_course))
                .route("/index/{val1}_{val2}/{val3}",web::get().to(index))
                .route("/find", web::get().to(get_courses))
                .route("/delete", web::delete().to(delete_courses))
                .route("/update", web::post().to(update_course))
        );
}
// #[derive(Deserialize)]
// pub struct PathParam{
//     pub course: usize,
//     pub name: String,
// }

