use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::instrument::WithSubscriber;

pub async fn mysql_pool(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url)
        .with_current_subscriber()
        .await
}
