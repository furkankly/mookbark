use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221011_000003_create_closure_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Closure::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Closure::Ancestor).string().not_null())
                    .col(ColumnDef::new(Closure::Descendant).string().not_null())
                    .col(
                        ColumnDef::new(Closure::InsertionOrder)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Closure::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Closure {
    Table,
    Ancestor,
    Descendant,
    InsertionOrder,
}
