use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "t_user_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub code: String,
    pub password: String,
    pub name: String,
    pub deleted: bool,
    pub created_datetime: NaiveDateTime,
    pub updated_datetime: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}