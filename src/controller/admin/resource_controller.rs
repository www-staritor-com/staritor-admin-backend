use crate::dao::resource_dao;
use crate::entity::base::error::BizError;
use crate::entity::base::response::Response;
use crate::entity::req::resource_req::SaveReq;
use rocket::serde::json::Json;

#[put("/resource", data = "<req>")]
pub async fn save(req: Json<SaveReq>) -> Result<Response<i64>, BizError> {
    let tags = match &req.tags {
        None => None,
        Some(v) => {
            let join = v
                .iter()
                .filter(|&x| !x.is_empty())
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .join(",");
            Some(format!(",{}", join))
        }
    };

    let po = resource_dao::save(&req.title, &req.category, &tags, &req.url, &req.sort).await?;
    Ok(Response::success(po.id))
}
