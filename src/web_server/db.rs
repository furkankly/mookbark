use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn create_db_conn_pool() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env var not found");
    let pool = Database::connect(db_url);
    pool.await.expect("Couldn't create db conn. pool!")
}
