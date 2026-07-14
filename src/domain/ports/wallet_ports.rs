use crate::domain::exceptions::DomainError;
use crate::domain::entities::wallet::Wallet;
use crate::domain::enums::address_types_enums::AddressType;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::balance::Balance;
use async_trait::async_trait;

#[async_trait]
pub trait WalletPort: Send + Sync {
    async fn get_wallet_info(&self) -> Result<Wallet, DomainError>;
    async fn get_balance(&self) -> Result<Balance, DomainError>;
    async fn generate_address(&self, address_type: AddressType) -> Result<Address, DomainError>;
}