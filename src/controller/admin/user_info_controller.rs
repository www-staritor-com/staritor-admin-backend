use crate::dao;
use crate::entity::response::{Response, ResponseError};
use crate::util::validate::SPECIAL_CHAR_REGEX;
use chrono::NaiveDateTime;
use rocket::serde::{json::Json, Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct SignInReq<'r> {
    #[validate(regex(path = SPECIAL_CHAR_REGEX))]
    code: &'r str,
    password: &'r str,
    name: &'r str,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfoResp {
    id: i64,
    code: String,
    name: String,
    created_datetime: NaiveDateTime,
    updated_datetime: NaiveDateTime,
}

#[get("/user/<code>")]
pub async fn get_user(code: &str) -> Json<Response<UserInfoResp>> {
    let po = dao::user_info_dao::find_by_code(code).await;

    let user_info = po.unwrap_or_else(|err| {
        warn!("get_user err:{:?}", err);
        None
    });

    let resp = match user_info {
        None => Response::error(404, "User not found".to_string()),
        Some(v) => Response::success(UserInfoResp {
            id: v.id,
            code: v.code,
            name: v.name,
            created_datetime: v.created_datetime,
            updated_datetime: v.updated_datetime,
        }),
    };

    Json(resp)
}

#[put("/user", data = "<req>")]
pub async fn sign_in(req: Json<SignInReq<'_>>) -> Json<Response<UserInfoResp>> {
    if let Err(e) = req.validate() {
        return Json(e.to_response());
    }

    let req = req.into_inner();
    let po = dao::user_info_dao::find_by_code(&req.code).await;

    let resp = match po {
        Ok(None) => {
            let result = dao::user_info_dao::save(&req.code, &req.password, &req.name).await;
            match result {
                Ok(v) => Response::success(UserInfoResp {
                    id: v.id,
                    code: v.code,
                    name: v.name,
                    created_datetime: v.created_datetime,
                    updated_datetime: v.updated_datetime,
                }),
                Err(err) => {
                    error!("sign_in err:{:?}", err);
                    Response::error(500, "Insert user failed".to_string())
                }
            }
        }
        Ok(Some(_)) => Response::error(404, "User already existed".to_string()),
        Err(err) => {
            error!("sign_in err:{:?}", err);
            Response::error(500, "Insert user failed".to_string())
        }
    };

    Json(resp)
}

#[delete("/user/<code>")]
pub async fn delete_user(code: &str) -> Json<Response<bool>> {
    let result = dao::user_info_dao::delete_by_code(&code).await;

    let resp = match result {
        Ok(v) => Response::success(v),
        Err(_) => Response::error(404, "User not found".to_string()),
    };

    Json(resp)
}
