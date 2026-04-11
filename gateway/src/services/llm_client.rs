
use mistralai_client::v1::{client::Client};
use crate::utils::read_env::read_env;

// Creates a new Mistral AI client.
// # Returns
// * `Ok(Client)` - A new instance of the Mistral AI client if the API key is successfully read and the client is created.
// * `Err(Box<dyn std::error::Error>)` - An error if the API key cannot be read or the client cannot be created.
// # Example
// ```
// let llm_client = create_client().expect("Failed to create Mistral AI client");
// ```      

pub fn create_client() -> Result<Client, Box<dyn std::error::Error>> {
    let api_key = read_env("MISTRAL_API_KEY")?;

    let llm_client = Client::new(
        Some(api_key),
        None, 
        None, 
        None
    )?;

    Ok(llm_client)

}