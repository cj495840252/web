use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Article{
    title: String,
    content: String,
    cover: Option<Cover>,
    category: String
}
#[derive(Debug,Default,Deserialize)]
pub struct Cover{
    article_type: i32,
    images: Vec<HashMap<String,String>>
}

