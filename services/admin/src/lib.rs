use crate::app_state::AppState;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use lib_core::mysql_pool;
use sea_orm::ColIdx;
use tower_http::cors::{Any, CorsLayer};

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
        .allow_headers(Any);

    let state = AppState::new(mysql_client);

    let app = Router::new()
        .route("/", get(hello_world))
        .layer(cors)
        .with_state(state);

    Ok(app)
}
