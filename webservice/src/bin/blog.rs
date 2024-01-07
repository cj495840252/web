use actix_web::{web, App, HttpServer, http};
use std::{io};
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;
use webservice::routers::routers::{articles, user};
use webservice::state::state::AppState;




#[actix_rt::main]
async fn main() -> io::Result<()>{

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("URL not define in env");
    let db = MySqlPoolOptions::new()
        .connect(&database_url).await.expect("connect failed");

    // web::Data会加上Arc
    let shared_data = web::Data::new(AppState{
        db
    });

    let app = move || {
        App::new()
            .wrap(
                Cors::default()
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
            .configure(user)
            .configure(articles)

    };

    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}

