use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use mistralai_client::v1::{chat::{ChatMessage, ChatMessageRole, ChatParams}, constants::Model};

use crate::services::llm_client::create_client;


#[derive(Serialize)]
struct InvokeResponse {
    status: String,
    message: String,
    prompt: String,
    result: String,
}

// This handler is responsible for invoking the LLM (Language Model) with a given prompt and returning the generated result.
// It takes a string slice `prompt` as input, which represents the user's message or query to be sent to the LLM.
// The handler calls the `invoke_llm` function, which interacts with the LLM client to generate a response based on the provided prompt.
// If the LLM invocation is successful, it returns a JSON response with a status of "Success", a message indicating that the result was generated, 
// the original prompt, and the generated result.
// If the LLM invocation fails, it logs the error and returns a JSON response with a status of "Error",
// a message indicating the failure, the original prompt, and an empty result string
pub async fn invoking(prompt: &str) -> impl IntoResponse {
    let result = match invoke_llm(prompt) {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("LLM Error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InvokeResponse {
                    status: "Error".to_string(),
                    message: format!("Failed to generate result: {}", e),
                    prompt: prompt.to_string(),
                    result: "".to_string(),
                }),
            ).into_response();
        }
    };

    tracing::info!("Result successfully generated for prompt: {}", prompt);
    (
        StatusCode::CREATED,
        Json(InvokeResponse {
            status: "Success".to_string(),
            message: "Result generated".to_string(),
            prompt: prompt.to_string(),
            result: result, // Move the owned String here
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

fn invoke_llm(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let new_client = create_client().map_err(|e| {
        tracing::error!("Failed to create LLM client: {}", e);
        e
    })?;

    let model = Model::MistralTiny;

    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: prompt.to_string(),
        tool_calls: None,
    }];

    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };

    match new_client.chat(model, messages, Some(options)) {
        Ok(result) => {
            let content = result.choices[0].message.content.clone();
            tracing::info!("LLM invocation successful");
            Ok(content)
        }
        Err(e) => {
            tracing::error!("LLM chat error: {:?}", e);
            Err(Box::new(e))
        }
    }
}


