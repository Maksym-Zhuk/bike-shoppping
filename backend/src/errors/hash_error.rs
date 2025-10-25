use argon2::password_hash::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HashError {
    #[error("Failed to hash password")]
    FailedHash,

    #[error("Password verification error: {0}")]
    VerificationError(Error),

    #[error("Failed to parse password hash: {0}")]
    FailedParse(Error),
}
