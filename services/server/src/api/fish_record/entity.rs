use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FishRecordPageParam {
    pub page_num: u64,

    pub page_size: u64,

    pub name: Option<String>,
}
