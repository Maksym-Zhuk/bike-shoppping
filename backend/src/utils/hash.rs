use argon2::{
    Argon2,
    password_hash::{
        Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        rand_core::OsRng,
    },
};

use crate::errors::{AppErrors, hash_error::HashError};

pub fn hash_password(password: &str) -> Result<String, AppErrors> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AppErrors::Hash(HashError::FailedHash))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppErrors> {
    let parsed_hash: PasswordHash<'_> =
        PasswordHash::new(hash).map_err(|e| AppErrors::Hash(HashError::FailedParse(e)))?;
    let argon2 = Argon2::default();

    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(e) => Err(AppErrors::Hash(HashError::VerificationError(e))),
    }
}
