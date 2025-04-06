use crate::dao;
use crate::entity::Response;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SignInReq<'r> {
    code: &'r str,
    password: &'r str,
    name: &'r str,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfoResp {
    code: String,
    password: String,
    name: String,
}

#[get("/user/<code>")]
pub async fn get_user(code: &str) -> Json<Response<UserInfoResp>> {
    let result = dao::user_info_dao::select_by_code(code).await;

    let user_info = result.unwrap_or_else(|err| {
        warn!("get_user err:{:?}", err);
        None
    });

    let resp = match user_info {
        None => Response::error(404, "User not found".to_string()),
        Some(value) => Response::success(UserInfoResp {
            code: value.code.unwrap_or("".to_string()),
            password: value.password.unwrap_or("".to_string()),
            name: value.name.unwrap_or("".to_string()),
        }),
    };

    Json(resp)
}

#[post("/user/sign-in", data = "<req>")]
pub async fn sign_in(req: Json<SignInReq<'_>>) -> Json<Response<UserInfoResp>> {
    let req = req.into_inner();

    let resp = UserInfoResp {
        code: req.code.to_string(),
        password: req.password.to_string(),
        name: req.name.to_string(),
    };

    Json(Response::success(resp))
}
