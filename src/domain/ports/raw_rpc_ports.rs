use crate::domain::exceptions::DomainError;
use async_trait::async_trait;
use serde_json::Value;



#[async_trait]
pub trait RawRpcPort: Send + Sync {
    async fn execute(&self, method: &str, params: Vec<Value>) -> Result<Value, DomainError>;
}