use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, io};
use dotenvy::from_filename;

#[derive(Debug)]
pub struct Course{
    pub id: Option<usize>,
    pub teacher_id: usize,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl Default for Course {
    fn default() -> Self {
        Course{
            id: None,
            teacher_id: 0,
            name: "".to_string(),
            time: None
        }
    }
}

#[actix_rt::main]
async fn main() -> io::Result<()>{
    from_filename("/Users/zackjchen/Desktop/RustProject/ws/db/resource/.env").ok();
    //如果项目根目录下有.env文件，则直接用这个,否者cargo sqlx prepare生成.sqlx
    // dotenvy::dotenv().ok();
    // env::set_var("DATABASE_URL", "mysql://root:root@localhost:3306/test");//这种直接设置的方法没有用
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not in .env configuration file");
    println!("{}",url);
    let db_pool = MySqlPoolOptions::new()
        .connect(&url).await.unwrap();

    let course_rows = sqlx::query!(r#"select * from course where teacher_id = ? and name=?;"#, 1i32,"aa")
        .fetch_all(&db_pool)
        .await.unwrap();

    println!("{:?}",course_rows);

    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(Course{
            id: Some(row.id as usize),
            teacher_id: (row.teacher_id as usize),
            name: row.name,
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap_or_default().naive_local()))
        })
    }

    println!("Courses={:?}", courses_list);
    Ok(())
}
