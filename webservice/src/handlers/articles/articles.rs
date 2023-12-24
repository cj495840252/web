use actix_web::{HttpRequest, HttpResponse, web};
use crate::state::state::AppState;
mod structs;

pub async  fn handle_channels(_request: HttpRequest,_shared_data:web::Data<AppState>) -> HttpResponse{
    return HttpResponse::Ok().json(r#"["大数据", "Rust", "前端", "后端", "机器学习"]"#);
}

pub async  fn handle_create_article(request: HttpRequest
                                    , article: web::Json<structs::Article>
                                    ,_shared_data:web::Data<AppState>)
-> HttpResponse{
    println!("{:?}",article);
    return HttpResponse::Ok().json(r#"文章保存成功"#);
}