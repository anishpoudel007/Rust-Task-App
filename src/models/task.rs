use sea_orm::{ActiveModelBehavior, Related, RelationDef, RelationTrait as _};

use super::_entities::{
    label,
    task::{ActiveModel, Entity},
    task_label,
};

impl Related<label::Entity> for Entity {
    fn to() -> RelationDef {
        task_label::Relation::Label.def()
    }
    fn via() -> Option<RelationDef> {
        Some(task_label::Relation::Task.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
