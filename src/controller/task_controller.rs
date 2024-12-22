use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{OriginalUri, Path, Query, State},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use validator::Validate;

use crate::{
    api_response::{JsonResponse, ResponseMetadata},
    error::AppError,
    form::task_form::{CreateTaskRequest, UpdateTaskRequest},
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
}

#[axum::debug_handler]
pub async fn get_tasks(
    State(app_state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    OriginalUri(original_uri): OriginalUri,
    Extension(user_model): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let mut task_query = user_model.find_related(task::Entity);

    if let Some(status) = params.get("status") {
        task_query = task_query.filter(task::Column::Status.eq(status))
    }

    let page = params
        .get("page")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(1);

    let task_count = task_query.clone().count(&app_state.db).await?;

    let response_metadata = ResponseMetadata {
        count: task_count,
        per_page: 10,
        total_page: task_count.div_ceil(10),
        current_url: Some(original_uri.to_string()),
        ..Default::default()
    };

    let tasks: Vec<TaskSerializer> = task_query
        .order_by(task::Column::DateCreated, sea_orm::Order::Desc)
        .paginate(&app_state.db, 10)
        .fetch_page(page - 1)
        .await?
        .iter()
        .map(|task| TaskSerializer::from(task.clone()))
        .collect();

    Ok(JsonResponse::paginate(tasks, response_metadata, None))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_world() {
        assert_eq!(1, 1);
    }
}
