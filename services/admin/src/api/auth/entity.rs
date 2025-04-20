use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}
