
use mistralai_client::v1::{chat::{ChatMessage, ChatMessageRole, ChatParams}, constants::Model};

use crate::services::llm_client::create_client;

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

pub fn invoke_llm(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
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


