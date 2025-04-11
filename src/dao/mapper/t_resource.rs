use crate::entity::r#enum::ResourceCategoryEnum;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "t_resource")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub category: ResourceCategoryEnum,
    pub tags: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub url: String,
    pub sort: Option<i32>,
    pub created_datetime: NaiveDateTime,
    pub created_user: Option<String>,
    pub updated_datetime: NaiveDateTime,
    pub updated_user: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
