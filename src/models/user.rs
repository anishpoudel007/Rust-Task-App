use sea_orm::{ActiveModelBehavior, ConnectionTrait, DbErr, Related, RelationDef, RelationTrait};

use super::_entities::user::ActiveModel;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = chrono::Utc::now();

        if insert && self.date_created.is_not_set() {
            let mut this = self;
            this.date_created = sea_orm::ActiveValue::Set(now.into());
            Ok(this)
        } else if !insert && self.date_updated.is_unchanged() {
            let mut this = self;
            this.date_updated = sea_orm::ActiveValue::Set(Some(now.into()));
            Ok(this)
        } else {
            Ok(self)
        }
    }
}
