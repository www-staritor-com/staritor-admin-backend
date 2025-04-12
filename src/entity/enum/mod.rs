use rocket::serde::{Deserialize, Serialize};
use sea_orm::prelude::StringLen;
use sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(32))")]
pub enum ResourceCategoryEnum {
    /// 1. 资源
    #[sea_orm(string_value = "resource")]
    Resource,
    /// 2. 文章
    #[sea_orm(string_value = "article")]
    Article,
    /// 3. 视频
    #[sea_orm(string_value = "video")]
    Video,
    /// 4. 音频
    #[sea_orm(string_value = "audio")]
    Audio,
    /// 5. 图片
    #[sea_orm(string_value = "image")]
    Image,
    /// 6. 文档
    #[sea_orm(string_value = "document")]
    Document,
}
