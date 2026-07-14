use crate::domain::exceptions::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address(String);

impl Address {
    pub fn new(value: String) -> Result<Self, DomainError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidAddress(
                "Address string cannot be empty".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}