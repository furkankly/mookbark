use crate::server::db;
use crate::services;

use rocket::http::Status;
use rocket_db_pools::Connection;
use sea_orm::TransactionTrait;

#[post("/container/<parent_container_name>/<container_name>")]
pub async fn add_container(
    conn: Connection<db::Db>,
    parent_container_name: &str,
    container_name: &str,
) -> Result<(), Status> {
    let txn = conn.begin().await.unwrap();
    services::add_container(&txn, parent_container_name, container_name)
        .await
        .unwrap();
    txn.commit().await.unwrap();
    return Ok(());
}
