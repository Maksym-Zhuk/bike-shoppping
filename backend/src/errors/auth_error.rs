use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthFailed,

    #[error("Invalid email or password")]
    InvalidEmailORPassword,
}
