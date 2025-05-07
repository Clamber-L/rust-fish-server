pub mod password;
pub mod result;

use crate::result::{error_result, HttpResult};
use axum::http::{header, HeaderValue};
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use serde::Serialize;

pub fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response {
    match serde_json::to_string(body) {
        Ok(json) => {
            let mut response = (status, json).into_response();
            response.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            response
        }
        Err(err) => {
            // fallback error response
            tracing::error!("JSON serialization error: {:?}", err);
            let fallback: HttpResult<()> = error_result("Internal Server Error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::to_string(&fallback).unwrap(),
            )
                .into_response()
        }
    }
}
