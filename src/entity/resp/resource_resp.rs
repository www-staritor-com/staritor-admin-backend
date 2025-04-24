use crate::entity::r#enum::ResourceCategoryEnum;
use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResourceResp {
    pub id: i64,
    pub title: String,
    pub category: ResourceCategoryEnum,
    pub tags: Option<Vec<String>>,
    pub url: String,
    pub sort: i32,
    pub created_datetime: NaiveDateTime,
    pub created_user: Option<String>,
    pub updated_datetime: NaiveDateTime,
    pub updated_user: Option<String>,
}
