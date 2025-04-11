use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfoResp {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub created_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}
