#![allow(unused)]

use std::collections::HashMap;
use std::env;
use std::fmt::format;
use std::ops::Deref;
use crate::state::state::AppState;
use actix_web::{web, HttpResponse, HttpRequest, web::Query, http::StatusCode};
use crate::models::course::{Course, UpdateCourse};
use chrono::{DateTime, NaiveDateTime, Utc};
use dotenvy::dotenv;
use sqlx::{Column, Executor, MySql, mysql::MySqlPoolOptions, Row, Value, ValueRef};
use sqlx::mysql::MySqlValue;
use crate::error_handle::MyError;

pub async fn index(req: HttpRequest,d1: web::Data<String>,d2:web::Data<AppState>,
                   path: web::Path<(String,i32,String)>)
    -> HttpResponse
{
    // println!("{:?}",req);
    // println!("{:?}",d1);
    // println!("AppState:{:?}",&d2.visit_count);
    // let web::Path((val1, val2, val3)) = path;
    let val1 = &path.into_inner();
    // let val1 = &path.0
    // let val2 = &path.1;
    // let val3 = &path.2;
    println!("{:?},{:?},{:?}",val1.0,val1.1,val1.2);
    HttpResponse::Ok().json("success")
}


pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
//**使用数据库连接的*****************************************************************************
pub async fn new_course(
    req: HttpRequest,
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError>
{
    println!("{:?}",req);
    let time = match new_course.time.as_ref() {
        Some(t) => format!("'{}'",t.to_string()) ,
        None => "null".to_string()
    };
    let score = match new_course.score {
        Some(t) => t.to_string(),
        None => "null".to_string()
    };
    let sql = format!(r#"insert into course values({},'{}',{},{},{})"#,
                      new_course.course_id,
                      new_course.course_name,
                      new_course.description.clone().map(|x|format!("'{}'",x)).unwrap_or("null".to_string()),
                      score,
                      time);
    println!("{}",&sql);

    let insert_result = &app_state.db.execute(sql.as_str())
        .await?;
    Ok(HttpResponse::Ok().json(
        "rows affected:".to_string()+insert_result.rows_affected().to_string().as_str()
    ))
}

pub async fn update_course(
    params: web::Query<HashMap<String,i32>>,
    app_state: web::Data<AppState>,
    new_course: web::Json<UpdateCourse>
) -> Result<HttpResponse, MyError>
{
    let old_course_id = *params.get("course_id").unwrap();
    let old_course = sqlx::query!("select * from course where course_id = ?",old_course_id)
        .fetch_one(&app_state.db).await
        .map(|c| {
            Course {
                course_id: c.course_id,
                course_name: c.course_name.unwrap(),
                description: c.description,
                score: c.score,
                time: match c.time {
                    Some(t) => Some(t.naive_utc()),
                    None => None
                },
            }
        })
    .map_err(|_e|MyError::NotFound("Not find need updated data".into()))?;

    println!("old:{:?}",&old_course);
    let course_name = if let Some(course_name) = new_course.clone().course_name {course_name} else { old_course.course_name };
    let course_id = if let Some(course_id) = new_course.course_id {course_id} else { old_course.course_id };
    let description = if let Some(description) = new_course.clone().description {description} else { old_course.description.unwrap() };
    let score = if let Some(score) = new_course.score {score} else { old_course.score.unwrap() };
    let time = if let Some(time) = new_course.time {time} else { old_course.time.unwrap() };

    let sql = format!(
            r#"update course set course_id = {} , course_name='{}'
                , description = '{}' , score = {} , time = '{}' where course_id = {}"#,
            course_id,course_name,description,score,time, old_course_id
    );
    println!("{}",&sql);

    let insert_result = &app_state.db.execute(sql.as_str())
        .await?;
    Ok(HttpResponse::Ok().json(
        "rows affected:".to_string()+insert_result.rows_affected().to_string().as_str()
    ))
}


pub async fn get_courses(app_state: web::Data<AppState>,
                                     query: web::Query<HashMap<String,String>>
) -> Result<HttpResponse, MyError>
{
    // 由于Course存在Option类型，建议手动解析，如下面的map方法
    let mut condition_string = "".to_string();
    let mut condition = Vec::new();
    if query.contains_key("course_id") {
        let (k,v) = query.get_key_value("course_id").unwrap();
        condition.push(format!("{} = {}", k,v));
    }

    if query.contains_key("course_name") {
        let (k,v) = query.get_key_value("course_name").unwrap();
        condition.push(format!("{} = '{}'", k,v));
    }

    if condition.len() > 0{
        condition_string = format!("where {}", condition.join(" and "))
    }

    let sql = format!("select * from course {}",&condition_string);
    println!("SQL: {}",&sql);
    let rows = app_state.db
        .fetch_all(sql.as_str())
        .await?
        .iter()
        .map(|x| {
            let map = x.columns().iter()
                .map(|x| { (x.name(), x.ordinal()) })
                .collect::<HashMap<&str,usize>>();

            Course {
                course_id: x.try_get::<i32, usize>(*map.get("course_id").unwrap()).unwrap() ,
                course_name: x.try_get::<String, usize>(*map.get("course_name").unwrap()).unwrap(),
                description: match x.try_get::<String, usize>(*map.get("description").unwrap()) {
                    Ok(t) => Some(t),
                    Err(_e) => None
                },
                score: Some(x.try_get::<i32,usize>(*map.get("score").unwrap()).unwrap()),
                time: Some(
                    x.try_get::<DateTime<Utc>,usize>(*map.get("time").unwrap()).unwrap().naive_local()
                )
            }
        }
        )
        .collect::<Vec<Course>>();

    if rows.len() > 0 {
        Ok(HttpResponse::Ok().json(rows))
    }else {
        Err(MyError::NotFound("Not found data in Database".into()))
    }
}


pub async fn delete_courses(app_state: web::Data<AppState>, query: web::Query<HashMap<String,i32>>)
-> Result<HttpResponse,MyError>
{
    let sql = format!("delete from course where course_id = {}", query.get("course_id").unwrap());
    let res = &app_state.db.execute(sql.as_str()).await?;
    Ok(HttpResponse::Ok().json(format!("{:?}",&res)))
}

#[actix_rt::test]
async fn delete_test(){
    use std::sync::Mutex;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("URL not define in env");
    // println!("{}", database_url);
    let db = MySqlPoolOptions::new()
        .connect(&database_url).await.expect("connect failed");

    let app_state: web::Data<AppState> = web::Data::new(AppState{
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db,
    });

    let mut map = HashMap::new();
    map.insert("course_id".to_string(),1001);
    let res = delete_courses(app_state, web::Query(map)).await.unwrap();

}


// #[cfg(ingore)]
#[actix_rt::test]
async fn post_new_course_test() {
    use std::sync::Mutex;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("URL not define in env");
    // println!("{}", database_url);
    let db = MySqlPoolOptions::new()
        .connect(&database_url).await.expect("connect failed");
    let course = web::Json(Course{
        course_id: 1001,
        course_name : "math".to_string(),
        description: None,
        score: Some(1),
        time: Some(Utc::now().naive_local())
    });

    let app_state: web::Data<AppState> = web::Data::new(AppState{
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db,
    });
    use actix_web::test::TestRequest;
    let req = TestRequest::default().to_http_request();
    let response = new_course(req,course, app_state).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    println!("{:?}",response);
}


#[actix_rt::test]
async fn get_all_courses_success_test() {
    use std::sync::Mutex;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("URL not define in env");
    // println!("{}", database_url);
    let db = MySqlPoolOptions::new()
        .connect(&database_url).await.expect("connect failed");

    let app_state: web::Data<AppState> = web::Data::new(AppState{
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db,
    });
    let mut map: HashMap<String,String> = HashMap::new();
    map.insert("course_name".to_string(),"234".to_string());
    let query = web::Query(map);
    let response = get_courses(app_state,query).await.unwrap();
    // println!("{:?}", &response.body());
    assert_eq!(response.status(), StatusCode::OK)
}


