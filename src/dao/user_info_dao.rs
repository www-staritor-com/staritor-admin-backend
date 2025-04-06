use crate::dao::mapper::user_info_mapper::UserInfoPo;
use crate::dao::RB;
use rbs::Error;

pub async fn select_by_code(code: &str) -> Result<Option<UserInfoPo>, Error> {
    UserInfoPo::select_by_code(&*RB, code).await
}
