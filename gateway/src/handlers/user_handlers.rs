use axum::{Extension, Json, extract::Path, http::StatusCode, routing::delete};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, prelude::Uuid};

use crate::{entities::{self, user}, models::user_models::{CreateUserModel, FetchUserModel, UpdateUserModel}, utils::errors::AppError};

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>
) -> Result<StatusCode, AppError>{
    println!("->> {:<12} - create_user", user_data.email.to_string());

    // check if user already exist or not
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Email.eq(user_data.email.clone()))
        .one(&db)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    match user {
        Some(_user) => {
            return Err(AppError::UserAlreadyExists)
        },
        None => {

            let new_user = entities::user::ActiveModel {
                username: Set(user_data.username.to_owned()),
                email: Set(user_data.email.to_owned()),
                password: Set(user_data.password.to_owned()),
                ..Default::default() 
            };

            new_user.insert(&db)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(StatusCode::CREATED)
        }
    }
}

pub async fn fetch_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<FetchUserModel>
) -> Result<StatusCode, AppError> {
    println!("->> {:<12} - fetch_user", user_data.email.to_string());

    let user = entities::user::Entity::find()
        .filter(
            Condition::all()
                .add(entities::user::Column::Email.eq(user_data.email))
                .add(entities::user::Column::Password.eq(user_data.password))
        )
        .one(&db)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    match user {
        Some(_user) => {
            Ok(StatusCode::OK)
        }
        None => {
            return Err(AppError::UserNotFound)
        }
    }

}

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
    Json(user_data): Json<UpdateUserModel>
) -> Result<StatusCode, AppError> {
    println!("->> {:<12} - update_user", user_data.username.to_string());
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::UserId.eq(uuid))
        .one(&db)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    match user {
        Some(user) => {
            let mut user_active: entities::user::ActiveModel = user.into();
            user_active.username = Set(user_data.username.to_owned());

            user_active.update(&db)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(StatusCode::CREATED)
        }
        None => {
            return Err(AppError::UserNotFound)
        }
    }

}

pub async fn detele_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    println!("->> {:<12} - detele_user", uuid.to_string());
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::UserId.eq(uuid))
        .one(&db)
        .await
        .map_err(|_| AppError::UserNotFound)?;

    match user {
        Some(_user) => {
            entities::user::Entity::delete_by_id(uuid)
                .exec(&db)
                .await
                .map_err(|_| AppError::InternalServerError)?;

            Ok(StatusCode::ACCEPTED)
        }
        None => {
            return Err(AppError::UserNotFound);
        }
    }
}