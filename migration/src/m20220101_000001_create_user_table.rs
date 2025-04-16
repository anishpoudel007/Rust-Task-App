use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name).string_len(100))
                    .col(string(User::Username).unique_key().string_len(50))
                    .col(string(User::Email).unique_key().string_len(320))
                    .col(string(User::Password).string_len(128))
                    .col(
                        timestamp_with_time_zone(User::DateCreated)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(User::DateUpdated))
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

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Username,
    Email,
    Password,
    DateCreated,
    DateUpdated,
}
