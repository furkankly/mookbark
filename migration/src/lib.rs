pub use sea_orm_migration::prelude::*;

mod m20242002_000001_create_bookmark_table;
mod m20242002_000002_create_container_table;
mod m20242002_000003_create_closure_table;
mod m20242002_000004_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20242002_000001_create_bookmark_table::Migration),
            Box::new(m20242002_000002_create_container_table::Migration),
            Box::new(m20242002_000003_create_closure_table::Migration),
            Box::new(m20242002_000004_create_user_table::Migration),
        ]
    }
}
