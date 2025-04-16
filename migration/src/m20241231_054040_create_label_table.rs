use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Label::Table)
                    .if_not_exists()
                    .col(pk_auto(Label::Id))
                    .col(string(Label::Title))
                    .col(integer(Label::UserId))
                    .col(string(Label::Color).default("#FFFFFF"))
                    .index(
                        Index::create()
                            .name("title__user_id__unique_key")
                            .col(Label::Title)
                            .col(Label::UserId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-label-user_id")
                            .from(Label::Table, Label::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Label::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Label {
    Table,
    Id,
    UserId,
    Title,
    Color,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
