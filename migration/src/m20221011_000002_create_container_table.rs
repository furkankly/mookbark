use sea_orm_migration::prelude::*;

// use super::m20221011_000003_create_relationship_table::Relationship;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221011_000002_create_container_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Container::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Container::Name)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Container::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Container {
    Table,
    Name,
}
