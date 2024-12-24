use std::sync::Arc;

use axum::{http::StatusCode, Router};
use controller::task_controller;
use sea_orm::{Database, DatabaseConnection};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

mod api_response;
mod auth;
mod controller;
mod error;
mod form;
mod middlewares;
mod models;
mod serializer;
mod utils;

#[derive(Clone, Debug)]
struct AppState {
    db: DatabaseConnection,
}

const TODO_TAG: &str = "task";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .pretty()
        .with_ansi(true)
        .init();

    let server_address = std::env::var("SERVER_ADDRESS").expect("Server Address not found");

    tracing::info!("Listening on {}", server_address);

    let listener = TcpListener::bind(server_address.clone())
        .await
        .expect("Could not create TCP Listener");

    let app = create_app().await;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn create_app() -> Router {
    // OpenApi Start
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        tags(
            (name = TODO_TAG, description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
                )
            }
        }
    }

    // OpenApi End

    let database_url = std::env::var("DATABASE_URL").expect("Database url not found");

    let db = Database::connect(&database_url)
        .await
        .expect("Cannot connect to a database");

    let app_state = Arc::new(AppState { db });

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/tasks", task_controller::get_router().await)
        // .route_layer(axum::middleware::from_fn_with_state(
        //     app_state.clone(),
        //     middlewares::auth_guard::auth_guard,
        // ))
        .with_state(app_state)
        .fallback(fallback_handler)
        .layer(TraceLayer::new_for_http())
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(Redoc::with_url("/redoc", api.clone()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Alternative to above
        // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api))
}

async fn fallback_handler() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use dotenvy::dotenv;
    use tower::{Service, ServiceExt};

    #[tokio::test]
    async fn hello_world() {
        dotenv().ok();

        let app = create_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tasks")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
