use crate::api::auth::service::{login, user_info};
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::Router;

mod entity;
mod service;

pub fn sys_auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/user", get(user_info))
}
