use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginParam {
    pub phone: String,

    pub verification_code: String,
}
