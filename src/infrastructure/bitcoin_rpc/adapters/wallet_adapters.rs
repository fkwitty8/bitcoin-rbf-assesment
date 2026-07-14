use crate::domain::exceptions::DomainError;
use crate::domain::entities::wallet::Wallet;
use crate::domain::enums::address_types_enums::AddressType;
use crate::domain::ports::wallet_ports::WalletPort;
use crate::domain::value_objects::address::Address;
use crate::domain::value_objects::balance::Balance;
use crate::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use crate::infrastructure::bitcoin_rpc::dtos::rpc_response::RawWalletInfo;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

pub struct BitcoinRpcWalletAdapter {
    client: Arc<BitcoinRpcClient>,
}

impl BitcoinRpcWalletAdapter {
    pub fn new(client: Arc<BitcoinRpcClient>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl WalletPort for BitcoinRpcWalletAdapter {

   async fn get_wallet_info(&self) -> Result<Wallet, DomainError> {
    let raw: RawWalletInfo = self.client.call("getwalletinfo", vec![]).await?;

    // Construct default zero balances for the domain Wallet entity
    let balance = Balance::new(0.0)?;
    let unconfirmed = Balance::new(0.0)?;

    Ok(Wallet::new(
        raw.walletname,
        balance,
        unconfirmed,
        raw.txcount,
    ))
}
    async fn get_balance(&self) -> Result<Balance, DomainError> {
        let raw_balance: f64 = self.client.call("getbalance", vec![]).await?;
        Ok(Balance::new(raw_balance)?)
    }

    async fn generate_address(&self, address_type: AddressType) -> Result<Address, DomainError> {
        let params = vec![
            Value::String("".to_string()),
            Value::String(address_type.as_str().to_string()),
        ];
        let raw_addr: String = self.client.call("getnewaddress", params).await?;
        Ok(Address::new(raw_addr)?)
    }
}