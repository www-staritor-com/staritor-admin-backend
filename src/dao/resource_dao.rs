use crate::dao;
use crate::dao::mapper::prelude::{ResourceActiveModel, ResourcePo};
use crate::entity::r#enum::ResourceCategoryEnum;
use sea_orm::{ActiveModelTrait, DbErr, Set};

pub async fn save(
    title: &str,
    category: &ResourceCategoryEnum,
    tags: &Option<String>,
    url: &str,
    sort: &Option<i32>
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
