use crate::core::exceptions::AppError;
use crate::domain::enums::address_types_enums::AddressType;
use crate::domain::ports::wallet_ports::WalletPort;
use std::sync::Arc;

pub struct GenerateAddressUseCase {
    wallet_port: Arc<dyn WalletPort>,
}

impl GenerateAddressUseCase {
    pub fn new(wallet_port: Arc<dyn WalletPort>) -> Self {
        Self { wallet_port }
    }

    pub async fn execute(&self, address_type: AddressType) -> Result<String, AppError> {
        let address = self.wallet_port.generate_address(address_type).await?;
        Ok(address.value().to_string())
    }
}