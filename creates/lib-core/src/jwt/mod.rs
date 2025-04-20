use std::collections::BTreeMap;

use anyhow::Result;
use derive_builder::Builder;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::AppError;

const JWT_SECRET: &str = "jwt_secret";

#[derive(Debug, Serialize, Deserialize, Builder, Default, Clone)]
#[builder(setter(into))]
pub struct JwtUser {
    pub id: String,
}

pub fn generate_jwt(user: JwtUser) -> String {
    let mut claims = BTreeMap::new();
    claims.insert("id", user.id);

    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes()).unwrap();

    claims.sign_with_key(&key).unwrap()
}

pub fn verification_jwt(token: &str) -> Result<JwtUser> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(JWT_SECRET.as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;

    let id = claims.get("id");
    if let Some(id) = id {
        return Ok(JwtUserBuilder::default().id(id).build()?);
    }
    Err(AppError::InternalServerError.into())
}
