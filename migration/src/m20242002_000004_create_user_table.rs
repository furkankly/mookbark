use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20242002_000004_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::UserId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::OauthProvider).string().not_null())
                    .col(ColumnDef::new(User::OauthUserId).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Avatar).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    UserId,
    OauthProvider,
    OauthUserId,
    Email,
    Username,
    Avatar,
}
