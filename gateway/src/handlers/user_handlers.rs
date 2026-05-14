use axum::{Extension, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{entities, models::user_models::{CreateUserModel, UserModel}};






pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>
) {
    // check if user already exist or not
    let user = entities::user::Entity::find()
    

    // if not exitst - create a new user

    // if exit - error
    todo!();
}

pub async fn fetch_user(

) {
    todo!();
}

pub async fn update_user(

) {
    todo!();
}

pub async fn detele_user(

) {
    todo!();
}