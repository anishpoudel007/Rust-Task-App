use sea_orm::{EnumIter, Iterable};
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
                    .col(string(Task::Title))
                    .col(string(Task::Description))
                    .col(
                        enumeration(Task::Status, StatusEnum::Table, StatusEnum::iter().skip(1))
                            .default(StatusEnum::Pending.to_string())
                            .check(
                                Expr::col(Task::Status)
                                    .is_in(StatusEnum::iter().skip(1).map(|item| item.to_string())),
                            ),
                    )
                    .col(
                        enumeration(
                            Task::Priority,
                            PriorityEnum::Table,
                            PriorityEnum::iter().skip(1),
                        )
                        .default(PriorityEnum::High.to_string())
                        .check(
                            Expr::col(Task::Priority)
                                .is_in(PriorityEnum::iter().skip(1).map(|item| item.to_string())),
                        ),
                    )
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
                "CREATE TRIGGER set_date_updated
                AFTER UPDATE ON task
                FOR EACH ROW
                BEGIN
                    UPDATE task SET date_updated = CURRENT_TIMESTAMP WHERE id = NEW.id;
                END;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
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

#[derive(Iden, EnumIter)]
enum StatusEnum {
    Table,
    Pending,
    InProgress,
    Completed,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(Iden, EnumIter)]
enum PriorityEnum {
    Table,
    Low,
    Medium,
    High,
}
