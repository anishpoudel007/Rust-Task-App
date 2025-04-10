use sea_orm::prelude::DateTimeWithTimeZone;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTimeWithTimeZone>,
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskStatusRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTaskPriorityRequest {
    #[validate(length(min = 3, message = "Must have at least 3 characters"))]
    pub priority: String,
}
