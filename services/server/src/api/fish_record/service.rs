use crate::app_state::AppState;
use axum::extract::State;
use chrono::Utc;
use lib_core::{generate_snowflake_id, ApiResult, ExtractJson, ExtractQuery};
use lib_entity::mysql::fish_record;
use lib_entity::mysql::fish_record::Model;
use lib_entity::mysql::prelude::FishRecord;
use lib_utils::result::request_entity::PageResult;
use lib_utils::{ResData, ResMessage};
use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait, QueryOrder, Set};
use serde::Deserialize;

use super::entity::FishRecordPageParam;

pub async fn fish_record(State(state): State<AppState>) -> ApiResult<Vec<Model>> {
    let connection = &state.mysql_client;

    let result = FishRecord::find().all(connection).await?;
    Ok(ResData::ok(result))
}

pub async fn fish_record_by_page(
    State(state): State<AppState>,
    ExtractQuery(param): ExtractQuery<FishRecordPageParam>,
) -> ApiResult<PageResult<Model>> {
    let paginate = FishRecord::find()
        .order_by_desc(fish_record::Column::Id)
        .paginate(&state.mysql_client, param.page_size);
    let items_and_pages_number = paginate.num_items_and_pages().await?;

    let items = paginate.fetch_page(param.page_num - 1).await?;
    let page_result = PageResult::new(
        param.page_num,
        param.page_size,
        items_and_pages_number,
        items,
    );
    Ok(ResData::ok(page_result))
}

#[derive(Deserialize, Debug, Default)]
pub struct InsertParam {
    pub name: Option<String>,
}

pub async fn inset_record(
    State(state): State<AppState>,
    ExtractJson(param): ExtractJson<InsertParam>,
) -> ApiResult<String> {
    let record = fish_record::ActiveModel {
        id: Set(generate_snowflake_id()?),
        weight: Set(Some("1.2".to_string())),
        harvest_time: Set(Some(Utc::now().naive_local())),
        bait: Set(Some("玉米粒".to_string())),
        image_url: Set(Some(
            "https://cdn.pixabay.com/photo/2017/05/31/00/24/aquarium-2358739_1280.jpg".to_string(),
        )),
        address: Set(Some("南湖水库".to_string())),
        fish_type: Set(Some("草鱼".to_string())),
        ..Default::default()
    };
    record.insert(&state.mysql_client).await?;

    Ok(ResMessage::ok_with_message())
}
