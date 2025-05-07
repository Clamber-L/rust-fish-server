pub mod request_entity;

use crate::json_response;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResult<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> HttpResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 200,
            message: String::from("成功"),
            data: Some(data),
        }
    }

    pub fn error_with_token(message: &str) -> Self {
        HttpResult {
            code: 430,
            message: message.to_owned(),
            data: None,
        }
    }

    pub fn ok_with_message() -> Self {
        HttpResult {
            code: 200,
            message: String::from("成功"),
            data: None,
        }
    }
}

pub fn ok_result<T>(data: T) -> HttpResult<T> {
    HttpResult {
        code: 200,
        message: String::from("成功"),
        data: Some(data),
    }
}

pub fn error_result<T>(message: &str) -> HttpResult<T> {
    HttpResult {
        code: 555,
        message: message.to_owned(),
        data: None,
    }
}

impl<T> IntoResponse for HttpResult<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        json_response(
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK),
            &self,
        )
    }
}
