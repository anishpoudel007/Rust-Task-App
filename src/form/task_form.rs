use crate::models::_entities::task::ActiveModel;
use sea_orm::DeriveIntoActiveModel;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
pub struct CreateTaskRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: String,
}
