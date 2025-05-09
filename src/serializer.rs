use serde::Serialize;

use crate::models::_entities::{label, task, user, user_profile};

#[derive(Debug, Serialize)]
pub struct UserSerializer {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub date_created: String,
    pub date_updated: Option<String>,
}

impl From<user::Model> for UserSerializer {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
            date_created: value.date_created.to_string(),
            date_updated: value.date_updated.map(|v| v.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserProfileSerializer {
    pub id: i32,
    pub address: Option<String>,
    pub mobile_number: Option<String>,
}

impl From<user_profile::Model> for UserProfileSerializer {
    fn from(value: user_profile::Model) -> Self {
        Self {
            id: value.id,
            address: value.address,
            mobile_number: value.mobile_number,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserWithProfileSerializer {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub date_created: String,
    pub date_updated: Option<String>,
    pub profile: Option<UserProfileSerializer>,
}

impl From<(user::Model, Option<user_profile::Model>)> for UserWithProfileSerializer {
    fn from(value: (user::Model, Option<user_profile::Model>)) -> Self {
        let (user, profile) = value;

        let profile_serializer = profile.map(UserProfileSerializer::from);

        Self {
            id: user.id,
            name: user.name,
            username: user.username,
            email: user.email,
            date_created: user.date_created.to_string(),
            date_updated: user.date_updated.map(|v| v.to_string()),
            profile: profile_serializer,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TaskSerializer {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub uuid: String,
    pub due_date: Option<String>,
    pub date_created: String,
    pub date_updated: Option<String>,
}

impl From<task::Model> for TaskSerializer {
    fn from(value: task::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status,
            priority: value.priority,
            uuid: value.uuid,
            due_date: value.due_date.map(|v| v.to_string()),
            date_created: value.date_created.to_string(),
            date_updated: value.date_updated.map(|v| v.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FullTaskSerializer {
    pub task: TaskSerializer,
    pub labels: Vec<LabelSerializer>,
}

#[derive(Debug, Serialize)]
pub struct LabelSerializer {
    pub id: i32,
    pub title: String,
}

impl From<label::Model> for LabelSerializer {
    fn from(value: label::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}
