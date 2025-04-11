use rocket::response::Responder;
use rocket::serde::json::serde_json::json;
use rocket::serde::Serialize;
use rocket::{response, Request};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            code: 200,
            msg: "ok".to_owned(),
            data: Some(data),
        }
    }

    pub fn error(code: u16, msg: String) -> Self {
        Response {
            code,
            msg,
            data: None,
        }
    }
}

#[rocket::async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for Response<T> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        response::content::RawJson(json!(&self)).respond_to(request)
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Page<T> {
    page: u64,
    size: u64,
    total: u64,
    pub data: Option<Vec<T>>,
}

impl<T> Page<T> {
    pub fn new(page: u64, size: u64, total: u64, data: Option<Vec<T>>) -> Self {
        Page {
            page: page + 1,
            size,
            total,
            data,
        }
    }

    pub fn replace<U>(&self, data: Option<Vec<U>>) -> Page<U> {
        Page {
            page: self.page,
            size: self.size,
            total: self.total,
            data,
        }
    }
}
