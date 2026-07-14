use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Debug)]
pub struct RpcRequest<'a> {
    pub jsonrpc: &'static str,
    pub id: &'static str,
    pub method: &'a str,
    pub params: Vec<Value>,
}

impl<'a> RpcRequest<'a> {
    pub fn new(method: &'a str, params: Vec<Value>) -> Self {
        Self {
            jsonrpc: "1.0",
            id: "btc-cli",
            method,
            params,
        }
    }
}