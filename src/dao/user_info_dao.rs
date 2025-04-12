use crate::dao;
use crate::dao::mapper::prelude::{
    UserInfoActiveModel, UserInfoColumn, UserInfoMapper, UserInfoPo,
};
use crate::entity::base::request::PageRequest;
use crate::entity::base::response::Page;
use crate::entity::req::user_info_req::PageReq;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

pub async fn find_by_code(code: &str) -> Result<Option<UserInfoPo>, DbErr> {
    UserInfoMapper::find()
        .filter(UserInfoColumn::Code.eq(code))
        .filter(UserInfoColumn::Deleted.eq(false))
        .one(dao::conn())
        .await
}

pub async fn page(req: &PageRequest<PageReq<'_>>) -> Result<Page<UserInfoPo>, DbErr> {
    let page = req.page();
    let size = req.size();
    let mut opt = UserInfoMapper::find().filter(UserInfoColumn::Deleted.eq(false));
    if let Some(v) = &req.data {
        if !v.fuzzy.is_empty() {
            opt = opt
                .filter(UserInfoColumn::Code.contains(v.fuzzy))
                .filter(UserInfoColumn::Name.contains(v.fuzzy));
        }
    }

    let total = opt.clone().count(dao::conn()).await?;
    let data = opt
        .order_by(UserInfoColumn::Id, sea_orm::Order::Asc)
        .paginate(dao::conn(), size)
        .fetch_page(page)
        .await?;

    Ok(Page::new(page, size, total, Some(data)))
}

pub async fn save(code: &str, password: &str, name: &str) -> Result<UserInfoPo, DbErr> {
    let user_info = UserInfoActiveModel {
        code: Set(code.to_owned()),
        password: Set(password.to_owned()),
        name: Set(name.to_owned()),
        deleted: Set(false),
        ..Default::default()
    };

    user_info.insert(dao::conn()).await
}

pub async fn delete_by_code(code: &str) -> Result<bool, DbErr> {
    let model = find_by_code(code).await?;
    match model {
        None => Ok(false),
        Some(po) => {
            let mut update: UserInfoActiveModel = po.into();
            update.deleted = Set(true);

            match update.update(dao::conn()).await {
                Ok(_) => Ok(true),
                Err(err) => Err(err),
            }
        }
    }
}
