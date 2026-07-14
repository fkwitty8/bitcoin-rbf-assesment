use crate::domain::exceptions::DomainError;
use crate::domain::ports::raw_rpc_ports::RawRpcPort;
use serde_json::Value;
use std::sync::Arc;

pub struct ExecuteRawRpcUseCase {
    raw_rpc_port: Arc<dyn RawRpcPort>,
}

impl ExecuteRawRpcUseCase {
    pub fn new(raw_rpc_port: Arc<dyn RawRpcPort>) -> Self {
        Self { raw_rpc_port }
    }

    pub async fn execute(&self, method: &str, params: Vec<Value>) -> Result<Value, DomainError> {
        self.raw_rpc_port.execute(method, params).await
    }
}