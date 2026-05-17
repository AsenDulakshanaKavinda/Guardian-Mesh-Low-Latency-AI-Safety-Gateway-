//! This module provides utility functions for hashing and verifying passwords using the Argon2 algorithm.
//! It defines two main functions: `hash_password` for hashing a plaintext password and `verify_password`
//! for verifying a plaintext password against a stored hash.
//! The module uses the `argon2` crate for password hashing and verification,
//! and it also handles errors using a custom `AppError` type defined in the `utils::errors` module.
//! The `hash_password` function generates a random salt and creates a hash of the provided password,
//! while the `verify_password` function checks if the provided password matches the stored hash by
//! parsing the hash and using the Argon2 algorithm to verify it. Both functions return results that indicate
//! success or failure, allowing for proper error handling in the authentication process.

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::utils::errors::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppError::HashingError)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::InternalServerError)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
