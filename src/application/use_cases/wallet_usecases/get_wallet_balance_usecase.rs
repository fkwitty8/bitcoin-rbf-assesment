use crate::core::exceptions::AppError;
use crate::domain::ports::wallet_ports::WalletPort;
use std::sync::Arc;

pub struct GetBalanceUseCase {
    wallet_port: Arc<dyn WalletPort>,
}

impl GetBalanceUseCase {
    pub fn new(wallet_port: Arc<dyn WalletPort>) -> Self {
        Self { wallet_port }
    }

    pub async fn execute(&self) -> Result<f64, AppError> {
        let balance = self.wallet_port.get_balance().await?;
        Ok(balance.btc())
    }
}