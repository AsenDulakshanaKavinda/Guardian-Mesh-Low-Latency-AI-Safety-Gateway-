use serde::{Serialize, Deserialize};


// invoke
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

// home
#[derive(Serialize)]
pub struct HomeResponse {
    pub status: String,
    pub message: String
}

// health
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub message: String
}

