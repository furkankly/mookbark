use crate::server::db;
use crate::services;

use rocket::http::Status;
use rocket_db_pools::Connection;

#[delete("/<id>")]
pub async fn delete_entity(conn: Connection<db::Db>, id: &str) -> Result<(), Status> {
    services::delete_entity(&conn, id).await.unwrap();
    return Ok(());
}
