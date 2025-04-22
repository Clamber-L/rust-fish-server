use crate::api::auth::entity::LoginParam;
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::{ApiResult, ExtractJson};
use lib_entity::mysql::prelude::SysUser;
use lib_entity::mysql::sys_user;
use lib_utils::password::password_salt_hash;
use lib_utils::{ResData, ResMessage};
use sea_orm::prelude::Expr;
use sea_orm::{EntityTrait, QueryFilter};

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<String> {
    if let Some(user) = SysUser::find()
        .filter(Expr::col(sys_user::Column::Username).eq(param.username))
        .one(&state.mysql_client)
        .await?
    {
        let salt_hash = password_salt_hash(&user.password);
        println!("password:{:?}", salt_hash.0);
        println!("salt:{:?}", salt_hash.1);
        Ok(ResData::ok(user.id.to_string()))
    } else {
        Ok(ResData::ok("123".to_string()))
    }
}
