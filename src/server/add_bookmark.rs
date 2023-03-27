use crate::server::db;
use crate::services;

use rocket::http::Status;
use rocket_db_pools::Connection;
use sea_orm::TransactionTrait;

#[post("/bookmark/<container_name>/<bookmark_url>")]
pub async fn add_bookmark(
    conn: Connection<db::Db>,
    container_name: &str,
    bookmark_url: &str,
) -> Result<(), Status> {
    let txn = conn.begin().await.unwrap();
    services::add_bookmark(&txn, container_name, bookmark_url)
        .await
        .unwrap();
    txn.commit().await.unwrap();
    return Ok(());
}
