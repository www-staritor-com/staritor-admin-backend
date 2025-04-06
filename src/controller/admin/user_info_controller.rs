use crate::dao;
use crate::entity::Response;
use log::warn;
use rocket::serde::{json::Json, Deserialize, Serialize};

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
        None => Response {
            code: 404,
            msg: "User not found".to_string(),
            data: None,
        },
        Some(value) => Response {
            code: 200,
            msg: "ok".to_string(),
            data: Some(UserInfoResp {
                code: value.code.unwrap(),
                password: value.password.unwrap(),
                name: value.name.unwrap(),
            }),
        },
    };

    Json(resp)
}

#[post("/user/sign-in", data = "<req>")]
pub async fn sign_in(req: Json<SignInReq<'_>>) -> Json<Response<UserInfoResp>> {
    let req = req.into_inner();

    // select(&RB, 1).await?;

    let resp = UserInfoResp {
        code: req.code.to_string(),
        password: req.password.to_string(),
        name: req.name.to_string(),
    };

    Json(Response {
        code: 200,
        msg: "ok".to_string(),
        data: Some(resp),
    })
}
