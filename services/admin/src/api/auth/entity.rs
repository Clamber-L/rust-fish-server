use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}
