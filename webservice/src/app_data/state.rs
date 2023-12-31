// use std::sync::Mutex;
// use super::models::Course;
use sqlx::mysql::MySqlPool;

pub struct AppState {
    // pub health_check_response: String,
    // 如果多个地方要修改
    // pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: MySqlPool,
}

