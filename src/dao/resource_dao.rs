use crate::dao;
use crate::dao::mapper::prelude::{
    ResourceActiveModel, ResourceColumn, ResourceMapper, ResourcePo,
};
use crate::entity::base::request::PageRequest;
use crate::entity::base::response::Page;
use crate::entity::r#enum::ResourceCategoryEnum;
use crate::entity::req::resource_req::PageReq;
use rocket::serde::json::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm::{PaginatorTrait, QueryOrder};

pub async fn page(req: &Json<PageRequest<PageReq>>) -> Result<Page<ResourcePo>, DbErr> {
    let page = req.page();
    let size = req.size();
    let mut opt = ResourceMapper::find();

    if let Some(v) = &req.select {
        match v.fuzzy {
            Some(ref v) => {
                opt = opt.filter(
                    Condition::any()
                        .add(ResourceColumn::Title.contains(v))
                        .add(ResourceColumn::Tags.contains(format!(",{}", v))),
                );
            }
            None => {}
        }
        match v.category {
            Some(ref v) => {
                opt = opt.filter(ResourceColumn::Category.eq(v.to_owned()));
            }
            None => {}
        }
    }

    let total = opt.clone().count(dao::conn()).await?;
    let data = opt
        .order_by(ResourceColumn::Id, sea_orm::Order::Asc)
        .paginate(dao::conn(), size)
        .fetch_page(page)
        .await?;

    Ok(Page::new(page, size, total, Some(data)))
}

pub async fn save(
    title: &str,
    category: &ResourceCategoryEnum,
    tags: &Option<String>,
    url: &str,
    sort: &Option<i32>,
) -> Result<ResourcePo, DbErr> {
    let resource = ResourceActiveModel {
        title: Set(title.to_owned()),
        category: Set(category.to_owned()),
        tags: Set(tags.to_owned()),
        url: Set(url.to_owned()),
        sort: Set(sort.to_owned()),
        ..Default::default()
    };

    resource.insert(dao::conn()).await
}

pub async fn update(
    id: &i64,
    title: &str,
    category: &ResourceCategoryEnum,
    tags: &Option<String>,
    url: &str,
    sort: &Option<i32>,
) -> Result<ResourcePo, DbErr> {
    let resource = ResourceActiveModel {
        id: Set(id.to_owned()),
        title: Set(title.to_owned()),
        category: Set(category.to_owned()),
        tags: Set(tags.to_owned()),
        url: Set(url.to_owned()),
        sort: Set(sort.to_owned()),
        ..Default::default()
    };

    resource.update(dao::conn()).await
}

pub async fn delete(id: &i64) -> Result<bool, DbErr> {
    ResourceMapper::delete_by_id(id.to_owned())
        .exec(dao::conn())
        .await
        .map(|v| {
            if v.to_owned().rows_affected > 0 {
                true
            } else {
                false
            }
        })
}
