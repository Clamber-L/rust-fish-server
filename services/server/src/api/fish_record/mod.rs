mod entity;
mod service;

use crate::api::fish_record::service::{fish_record, fish_record_by_page};
use crate::app_state::AppState;
use axum::routing::{get, post};
use axum::Router;
use service::inset_record;

pub fn fish_record_router() -> Router<AppState> {
    let fish_router = Router::new()
        .route("/list", get(fish_record))
        .route("/list_page", get(fish_record_by_page))
        .route("/insert", post(inset_record));
    fish_router
}
