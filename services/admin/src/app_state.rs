use sea_orm::DatabaseConnection;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub mysql_client: DatabaseConnection,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(mysql_client: DatabaseConnection) -> Self {
        Self {
            inner: Arc::new(AppStateInner { mysql_client }),
        }
    }
}
