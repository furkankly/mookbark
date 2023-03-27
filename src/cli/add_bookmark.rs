use crate::entities::prelude::Container;
use crate::services;

use sea_orm::{DatabaseConnection, DbErr, EntityTrait, TransactionTrait};

pub async fn add_bookmark(
    db: &DatabaseConnection,
    container_name: Option<&str>,
    url: &str,
) -> Result<(), DbErr> {
    let container_name = container_name.unwrap_or("root");
    let txn = db.begin().await?;

    if container_name == "root" {
        services::add_bookmark(&txn, container_name, url).await?;
    } else {
        let container_find_query = Container::find_by_id(container_name.to_owned())
            .one(&txn)
            .await?;
        match container_find_query {
            Some(_container_model) => {
                services::add_bookmark(&txn, container_name, url).await?;
            }
            None => {
                services::add_container(&txn, "root", container_name).await?;
                services::add_bookmark(&txn, container_name, url).await?;
            }
        }
    }
    txn.commit().await?;
    return Ok(());
}
