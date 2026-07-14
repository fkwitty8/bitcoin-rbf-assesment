use crate::application::dtos::wallet_dto::WalletSummaryDto;
use crate::core::exceptions::AppError;
use crate::domain::ports::wallet_ports::WalletPort;
use std::sync::Arc;

pub struct GetWalletInfoUseCase {
    wallet_port: Arc<dyn WalletPort>,
}

impl GetWalletInfoUseCase {
    pub fn new(wallet_port: Arc<dyn WalletPort>) -> Self {
        Self { wallet_port }
    }

    pub async fn execute(&self) -> Result<WalletSummaryDto, AppError> {
        let wallet = self.wallet_port.get_wallet_info().await?;
        Ok(WalletSummaryDto::from(wallet))
    }
}