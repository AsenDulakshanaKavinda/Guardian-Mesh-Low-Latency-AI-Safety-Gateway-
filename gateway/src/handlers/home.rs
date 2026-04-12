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
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "Ok");
        assert_eq!(json["message"], "Home Page");
    }
}