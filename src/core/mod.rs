pub mod database;
pub mod jwt;
pub mod middleware;

use axum::extract::{Path, Query};
use axum::Json;
use axum_extra::extract::WithRejection;
pub use database::*;

use crate::utils::result::HttpResult;
use crate::AppError;

pub type JsonParam<T> = WithRejection<Json<T>, AppError>;
pub type PathParam<T> = WithRejection<Path<T>, AppError>;
pub type QueryParam<T> = WithRejection<Query<T>, AppError>;

pub type ApiResult<T> = Result<HttpResult<T>, AppError>;

pub fn generate_snowflake_id() -> Result<String, AppError> {
    Ok(sonyflake::Sonyflake::new()?.next_id()?.to_string())
}
