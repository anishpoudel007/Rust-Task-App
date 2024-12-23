//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub description: String,
    #[sea_orm(column_type = "custom(\"enum_text\")")]
    pub status: String,
    #[sea_orm(column_type = "custom(\"enum_text\")")]
    pub priority: String,
    #[sea_orm(unique)]
    pub uuid: String,
    pub due_date: Option<DateTimeWithTimeZone>,
    pub date_created: DateTimeWithTimeZone,
    pub date_updated: Option<DateTimeWithTimeZone>,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
