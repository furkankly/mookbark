use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, DbErr};
use std::env;
pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let db = sea_orm::Database::connect(database_url).await?;
    println!("{:?}\n", db);

    Ok(db)
}
