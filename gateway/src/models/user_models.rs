
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime
}


#[derive(Serialize, Deserialize)]
pub struct CreateUserModel {
    pub username: String,
    pub email: String,
    pub password: String,
}