use crate::services;

use sea_orm::{DatabaseConnection, DbErr, TransactionTrait};

pub async fn add_container(
    db: &DatabaseConnection,
    parent_container_name: &str,
    container_name: &str,
) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    services::add_container(&txn, parent_container_name, container_name).await?;
    txn.commit().await?;
    return Ok(());
}
