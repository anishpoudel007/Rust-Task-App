use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskLabel::Table)
                    .if_not_exists()
                    .col(pk_auto(TaskLabel::Id))
                    .col(integer(TaskLabel::TaskId))
                    .col(integer(TaskLabel::LabelId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-label-task_id")
                            .from(TaskLabel::Table, TaskLabel::TaskId)
                            .to(Task::Table, Task::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-label-label_id")
                            .from(TaskLabel::Table, TaskLabel::LabelId)
                            .to(Label::Table, Label::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskLabel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TaskLabel {
    Table,
    Id,
    TaskId,
    LabelId,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Label {
    Table,
    Id,
}
