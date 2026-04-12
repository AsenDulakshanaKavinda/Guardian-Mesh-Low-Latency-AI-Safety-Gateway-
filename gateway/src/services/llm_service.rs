
use crate::services::llm_client::load_gemini_client;

// Invokes the LLM service by sending the user prompt to the Gemini client and retrieving the generated response.
// # Arguments
// * `user_prompt: &str` - The user prompt to be sent to the LLM service.
// # Returns
// * `Ok(String)` - The generated response from the LLM service if the invocation is successful.
// * `Err(Box<dyn std::error::Error>)` - An error if the invocation fails, such as issues with the Gemini client or the API key.
// # Example
// ```
// let result = invoke("What is Rust?").await.expect("Failed to invoke LLM service");
// println!("LLM Response: {}", result);
// ```  
pub async fn invoke(user_prompt: &str) -> Result<String, Box<dyn std::error::Error>>{
    let client = load_gemini_client().await?;

    let response_with_system = client
        .generate_content()
        .with_system_prompt("You are a helpful assistant specializing in Rust programming.")
        //.with_user_message("What makes Rust a good choice for systems programming?")
        .with_user_message(user_prompt)
        .execute()
        .await?;

    Ok(response_with_system.text())
}