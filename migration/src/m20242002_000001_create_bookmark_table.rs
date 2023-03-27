use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20242002_000001_create_bookmark_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookmark::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bookmark::Url).string().not_null())
                    .col(ColumnDef::new(Bookmark::UserId).string().not_null())
                    .col(
                        ColumnDef::new(Bookmark::Ingested)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .primary_key(Index::create().col(Bookmark::Url).col(Bookmark::UserId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookmark::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Bookmark {
    Table,
    Url,
    UserId,
    Ingested,
}
