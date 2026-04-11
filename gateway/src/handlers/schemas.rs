use serde::{Serialize, Deserialize};


// invoking
#[derive(Serialize)]
pub struct InvokeResponse {
    pub status: String,
    pub message: String,
    pub prompt: String,
    pub result: String,
}

#[derive(Deserialize)]
pub struct InvokeRequest {
    pub prompt: String,
}