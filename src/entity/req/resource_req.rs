use crate::entity::r#enum::ResourceCategoryEnum;
use rocket::serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct SaveReq {
    pub title: String,
    pub category: ResourceCategoryEnum,
    pub tags: Option<Vec<String>>,
    pub url: String,
    pub sort: Option<i32>,
}
