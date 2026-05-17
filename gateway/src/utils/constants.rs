//! This module defines constants and configuration settings for the application.
//! It uses the `dotenv` crate to load environment variables from a `.env` file
//! and the `lazy_static` crate to create static references to these variables for use throughout the application.

use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

// The `DATABASE_URL` constant holds the URL for the database connection, which is loaded from the environment variables.
// The `JWT_SECRET` constant holds the secret key used for JWT authentication, also loaded from the environment variables.
// Both constants are defined using `lazy_static` to ensure they are initialized at runtime
lazy_static! {
    pub static ref DATABASE_URL: String = set_database();
    pub static ref JWT_SECRET: String = set_jwt_secret();
}

// This function loads the environment variables from the .env file and retrieves the DATABASE_URL variable.
fn set_database() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL missing!")
}

// This function loads the environment variables from the .env file and retrieves the JWT_SECRET variable.
fn set_jwt_secret() -> String {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET missing!")
}
