use crate::domain::exceptions::DomainError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    Legacy,
    P2shSegwit,
    Bech32,
    Bech32m,
}

impl AddressType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AddressType::Legacy => "legacy",
            AddressType::P2shSegwit => "p2sh-segwit",
            AddressType::Bech32 => "bech32",
            AddressType::Bech32m => "bech32m",
        }
    }
}

impl FromStr for AddressType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "legacy" => Ok(AddressType::Legacy),
            "p2sh-segwit" => Ok(AddressType::P2shSegwit),
            "bech32" => Ok(AddressType::Bech32),
            "bech32m" => Ok(AddressType::Bech32m),
            _ => Err(DomainError::InvalidAddressType(s.to_string())),
        }
    }
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}