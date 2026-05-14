use axum::{http::StatusCode, response::IntoResponse};

mod auth_handlers;
mod user_handlers;




pub async fn test_handler() -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Home").into_response()
}

