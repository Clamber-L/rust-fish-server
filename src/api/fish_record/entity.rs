use serde::Deserialize;

use crate::utils::result::request_entity::Page;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FishRecordPageParam {
    #[serde(rename = "pageNum")]
    pub page_num: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub name: Option<String>,
}
