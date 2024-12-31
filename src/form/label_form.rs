use crate::models::_entities::label::ActiveModel;
use sea_orm::DeriveIntoActiveModel;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct CreateLabelRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateLabelRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
}
