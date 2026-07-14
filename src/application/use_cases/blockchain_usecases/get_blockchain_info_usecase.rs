use crate::application::dtos::blockchain_dto::BlockchainSummaryDto;
use crate::core::exceptions::AppError;
use crate::domain::ports::blockchain_ports::BlockchainPort;
use std::sync::Arc;

pub struct GetBlockchainInfoUseCase {
    blockchain_port: Arc<dyn BlockchainPort>,
}

impl GetBlockchainInfoUseCase {
    pub fn new(blockchain_port: Arc<dyn BlockchainPort>) -> Self {
        Self { blockchain_port }
    }

    pub async fn execute(&self) -> Result<BlockchainSummaryDto, AppError> {
        let chain_state = self.blockchain_port.get_chain_state().await?;
        Ok(BlockchainSummaryDto::from(chain_state))
    }
}