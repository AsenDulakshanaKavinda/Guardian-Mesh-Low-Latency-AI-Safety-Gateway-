
use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::{handlers::schemas::{InvokeRequest, InvokeResponse}, services::llm_service::invoke};


// Handles the invocation of the LLM service by processing the incoming request, invoking the service, and returning the appropriate response.
// # Arguments
// * `Json(payload): Json<InvokeRequest>` - The incoming request containing the user prompt.
// # Returns
// * `impl IntoResponse` - The response containing the status, message, original prompt, and the generated result if successful, or an error message if the invocation fails.
// # Example
// ```
// let response = invoking(Json(InvokeRequest { prompt: "What is Rust?".to_string() })).await.into_response();
// ```  
pub async fn invoking(Json(payload): Json<InvokeRequest>) -> impl IntoResponse {
    let user_prompt = payload.prompt;

    let result = match invoke(&user_prompt).await {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("LLM Error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InvokeResponse {
                    status: "Error".to_string(),
                    message: format!("Failed to generate result: {}", e),
                    prompt: user_prompt.to_string(),
                    result: "".to_string(),
                }),
            ).into_response();
            
        }
    };

    tracing::info!("Result successfully generated for prompt");
    (
        StatusCode::CREATED,
        Json(InvokeResponse {
            status: "Success".to_string(),
            message: "Result generated".to_string(),
            prompt: user_prompt.to_string(),
            result: result.to_string(), 
        }),
    ).into_response()
}

