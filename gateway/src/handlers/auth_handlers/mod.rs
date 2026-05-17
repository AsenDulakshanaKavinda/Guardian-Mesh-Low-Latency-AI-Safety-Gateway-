use serde::Serialize;

pub mod helper;
pub mod login_handler;
pub mod register_handler;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}
