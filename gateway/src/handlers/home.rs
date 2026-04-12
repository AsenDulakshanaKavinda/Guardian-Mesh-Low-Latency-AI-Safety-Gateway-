use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::handlers::schemas::HomeResponse;

// This is a simple handler for the home page.
// It returns a JSON response with a status and message.
pub async fn home() -> impl IntoResponse {
    tracing::info!("Home page loading ...");
    (
        StatusCode::OK,
        Json(HomeResponse {
            status: "Ok".to_string(),
            message: "Home Page".to_string(),
        })
    )
}