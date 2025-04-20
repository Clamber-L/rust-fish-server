use crate::api::auth::entity::LoginParam;
use crate::app_state::AppState;
use axum::extract::State;
use lib_core::{ApiResult, ExtractJson};
use lib_utils::{ResData, ResMessage};
use tracing::info;

#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<LoginParam>,
) -> ApiResult<String> {
    info!("state: {:?}", state);
    info!("user: {:?}", param);
    Ok(ResData::ok("123".to_string()))
}
