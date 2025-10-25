use thiserror::Error;

#[derive(Debug, Error)]
pub enum JWTError {
    #[error("Failed to generate tokens")]
    FailedGenerateTokens,

    #[error("Failed to generate refresh tokens")]
    FailedGenerateRefreshToken,

    #[error("Failed to generate access token")]
    FailedGenerateAccessToken,

    #[error("Invalid or expired refresh token")]
    InvalidRefreshToken,

    #[error("Failed to decode and validate token")]
    FailedDecode,
}
