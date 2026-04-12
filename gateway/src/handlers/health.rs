// External Crates
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;

// Internal Modules
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

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::{Request, StatusCode}};
    use http_body_util::BodyExt; 
    use serde_json::Value;
    use tower::ServiceExt;

    use crate::routes::chat::create_app;


    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "Ok");
        assert_eq!(json["message"], "Health Check");
    }
}