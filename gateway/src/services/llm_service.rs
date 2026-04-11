use gemini_rust::client;

use crate::services::llm_client::load_gemini_client;



pub async fn invoke() -> Result<String, Box<dyn std::error::Error>>{
    let client = load_gemini_client().await?;

    let response_with_system = client
        .generate_content()
        .with_system_prompt("You are a helpful assistant specializing in Rust programming.")
        .with_user_message("What makes Rust a good choice for systems programming?")
        .execute()
        .await?;

    Ok(response_with_system.text())
}