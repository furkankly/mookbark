use crate::cli;
use crate::server::db;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use termtree::Tree;

#[get("/bookmarks")]
pub async fn get_bookmarks(conn: Connection<db::Db>) -> Result<Json<Tree<String>>, Status> {
    let bookmarks = cli::get_bookmarks::get_bookmarks(&conn).await.unwrap();
    return Ok(Json(bookmarks));
}
