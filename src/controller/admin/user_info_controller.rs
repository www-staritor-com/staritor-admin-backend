use crate::dao;
use crate::entity::base::error::BizError;
use crate::entity::base::request::PageRequest;
use crate::entity::base::response::{Page, Response};
use crate::entity::req::user_info_req::{PageReq, SignInReq};
use crate::entity::resp::user_info_resp::UserInfoResp;
use rocket::serde::json::Json;
use validator::Validate;

#[get("/user/<code>")]
pub async fn get(code: String) -> Result<Response<UserInfoResp>, BizError> {
    let po = dao::user_info_dao::find_by_code(&code).await?;

    let resp = match po {
        None => return Err(BizError::NotFound(code)),
        Some(v) => UserInfoResp {
            id: v.id,
            code: v.code,
            name: v.name,
            created_datetime: v.created_datetime,
            updated_datetime: v.updated_datetime,
        },
    };

    Ok(Response::success(resp))
}

#[post("/user/page", data = "<req>")]
pub async fn page(
    req: Json<PageRequest<PageReq>>,
) -> Result<Response<Page<UserInfoResp>>, BizError> {
    let po = dao::user_info_dao::page(&req).await?;

    let data = match &po.data {
        None => None,
        Some(v) => Some(
            v.into_iter()
                .map(|v| UserInfoResp {
                    id: v.id,
                    code: v.code.to_owned(),
                    name: v.name.to_owned(),
                    created_datetime: v.created_datetime,
                    updated_datetime: v.updated_datetime,
                })
                .collect(),
        ),
    };

    Ok(Response::success(po.replace(data)))
}

#[put("/user", data = "<req>")]
pub async fn sign_in(req: Json<SignInReq>) -> Result<Response<UserInfoResp>, BizError> {
    if let Err(e) = req.validate() {
        return Err(e.into());
    }

    let req = req.into_inner();
    let po = dao::user_info_dao::find_by_code(&req.code).await?;

    let resp = match po {
        None => {
            let result = dao::user_info_dao::save(&req.code, &req.password, &req.name).await?;
            UserInfoResp {
                id: result.id,
                code: result.code,
                name: result.name,
                created_datetime: result.created_datetime,
                updated_datetime: result.updated_datetime,
            }
        }
        Some(_) => return Err(BizError::CustomError("User already existed".to_owned())),
    };

    Ok(Response::success(resp))
}

#[delete("/user/<code>")]
pub async fn delete(code: &str) -> Result<Response<bool>, BizError> {
    let result = dao::user_info_dao::delete_by_code(&code).await?;

    Ok(Response::success(result))
}
