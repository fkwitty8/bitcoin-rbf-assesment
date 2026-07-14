use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<RpcErrorData>,
}

#[derive(Deserialize, Debug)]
pub struct RpcErrorData {
    pub code: i64,
    pub message: String,
}

// Bitcoin Core `getblockchaininfo` raw JSON response
#[derive(Deserialize, Debug)]
pub struct RawBlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

// Bitcoin Core `getwalletinfo` raw JSON response
#[derive(Deserialize, Debug)]
pub struct RawWalletInfo {
    pub walletname: String,
    pub txcount: u64,
}