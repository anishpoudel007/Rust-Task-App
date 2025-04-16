use std::sync::Arc;

use crate::{
    api_response::JsonResponse,
    auth::jwt::{create_user_token, UserToken},
    error::AppError,
    form::user_form::{CreateUserRequest, UserLogin},
    models::_entities::{user, user_profile},
    serializer::UserWithProfileSerializer,
    utils::verify_password,
    AppState,
};

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use validator::Validate;

pub async fn get_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/logout", post(logout))
        .route("/auth/register", post(register))
}

pub async fn get_login_route() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(login))
}

pub async fn get_logout_route() -> Router<Arc<AppState>> {
    Router::new().route("/logout", post(logout))
}

pub async fn get_register_route() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register))
}

#[axum::debug_handler]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(user_login): Json<UserLogin>,
) -> Result<impl IntoResponse, AppError> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(user_login.username))
        .one(&app_state.db)
        .await?
        .ok_or(AppError::GenericError("User not found.".to_string()))?;

    if !verify_password(&user.password, &user_login.password)? {
        return Err(AppError::GenericError("Invalid user".to_string()));
    }

    let access_token = create_user_token(&user.email, 10).await;
    let refresh_token = create_user_token(&user.email, 1440).await;

    let user_token = UserToken {
        access_token,
        refresh_token: Some(refresh_token),
    };

    Ok(JsonResponse::data(user_token, None))
}

pub async fn logout() {}

#[axum::debug_handler]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    Json(user_request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    user_request.validate()?;

    let user_with_profile = app_state
        .db
        .transaction::<_, (user::Model, Option<user_profile::Model>), sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                let user = user::ActiveModel::from(user_request.clone())
                    .insert(txn)
                    .await?;

                let user_profile = user_profile::ActiveModel {
                    id: NotSet,
                    user_id: Set(user.id),
                    address: Set(Some(user_request.address)),
                    mobile_number: Set(Some(user_request.mobile_number)),
                }
                .insert(txn)
                .await?;

                Ok((user, Some(user_profile)))
            })
        })
        .await
        .map_err(|e| AppError::GenericError(e.to_string()))?; // should be database error

    let user_serializer = UserWithProfileSerializer::from(user_with_profile);

    Ok(JsonResponse::data(user_serializer, None))
}
