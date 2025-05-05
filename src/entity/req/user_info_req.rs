use crate::util::validate::SPECIAL_CHAR_REGEX;
use rocket::serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct SignInReq {
    #[validate(regex(path = SPECIAL_CHAR_REGEX))]
    pub code: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PageReq {
    pub fuzzy: Option<String>,
}
