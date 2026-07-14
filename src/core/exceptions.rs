use crate::domain::exceptions::DomainError;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Domain Error: {0}")]
    Domain(#[from] DomainError),

    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Network connection failed. Is Polar running? Details: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: Invalid RPC username or password.")]
    AuthFailed,

    #[error("RPC Error ({code}): {message}")]
    RpcError { code: i64, message: String },

    #[error("JSON Serialization Error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP Transport Error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Execution Error: {0}")]
    Execution(String),
}