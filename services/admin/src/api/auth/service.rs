use crate::api::auth::entity::{LoginParam, LoginResponse};
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::jwt::{generate_jwt, JwtUser};
use lib_core::{ApiResult, ExtractJson};
use lib_entity::mysql::prelude::SysUser;
use lib_entity::mysql::sys_user;
use lib_utils::password::verify_password;
use lib_utils::result::{error_result, ok_result, HttpResult};
use sea_orm::prelude::Expr;
use sea_orm::{EntityTrait, QueryFilter};

pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<LoginResponse> {
    if let Some(user) = SysUser::find()
        .filter(Expr::col(sys_user::Column::Username).eq(param.username))
        .one(&state.mysql_client)
        .await?
    {
        if verify_password(&param.password, &user.password) {
            let access_token = generate_jwt(JwtUser {
                id: user.id.clone(),
            });
            Ok(ok_result(LoginResponse::new(user, access_token)))
        } else {
            Ok(error_result("密码错误"))
        }
    } else {
        Ok(error_result("用户不存在"))
    }
}

pub async fn user_info(State(_state): State<AppState>) -> ApiResult<String> {
    Ok(HttpResult::ok_with_message())
}
