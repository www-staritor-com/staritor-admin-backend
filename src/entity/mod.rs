pub mod config;

use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}
