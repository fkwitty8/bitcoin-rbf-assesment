use crate::domain::exceptions::DomainError;
use crate::domain::entities::blockchain::ChainState;
use async_trait::async_trait;

#[async_trait]
pub trait BlockchainPort: Send + Sync {
    async fn get_chain_state(&self) -> Result<ChainState, DomainError>;
}