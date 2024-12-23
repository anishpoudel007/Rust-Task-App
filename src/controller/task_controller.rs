use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{OriginalUri, Path, Query, State},
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    api_response::{JsonResponse, ResponseMetadata},
    error::AppError,
    form::task_form::{
        CreateTaskRequest, UpdateTaskPriorityRequest, UpdateTaskRequest, UpdateTaskStatusRequest,
    },
    models::_entities::{task, user},
    serializer::TaskSerializer,
    AppState,
};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route(
            "/:task_uuid",
            get(get_task).put(update_task).delete(delete_task),
        )
        .route("/:task_uuid/update_status", put(update_task_status))
        .route("/:task_uuid/update_priority", put(update_task_priority))
}

pub async fn get_router() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new().routes(routes!(get_tasks))
}

#[utoipa::path(
        get,
        path = "",
        tag = "task",
        responses(
            (status = 200, description = "List all todos successfully", body = [TaskSerializer])
        )
    )]
#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    OriginalUri(original_uri): OriginalUri,
    // Extension(user_model): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let tasks: Vec<TaskSerializer> = task::Entity::find()
        .all(&app_state.db)
        .await?
        .into_iter()
        .map(TaskSerializer::from)
        .collect();

    Ok(Json(tasks))
}

#[axum::debug_handler]
pub async fn create_task(
    State(app_state): State<Arc<AppState>>,
    Extension(user_model): Extension<user::Model>,
    Json(task_request): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    task_request.validate()?;

    let mut task_active_model = task_request.into_active_model();

    task_active_model.user_id = Set(user_model.id);
    task_active_model.uuid = Set(uuid::Uuid::new_v4().to_string());

    let task: TaskSerializer = task_active_model.insert(&app_state.db).await?.into();

    Ok(JsonResponse::data(task, None))
}

pub async fn get_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let task: TaskSerializer = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    Ok(JsonResponse::data(task, None))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
    Json(task_request): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut task: task::ActiveModel = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    task.title = Set(task_request.title);
    task.description = Set(task_request.description.unwrap());
    task.status = Set(task_request.status);
    task.user_id = Set(user_model.id);

    let task_serializer: TaskSerializer = task.update(&app_state.db).await?.into();

    Ok(JsonResponse::data(task_serializer, None))
}

pub async fn delete_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let task_model = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

    let res = task_model.delete(&app_state.db).await?;

    println!("{:?}", res);

    Ok(JsonResponse::data(
        None::<String>,
        Some("Task deleted successfully".to_string()),
    ))
}

pub async fn update_task_status(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
    Json(task_request): Json<UpdateTaskStatusRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut task: task::ActiveModel = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    task.status = Set(task_request.status);

    let task_serializer: TaskSerializer = task.update(&app_state.db).await?.into();

    Ok(JsonResponse::data(task_serializer, None))
}

pub async fn update_task_priority(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
    Json(task_request): Json<UpdateTaskPriorityRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut task: task::ActiveModel = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?
        .into();

    task.priority = Set(task_request.priority);

    let task_serializer: TaskSerializer = task.update(&app_state.db).await?.into();

    Ok(JsonResponse::data(task_serializer, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_world() {
        assert_eq!(1, 1);
    }
}
