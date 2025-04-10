use crate::dao;
use crate::dao::mapper::prelude::{
    UserInfoActiveModel, UserInfoColumn, UserInfoMapper, UserInfoPo,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

pub async fn find_by_code(code: &str) -> Result<Option<UserInfoPo>, DbErr> {
    UserInfoMapper::find()
        .filter(UserInfoColumn::Code.eq(code))
        .filter(UserInfoColumn::Deleted.eq(false))
        .one(dao::conn())
        .await
}

pub async fn save(code: &str, password: &str, name: &str) -> Result<UserInfoPo, DbErr> {
    let user_info = UserInfoActiveModel {
        code: Set(code.to_string()),
        password: Set(password.to_string()),
        name: Set(name.to_string()),
        deleted: Set(false),
        ..Default::default()
    };

    user_info.insert(dao::conn()).await
}

pub async fn delete_by_code(code: &str) -> Result<bool, DbErr> {
    match find_by_code(code).await {
        Ok(Some(po)) => {
            let mut update: UserInfoActiveModel = po.into();
            update.deleted = Set(true);

            match update.update(dao::conn()).await {
                Ok(_) => Ok(true),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
        Ok(None) => Ok(false),
    }
}
