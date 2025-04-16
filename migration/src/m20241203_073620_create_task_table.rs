use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Title).string_len(400))
                    .col(text(Task::Description))
                    .col(string(Task::Status).default("pending"))
                    .col(string(Task::Priority).default("low"))
                    .col(string(Task::Uuid).unique_key())
                    .col(timestamp_with_time_zone_null(Task::DueDate))
                    .col(
                        timestamp_with_time_zone(Task::DateCreated)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Task::DateUpdated))
                    .col(integer(Task::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-user_id")
                            .from(Task::Table, Task::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE OR REPLACE FUNCTION update_date_updated_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.date_updated := CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    UserId,
    Title,
    Description,
    Status,
    Uuid,
    Priority,
    DueDate,
    DateCreated,
    DateUpdated,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
