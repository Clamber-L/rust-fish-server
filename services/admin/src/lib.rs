use crate::api::sys_auth_router;
use crate::app_state::AppState;
use anyhow::Result;
use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::Router;
use lib_core::mysql_pool;
use sea_orm::ColIdx;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod app_state;

static MYSQL_URL: &'static str = "mysql://root:Lsw%400516@47.95.179.146:3306/fish";

async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn init_router() -> Result<Router> {
    let mysql_client = mysql_pool(MYSQL_URL).await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_credentials(false);

    let state = AppState::new(mysql_client);

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/auth", sys_auth_router())
        .layer(cors)
        .with_state(state);

    Ok(app)
}
