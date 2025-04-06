pub mod config;

use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            code: 200,
            msg: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, msg: String) -> Self {
        Response {
            code,
            msg,
            data: None,
        }
    }
}
