use thiserror::Error;
use crate::core::exceptions::AppError;

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid Bitcoin address format: {0}")]
    InvalidAddress(String),

    #[error("Invalid address type provided: {0}")]
    InvalidAddressType(String),

    #[error("Balance cannot be negative: {0}")]
    InvalidBalance(String),

    #[error("Domain validation failed: {0}")]
    Validation(String),

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<AppError> for DomainError {
    fn from(err: AppError) -> Self {
        DomainError::Infrastructure(err.to_string())
    }
}