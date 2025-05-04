use crate::api::user::entity::LoginParam;
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::jwt::{generate_jwt, JwtUserBuilder};
use lib_core::{ApiResult, ExtractJson};
use lib_entity::mysql::app_user;
use lib_entity::mysql::prelude::AppUser;
use lib_utils::{ResData, ResMessage};
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, QueryFilter, QueryTrait};

pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<String> {
    println!("{:?}", param);
    let result = AppUser::find()
        .filter(Expr::col(app_user::Column::Phone).eq(param.phone))
        .one(&state.mysql_client)
        .await?;
    println!("result: {:?}", result);
    if let Some(user) = result {
        let token = generate_jwt(JwtUserBuilder::default().id(user.id).build().unwrap());
        Ok(ResData::ok(token))
    } else {
        Ok(ResMessage::error_with_message("Invalid user ID"))
    }
}
