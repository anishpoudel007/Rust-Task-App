use std::sync::Arc;

use crate::{error::AppError, utils::verify_token, AppState};
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};

pub async fn auth_guard(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized(
            "Authentication credentials were not provided.".into(),
        ))?;

    let user = verify_token(app_state, token).await?;

    request.extensions_mut().insert(user);

    let response = next.run(request).await;

    Ok(response)
}
