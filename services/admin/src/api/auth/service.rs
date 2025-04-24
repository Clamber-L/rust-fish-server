use crate::api::auth::entity::LoginParam;
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::{ApiResult, ExtractJson};
use lib_entity::mysql::prelude::SysUser;
use lib_entity::mysql::sys_user;
use lib_utils::password::verify_password;
use lib_utils::{ResData, ResMessage};
use sea_orm::prelude::Expr;
use sea_orm::{EntityTrait, QueryFilter};

pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<String> {
    if let Some(user) = SysUser::find()
        .filter(Expr::col(sys_user::Column::Username).eq(param.username))
        .one(&state.mysql_client)
        .await?
    {
        if verify_password(&param.password, &user.password) {
            Ok(ResData::ok(user.id))
        } else {
            Ok(ResMessage::error_with_message("密码错误"))
        }
    } else {
        Ok(ResData::ok("123".to_string()))
    }
}

pub async fn user_info(State(state): State<AppState>) -> ApiResult<String> {
    Ok(ResMessage::ok_with_message())
}
