use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course{
    pub course_id: i32,
    pub course_name: String,
    pub description: Option<String>,
    pub score: Option<i32>,
    pub time: Option<NaiveDateTime>
}

impl From<web::Json<Course>> for Course {
    // from trait的用法是实现从一个范型转换到当前这个类型
    fn from(course: web::Json<Course>) -> Self {
        Course {
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            description: course.description.clone(),
            score: course.score,
            time:course.time,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateCourse{
    pub course_id: Option<i32>,
    pub course_name: Option<String>,
    pub description: Option<String>,
    pub score: Option<i32>,
    pub time: Option<NaiveDateTime>
}


// impl Default for Course {
//     fn default() -> Self {
//         Course{
//             course_id: None,
//             course_name: ,
//             description: None,
//             score: None,
//             time: None
//         }
//     }
// }



#[test]
fn test_json(){
    let c1 =  Course {
        course_id: 1001,
        course_name:"physic".into(),
        description: Some("不会物理".to_string()),
        score: Some(4),
        time: Option::from(
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap()
        )
    };
    println!("{:?}", c1);
    let j = serde_json::to_value(c1.clone()).unwrap();
    println!("{:?}", &j);
    println!("{:?}", web::Json(c1));
}