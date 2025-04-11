use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TaskStatusEnum)
                    .values(TaskStatusVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(TaskPriorityEnum)
                    .values(TaskStatusVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Title))
                    .col(string(Task::Description))
                    .col(
                        ColumnDef::new(Task::Status)
                            .enumeration(
                                TaskStatusEnum,
                                [
                                    TaskStatusVariants::Completed,
                                    TaskStatusVariants::InProgress,
                                    TaskStatusVariants::Pending,
                                ],
                            )
                            .default(TaskStatusVariants::Pending.to_string()),
                    )
                    .col(
                        ColumnDef::new(Task::Priority)
                            .enumeration(
                                TaskPriorityEnum,
                                [
                                    TaskPriorityVariants::Low,
                                    TaskPriorityVariants::Medium,
                                    TaskPriorityVariants::High,
                                ],
                            )
                            .default(TaskStatusVariants::Pending.to_string()),
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
            .drop_type(Type::drop().name(TaskStatusEnum).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(TaskPriorityEnum).to_owned())
            .await?;

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
struct TaskStatusEnum;

#[derive(DeriveIden, EnumIter)]
enum TaskStatusVariants {
    Pending,
    InProgress,
    Completed,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
struct TaskPriorityEnum;

#[derive(DeriveIden, EnumIter)]
enum TaskPriorityVariants {
    Low,
    Medium,
    High,
}
