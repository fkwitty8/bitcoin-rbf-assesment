use crate::domain::exceptions::DomainError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Balance(f64);

impl Balance {
    pub fn new(amount: f64) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::InvalidBalance(format!(
                "Balance amount cannot be negative: {}",
                amount
            )));
        }
        Ok(Self(amount))
    }

    pub fn btc(&self) -> f64 {
        self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_balance() {
        assert!(Balance::new(1.5).is_ok());
    }

    #[test]
    fn test_negative_balance_fails() {
        assert!(Balance::new(-0.5).is_err());
    }
}