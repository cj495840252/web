use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenvy::dotenv;
use std::env;
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
    let shared_data = web::Data::new(AppState{
        health_check_response:"I'm ok.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(Vec::new()),
        db: db
    });

    let app = move || {
        App::new()
            .wrap(
                Cors::default()
                    .supports_credentials()
                    .allow_any_origin()
            )
            .app_data(web::Data::new("aa".to_string()))
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:7878")?.run().await
}

