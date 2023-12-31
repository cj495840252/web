#![allow(unused)]

use std::any::Any;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use actix_multipart::form::{bytes, FieldReader};
use actix_web::{HttpRequest, HttpResponse, web};
use crate::state::state::AppState;
use actix_multipart::{Field, Multipart};
use actix_web::http::header::ContentLength;
use futures_util::StreamExt as _;
use serde_json::to_string;
use crate::error_handle::MyError;
use crate::handlers::articles;
use crate::handlers::articles::structs::Article;


pub mod structs;

pub async  fn handle_channels(_request: HttpRequest,_shared_data:web::Data<AppState>) -> HttpResponse{
    return HttpResponse::Ok().json(r#"["大数据", "Rust", "前端", "后端", "机器学习"]"#);
}

pub async  fn handle_create_article(request: HttpRequest
                                    , article: web::Json<structs::Article>
                                    , _shared_data:web::Data<AppState>,
                                    mut articles:  web::Data<Mutex<Vec<Article>>>
)
                                    -> HttpResponse{
    println!("POST /articles/create");
    println!("last count in vec：{:?}",articles.lock().unwrap().len());
    articles.lock().unwrap().push(Article::from(article));
    println!("add a record success：{:?}",articles.lock().unwrap().len());

    return HttpResponse::Ok().json(r#"文章保存成功"#);

}

pub async fn handle_upload(mut payload: Multipart)
-> Result<HttpResponse,MyError>{
    let mut files:Vec<String> = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let save_folder = std::env::current_dir().unwrap().join("webservice/public/");
        // string是新分配一个空间，为了满足可变引用后不可以引用的问题，这里新建一个
        let file_name = field.content_disposition().get_filename().unwrap().to_string();
        let mut file = fs::File::create(save_folder.join(file_name.to_owned())).unwrap();

        // next方法是可变引用，所以之后不可以可变引用
        while let Some(chunk) = field.next().await {
            file.write(&chunk.as_ref().unwrap());// 这里chunk要是引用，不然他就被drop了
        }
        // 由于出了这个scope，file_name会被drop，这里克隆
        files.push(file_name.to_owned());

    }
    files = files.iter_mut().map(|x|"public/".to_string()+x).collect::<Vec<String>>();
    let mut result = HashMap::new();

    result.insert("data",files);
    Ok(HttpResponse::Ok().json(result))
}



pub async fn handle_get_list(request: HttpRequest,
                             params: web::Query<HashMap<String,String>>,
                             mut articles:web::Data<Mutex<Vec<Article>>>)
-> HttpResponse{
    // println!("{:?}",params);//  根据参数去筛选数据
    // println!("{:?}",serde_json::to_string();
    // let a = &articles.lock().unwrap();
    // let mut s = String::from("{ \"data\": ");
    // s.push_str(&serde_json::to_string(&articles.lock().unwrap().as_slice()).unwrap());
    // s.push_str("}");
    println!("{:?}", articles.lock().unwrap().len());
    let mut res = HashMap::new();
    res.insert("data", articles.lock().unwrap().clone());
    return HttpResponse::Ok().json(res) ;
}
pub async fn handle_delete_article(request: HttpRequest,
                             params: web::Path<String>,
                             mut articles:web::Data<Mutex<Vec<Article>>>)
-> HttpResponse{
    let id = params.parse::<String>().unwrap().clone();
    println!("{:?}",id);//
    let mut deleted_index = -1;
    for (index,val) in articles.lock().unwrap().iter().enumerate() {
        if *(val.id.as_ref().unwrap()) == id {
            deleted_index = index as i32
        }
    }
    if deleted_index >= 0 {
        articles.lock().unwrap().remove(deleted_index as usize);
        return HttpResponse::Ok().json("删除成功") ;

    }
    return HttpResponse::Ok().json("没有该数据") ;

}