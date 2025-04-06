use rbatis::impl_select;
use rbatis::rbdc::DateTime;
use rocket::serde::{Deserialize, Serialize};

// 用户信息表
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoPo {
    pub id: Option<u64>,
    pub code: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub created_datetime: Option<DateTime>,
    pub updated_datetime: Option<DateTime>,
}

impl_select!(UserInfoPo{select_by_code(code: &str) -> Option => "`where code = #{code}`"}, "t_user_info");
