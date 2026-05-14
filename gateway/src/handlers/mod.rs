use axum::{http::StatusCode, response::IntoResponse};

pub mod auth_handlers;
pub mod user_handlers;




pub async fn test_handler() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Home").into_response()
}

