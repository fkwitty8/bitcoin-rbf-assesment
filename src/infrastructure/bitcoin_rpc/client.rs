use crate::core::exceptions::AppError;
use crate::infrastructure::bitcoin_rpc::dtos::rpc_request::RpcRequest;
use crate::infrastructure::bitcoin_rpc::dtos::rpc_response::RpcResponse;
use crate::infrastructure::config::Config;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct BitcoinRpcClient {
    client: Client,
    endpoint: String,
    user: String,
    pass: String,
}

impl BitcoinRpcClient {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            endpoint: config.get_endpoint_url(),
            user: config.rpc_user.clone(),
            pass: config.rpc_pass.clone(),
        }
    }

    pub async fn call<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T, AppError> {
        let payload = RpcRequest::new(method, params);

        let response = self
            .client
            .post(&self.endpoint)
            .basic_auth(&self.user, Some(&self.pass))
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::ConnectionFailed(e.to_string()))?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(AppError::AuthFailed);
        }

        let rpc_res: RpcResponse<T> = response
            .json()
            .await
            .map_err(|e| AppError::Execution(format!("Failed to parse JSON response: {}", e)))?;

        if let Some(err) = rpc_res.error {
            return Err(AppError::RpcError {
                code: err.code,
                message: err.message,
            });
        }

        rpc_res
            .result
            .ok_or_else(|| AppError::Execution("RPC response returned null result".to_string()))
    }
}