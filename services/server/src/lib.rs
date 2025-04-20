mod api;
mod app_state;

use crate::api::fish_record::fish_record_router;
use anyhow::Result;
use app_state::AppState;
use axum::{routing::get, Router};
use core::*;
pub use error::*;
use lib_core::mysql_pool;

pub async fn init_router(mysql_url: &str) -> Result<Router> {
    // init mysql pool
    let mysql_client = mysql_pool(mysql_url).await?;
    let state = AppState::new(mysql_client);

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/fish_record", fish_record_router())
        .with_state(state);
    Ok(router)
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
