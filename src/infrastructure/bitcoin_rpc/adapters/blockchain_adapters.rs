use crate::domain::exceptions::DomainError;
use crate::domain::entities::blockchain::ChainState;
use crate::domain::ports::blockchain_ports::BlockchainPort;
use crate::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use crate::infrastructure::bitcoin_rpc::dtos::rpc_response::RawBlockchainInfo;
use async_trait::async_trait;
use std::sync::Arc;

pub struct BitcoinRpcBlockchainAdapter {
    client: Arc<BitcoinRpcClient>,
}

impl BitcoinRpcBlockchainAdapter {
    pub fn new(client: Arc<BitcoinRpcClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl BlockchainPort for BitcoinRpcBlockchainAdapter {
    async fn get_chain_state(&self) -> Result<ChainState, DomainError> {
        
        let raw: RawBlockchainInfo = self.client.call("getblockchaininfo", vec![]).await?;

        Ok(ChainState::new(
            raw.chain,
            raw.blocks,
            raw.headers,
            raw.difficulty,
            raw.verificationprogress,
        ))
    }
}