use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Extension, Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
};
use validator::Validate;

use crate::{
    api_response::JsonResponse,
    error::AppError,
    form::label_form::{CreateLabelRequest, UpdateLabelRequest},
    models::_entities::{label, user},
    AppState,
};

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_labels).post(create_label))
        .route(
            "/{label_id}",
            get(get_label).put(update_label).delete(delete_label),
        )
}

pub async fn get_labels(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let labels = label::Entity::find().all(&app_state.db).await?;

    Ok(JsonResponse::data(labels, None))
}

#[axum::debug_handler]
pub async fn create_label(
    State(app_state): State<Arc<AppState>>,
    Extension(user): Extension<user::Model>,
    Json(payload): Json<CreateLabelRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let existing_label = label::Entity::find()
        .filter(label::Column::UserId.eq(user.id))
        .filter(label::Column::Title.eq(&payload.title))
        .one(&app_state.db)
        .await?;

    if existing_label.is_some() {
        return Err(AppError::GenericError("Label already exists.".to_string()));
    }

    let mut label = payload.into_active_model();
    label.user_id = Set(user.id);

    let created_label = label.insert(&app_state.db).await?;

    Ok(JsonResponse::data(created_label, None))
}

#[axum::debug_handler]
pub async fn get_label(
    State(app_state): State<Arc<AppState>>,
    Path(label_id): Path<i32>,
    Extension(user): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let label = user
        .find_related(label::Entity)
        .filter(label::Column::Id.eq(label_id))
        .one(&app_state.db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Label not found.".into()))?;

    Ok(JsonResponse::data(label, None))
}

pub async fn update_label(
    State(app_state): State<Arc<AppState>>,
    Path(label_id): Path<i32>,
    Extension(user): Extension<user::Model>,
    Json(payload): Json<UpdateLabelRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let label = user
        .find_related(label::Entity)
        .filter(label::Column::Id.eq(label_id))
        .one(&app_state.db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Label not found.".into()))?;

    let existing_label = label::Entity::find()
        .filter(label::Column::UserId.eq(user.id))
        .filter(label::Column::Title.eq(&payload.title))
        .filter(label::Column::Title.ne(&label.title))
        .one(&app_state.db)
        .await?;

    if existing_label.is_some() {
        return Err(AppError::GenericError("Label already exists.".to_string()));
    }

    let mut label: label::ActiveModel = label.into();
    label.title = Set(payload.title);

    let updated_label = label.update(&app_state.db).await?;

    Ok(JsonResponse::data(updated_label, None))
}

pub async fn delete_label(
    State(app_state): State<Arc<AppState>>,
    Path(label_id): Path<i32>,
    Extension(user): Extension<user::Model>,
) -> Result<impl IntoResponse, AppError> {
    let label = user
        .find_related(label::Entity)
        .filter(label::Column::Id.eq(label_id))
        .one(&app_state.db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound("Label not found.".into()))?;

    let res = label.delete(&app_state.db).await?;

    println!("{:?}", res);

    Ok(JsonResponse::data(
        None::<String>,
        Some("Label deleted successfully".to_string()),
    ))
}
