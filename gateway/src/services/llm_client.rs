
use gemini_rust::{Gemini};
use crate::utils::read_env::read_env;

// Loads the Gemini client by reading the API key from the environment variables.
// # Returns
// * `Ok(Gemini)` - The Gemini client if the API key is successfully read and the client is created.
// * `Err(Box<dyn std::error::Error>)` - An error if the API key cannot be read or the client cannot be created.
// # Example
// ```
// let gemini_client = load_gemini_client().await.expect("Failed to load Gemini client");
// ```


pub async fn load_gemini_client() -> Result<Gemini, Box<dyn std::error::Error>> {
    // read api key
    let api_key = read_env("GEMINI_API_KEY").await?;

    // create a client
    let client = Gemini::new(api_key)?;
    Ok(client)
}