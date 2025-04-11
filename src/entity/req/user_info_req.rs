use crate::util::validate::SPECIAL_CHAR_REGEX;
use rocket::serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct SignInReq<'r> {
    #[validate(regex(path = SPECIAL_CHAR_REGEX))]
    pub code: &'r str,
    pub password: &'r str,
    pub name: &'r str,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PageReq<'r> {
    pub fuzzy: &'r str,
}
