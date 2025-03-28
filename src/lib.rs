mod app_state;
mod core;
mod entity;
mod error;
mod utils;

use anyhow::Result;
use app_state::AppState;
use axum::{extract::State, routing::get, Router};
use chrono::Utc;
use core::*;
use entity::mysql::{
    fish_record::{self, Model},
    prelude::FishRecord,
};
pub use error::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use utils::*;

pub async fn init_router(mysql_url: &str) -> Result<Router> {
    // init mysql pool
    let mysql_client = mysql_pool(mysql_url).await?;
    let state = AppState::new(mysql_client);

    let fish_router = Router::new().route("/list", get(fish_record));

    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/fish_record", fish_router)
        .route("/add", get(inset_record))
        .with_state(state);
    Ok(router)
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fish_record(State(state): State<AppState>) -> ApiResult<Vec<Model>> {
    let connection = &state.mysql_client;

    let result = FishRecord::find().all(connection).await?;
    Ok(ResData::ok(result))
}

async fn inset_record(State(state): State<AppState>) -> ApiResult<String> {
    let record = fish_record::ActiveModel {
        id: Set(generate_snowflake_id()?),
        weight: Set(Some("1.2".to_string())),
        harvest_time: Set(Some(Utc::now().naive_local())),
        bait: Set(Some("玉米粒".to_string())),
        image_url: Set(Some(
            "https://cdn.pixabay.com/photo/2017/05/31/00/24/aquarium-2358739_1280\
        .jpg"
                .to_string(),
        )),
        address: Set(Some("南湖水库".to_string())),
        fish_type: Set(Some("草鱼".to_string())),
        ..Default::default()
    };
    record.insert(&state.mysql_client).await?;

    Ok(Res::ok_with_message())
}
