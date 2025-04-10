
use rocket::serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

pub trait ResponseError<T> {
    fn to_response(self) -> Response<T>;
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

impl<T> ResponseError<T> for ValidationErrors {
    fn to_response(self) -> Response<T> {
        Response {
            code: 400,
            msg: format!("{:?}", self),
            data: None,
        }
    }
}
