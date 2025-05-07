pub mod database;
pub mod error;
pub mod jwt;
pub mod middleware;

use std::ops::Deref;

use crate::error::AppError;
use axum::extract::{FromRequest, Path, Query, Request};
use axum::{Form, Json};
pub use database::*;
use lib_utils::result::HttpResult;
use serde::de::DeserializeOwned;

pub type ApiResult<T> = Result<HttpResult<T>, AppError>;

pub fn generate_snowflake_id() -> Result<String, AppError> {
    Ok(sonyflake::Sonyflake::new()?.next_id()?.to_string())
}

pub struct ExtractJson<T>(pub T);

impl<T> Deref for ExtractJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ExtractJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send + Sync + 'static + Default,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::JsonDeserializeError(err))?;
        Ok(ExtractJson(data))
    }
}

pub struct ExtractForm<T>(pub T);

impl<T> Deref for ExtractForm<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ExtractForm<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send + Sync + 'static + Default,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data) = Form::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::FormDeserializeError(err))?;
        Ok(ExtractForm(data))
    }
}

pub struct ExtractQuery<T>(pub T);

impl<T> Deref for ExtractQuery<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ExtractQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send + Sync + 'static + Default,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(data) = Query::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::QueryDeserializationError(err))?;
        Ok(ExtractQuery(data))
    }
}

pub struct ExtractPath<T>(pub T);

impl<T> Deref for ExtractPath<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> FromRequest<S> for ExtractPath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send + Sync + 'static + Default,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Path(data) = Path::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::PathDeserializationError(err))?;
        Ok(ExtractPath(data))
    }
}
