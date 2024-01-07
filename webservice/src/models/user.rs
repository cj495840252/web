
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug,Deserialize,Serialize)]
pub struct User{
    pub(crate) username: String,
    pub(crate) portrait: String
}