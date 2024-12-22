pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_task_table;
mod m20241203_073620_create_user_table;
mod m20241205_064650_create_user_profile_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_task_table::Migration),
            Box::new(m20241203_073620_create_user_table::Migration),
            Box::new(m20241205_064650_create_user_profile_table::Migration),
        ]
    }
}
