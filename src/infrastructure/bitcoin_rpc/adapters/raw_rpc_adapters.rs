use crate::domain::exceptions::DomainError;
use crate::domain::ports::raw_rpc_ports::RawRpcPort;
use crate::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

pub struct BitcoinRpcRawAdapter {
    client: Arc<BitcoinRpcClient>,
}

impl BitcoinRpcRawAdapter {
    pub fn new(client: Arc<BitcoinRpcClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RawRpcPort for BitcoinRpcRawAdapter {
    async fn execute(&self, method: &str, params: Vec<Value>) -> Result<Value, DomainError> {
        self.client.call(method, params).await.map_err(DomainError::from)
    }
}