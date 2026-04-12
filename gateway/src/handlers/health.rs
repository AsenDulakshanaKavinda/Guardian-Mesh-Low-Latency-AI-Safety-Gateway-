use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::handlers::schemas::HealthResponse;

// This is a simple handler for the health check endpoint.
// It returns a JSON response with a status and message.
pub async fn health_check() -> impl IntoResponse {
    tracing::info!("Health check loading ...");
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "Ok".to_string(),
            message: "Health Check".to_string(),
        })
    )
}