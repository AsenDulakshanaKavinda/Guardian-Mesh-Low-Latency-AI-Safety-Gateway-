use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

// This is a simple handler for the home page.
// It returns a JSON response with a status and message.


#[derive(Serialize)]
struct HomeResponse {
    status: String,
    message: String
}


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