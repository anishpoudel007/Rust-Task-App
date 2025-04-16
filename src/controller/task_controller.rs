use std::{clone, collections::HashMap, sync::Arc};

use axum::{
    extract::{OriginalUri, Path, Query, State},
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait as _,
};
use validator::Validate;

use crate::{
    api_response::{JsonResponse, ResponseMetadata},
    error::AppError,
    form::task_form::{
        CreateTaskRequest, UpdateTaskPriorityRequest, UpdateTaskRequest, UpdateTaskStatusRequest,
    },
    models::_entities::{label, task, task_label, user},
    serializer::{FullTaskSerializer, LabelSerializer, TaskSerializer},
    AppState,
};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_tasks).post(create_task))
        .route(
            "/{task_uuid}",
            get(get_task).put(update_task).delete(delete_task),
        )
        .route("/{task_uuid}/full", get(get_task_full_details))
        .route("/{task_uuid}/update_status", put(update_task_status))
        .route("/{task_uuid}/update_priority", put(update_task_priority))
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
    Json(payload): Json<CreateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let task_model = app_state
        .db
        .transaction::<_, task::Model, sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let uuid_v4 = uuid::Uuid::new_v4().to_string();

                let task_model = task::ActiveModel {
                    id: NotSet,
                    title: Set(payload.title),
                    description: Set(payload.description),
                    status: Set(payload.status),
                    priority: Set(payload.priority),
                    uuid: Set(uuid_v4),
                    due_date: payload.due_date.map_or_else(|| NotSet, |v| Set(Some(v))),
                    date_created: NotSet,
                    date_updated: NotSet,
                    user_id: Set(user_model.id),
                }
                .insert(txn)
                .await?;

                let task_labels: Vec<task_label::ActiveModel> = user_model
                    .find_related(label::Entity)
                    .filter(label::Column::Title.is_in(payload.labels))
                    .all(txn)
                    .await?
                    .iter()
                    .map(|label| task_label::ActiveModel {
                        id: NotSet,
                        task_id: Set(task_model.id),
                        label_id: Set(label.id),
                    })
                    .collect();

                if !task_labels.is_empty() {
                    task_label::Entity::insert_many(task_labels)
                        .exec(txn)
                        .await?;
                }

                Ok(task_model)
            })
        })
        .await
        .map_err(|e| AppError::GenericError(e.to_string()))?; // should be database error

    Ok(JsonResponse::data(TaskSerializer::from(task_model), None))
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
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?
        .into();

    Ok(JsonResponse::data(task, None))
}

pub async fn get_task_full_details(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let task = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?;

    let labels: Vec<LabelSerializer> = task
        .find_related(label::Entity)
        .all(&app_state.db)
        .await?
        .iter()
        .map(|label| LabelSerializer::from(label.clone()))
        .collect();

    let full_task_serializer = FullTaskSerializer {
        task: TaskSerializer::from(task),
        labels,
    };

    Ok(JsonResponse::data(full_task_serializer, None))
}

pub async fn update_task(
    State(app_state): State<Arc<AppState>>,
    Path(task_uuid): Path<String>,
    Extension(user_model): Extension<user::Model>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<impl IntoResponse, AppError> {
    let task = user_model
        .find_related(task::Entity)
        .filter(task::Column::Uuid.eq(task_uuid))
        .one(&app_state.db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?;

    // update labels start
    let assigned_labels: Vec<String> = task
        .find_related(label::Entity)
        .filter(label::Column::Title.is_in(payload.labels.clone()))
        .all(&app_state.db)
        .await?
        .into_iter()
        .map(|label| label.title)
        .collect();

    let unassigned_labels: Vec<String> = payload
        .labels
        .into_iter()
        .filter(|label| !assigned_labels.contains(label))
        .collect();

    let unassigned_labels = task
        .find_related(label::Entity)
        .filter(task::Column::Title.is_in(unassigned_labels))
        .all(&app_state.db)
        .await?;

    let task_labels: Vec<task_label::ActiveModel> = unassigned_labels
        .iter()
        .map(|label| task_label::ActiveModel {
            id: NotSet,
            task_id: Set(task.id),
            label_id: Set(label.id),
        })
        .collect();

    let task_model = app_state
        .db
        .transaction::<_, task::Model, sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                if !task_labels.is_empty() {
                    task_label::Entity::insert_many(task_labels)
                        .exec(txn)
                        .await?;
                }

                let mut task: task::ActiveModel = task.into();
                task.title = Set(payload.title);
                task.description = Set(payload.description.unwrap());
                task.status = Set(payload.status);
                task.due_date = payload.due_date.map_or_else(|| NotSet, |v| Set(Some(v)));
                task.user_id = Set(user_model.id);

                task.update(txn).await
            })
        })
        .await
        .map_err(|e| AppError::GenericError(e.to_string()))?; // should be database error

    // update labels end

    Ok(JsonResponse::data(TaskSerializer::from(task_model), None))
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
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?;

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
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?
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
        .ok_or(sea_orm::DbErr::RecordNotFound("Task not found.".into()))?
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
