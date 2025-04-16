use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::api_response::JsonResponse;

#[derive(Debug)]
pub enum AppError {
    GenericError(String),
    SeaOrm(sea_orm::DbErr),
    Validation(validator::ValidationErrors),
    Unauthorized(String),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(v: sea_orm::DbErr) -> Self {
        Self::SeaOrm(v)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        Self::Validation(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::GenericError(e) => (StatusCode::BAD_REQUEST, e),
            AppError::SeaOrm(db_err) => match db_err {
                sea_orm::DbErr::RecordNotFound(message) => (StatusCode::NOT_FOUND, message),
                sea_orm::DbErr::Exec(runtime_err) => match runtime_err {
                    sea_orm::RuntimeErr::SqlxError(error) => match error {
                        sea_orm::SqlxError::Database(e) => {
                            tracing::error!("Error {:#?}", e);
                            tracing::error!("Source {:#?}", e.constraint());
                            (StatusCode::BAD_REQUEST, e.to_string())
                        }
                        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error".into()),
                    },
                    sea_orm::RuntimeErr::Internal(_) => todo!(),
                },
                _ => (StatusCode::NOT_FOUND, db_err.to_string()),
            },
            AppError::Validation(validation_errors) => {
                (StatusCode::BAD_REQUEST, validation_errors.to_string())
            }
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
        };

        (
            status_code,
            JsonResponse::error(error_message, Some("Error".to_string())),
        )
            .into_response()
    }
}
