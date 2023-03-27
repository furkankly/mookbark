pub use sea_orm_migration::prelude::*;

mod m20221011_000001_create_bookmark_table;
mod m20221011_000002_create_container_table;
mod m20221011_000003_create_relationship_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221011_000001_create_bookmark_table::Migration),
            Box::new(m20221011_000002_create_container_table::Migration),
            Box::new(m20221011_000003_create_relationship_table::Migration),
        ]
    }
}
