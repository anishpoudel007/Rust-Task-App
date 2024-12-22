use serde::Serialize;

use crate::models::_entities::{task, user, user_profile};

#[derive(Debug, Serialize)]
pub struct UserSerializer {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
}

impl From<user::Model> for UserSerializer {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            username: value.username,
            email: value.email,
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
    pub uuid: String,
    pub date_created: chrono::naive::NaiveDateTime,
    pub date_updated: Option<String>,
}

impl From<task::Model> for TaskSerializer {
    fn from(value: task::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            status: value.status,
            uuid: value.uuid,
            date_created: value.date_created,
            date_updated: value.date_updated,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PermissionSerializer {
    pub id: i32,
    pub name: String,
    pub code_name: String,
}
