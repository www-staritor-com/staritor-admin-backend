use crate::dao::resource_dao;
use crate::entity::base::error::BizError;
use crate::entity::base::request::PageRequest;
use crate::entity::base::response::{Page, Response};
use crate::entity::req::resource_req::{PageReq, SaveOrUpdateReq};
use crate::entity::resp::resource_resp::ResourceResp;
use rocket::serde::json::Json;

#[post("/resource/page", data = "<req>")]
pub async fn page(req: Json<PageRequest<PageReq>>) -> Result<Response<Page<ResourceResp>>, BizError> {
    let po = resource_dao::page(&req).await?;

    let data = match &po.data {
        None => None,
        Some(v) => Some(
            v.iter()
                .map(|v| ResourceResp {
                    id: v.id,
                    title: v.title.to_owned(),
                    category: v.category.to_owned(),
                    tags: v.tags.to_owned().map(|x| {
                        x.split(',')
                            .filter(|&x| !x.is_empty())
                            .map(|x| x.to_owned())
                            .collect::<Vec<String>>()
                    }),
                    url: v.url.to_owned(),
                    sort: v.sort.unwrap_or_default(),
                    created_datetime: v.created_datetime,
                    created_user: None,
                    updated_datetime: v.updated_datetime,
                    updated_user: None,
                })
                .collect(),
        ),
    };

    Ok(Response::success(po.replace(data)))
}

#[put("/resource", data = "<req>")]
pub async fn save_or_update(req: Json<SaveOrUpdateReq>) -> Result<Response<i64>, BizError> {
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

    let po = match req.id {
        None => resource_dao::save(&req.title, &req.category, &tags, &req.url, &req.sort).await?,
        Some(id) => {
            resource_dao::update(&id, &req.title, &req.category, &tags, &req.url, &req.sort).await?
        }
    };

    Ok(Response::success(po.id))
}

#[delete("/resource/<id>")]
pub async fn delete(id: i64) -> Result<Response<bool>, BizError> {
    let result = resource_dao::delete(&id).await?;
    Ok(Response::success(result))
}
