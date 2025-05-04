use crate::api::user::service::login;
use crate::app_state::AppState;
use axum::routing::post;
use axum::Router;

mod entity;
mod service;

pub fn user_routers() -> Router<AppState> {
    let router = Router::new().route("/login", post(login));
    router
}
