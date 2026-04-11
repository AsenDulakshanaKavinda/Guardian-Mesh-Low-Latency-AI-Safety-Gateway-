
use axum::{Json, http::StatusCode, response::IntoResponse};
// use mistralai_client::v1::{chat::{ChatMessage, ChatMessageRole, ChatParams}, client::Client, constants::Model};

use crate::{handlers::schemas::{InvokeRequest, InvokeResponse}, services::llm_service::invoke};



pub async fn invoking(Json(payload): Json<InvokeRequest>) -> impl IntoResponse {
    let user_prompt = payload.prompt;

    let result = match invoke().await {
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


// Invokes the LLM (Language Model) to generate a response based on a user message.
// # Arguments
// * `prompt` - A string slice that holds the user's message or prompt to be sent to the LLM for generating a response.
// # Returns
// * `Ok(String)` - The generated response from the LLM if the invocation is successful.
// * `Err(Box<dyn std::error::Error>)` - An error if the LLM invocation fails, including errors from client creation or the chat method.
// # Example
// ```
// let response = invoke_llm().expect("Failed to invoke LLM");
// println!("LLM Response: {}", response);
// ```



