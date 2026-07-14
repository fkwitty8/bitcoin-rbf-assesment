use crate::application::use_cases::execute_raw_rpc_usecase::ExecuteRawRpcUseCase;
use crate::application::use_cases::wallet_usecases::generate_wallet_address_usecase::GenerateAddressUseCase;
use crate::application::use_cases::wallet_usecases::get_wallet_balance_usecase::GetBalanceUseCase;
use crate::application::use_cases::blockchain_usecases::get_blockchain_info_usecase::GetBlockchainInfoUseCase;
use crate::application::use_cases::wallet_usecases::get_wallet_info_usecase::GetWalletInfoUseCase;
use crate::infrastructure::bitcoin_rpc::adapters::blockchain_adapters::BitcoinRpcBlockchainAdapter;
use crate::infrastructure::bitcoin_rpc::adapters::raw_rpc_adapters::BitcoinRpcRawAdapter;
use crate::infrastructure::bitcoin_rpc::adapters::wallet_adapters::BitcoinRpcWalletAdapter;
use crate::infrastructure::bitcoin_rpc::client::BitcoinRpcClient;
use crate::infrastructure::config::Config;
use std::sync::Arc;

pub struct AppContainer {
    pub get_blockchain_info_uc: GetBlockchainInfoUseCase,
    pub get_wallet_info_uc: GetWalletInfoUseCase,
    pub get_balance_uc: GetBalanceUseCase,
    pub generate_address_uc: GenerateAddressUseCase,
    pub execute_raw_rpc_uc: ExecuteRawRpcUseCase,
}

impl AppContainer {
    pub fn new(config: &Config) -> Self {
        // Shared Infrastructure Driver
        let rpc_client = Arc::new(BitcoinRpcClient::new(config));

        // Adapters implementing Domain Ports
        let blockchain_adapter = Arc::new(BitcoinRpcBlockchainAdapter::new(rpc_client.clone()));
        let wallet_adapter = Arc::new(BitcoinRpcWalletAdapter::new(rpc_client.clone()));
        let raw_rpc_adapter = Arc::new(BitcoinRpcRawAdapter::new(rpc_client.clone()));

        // Instantiate Use Cases with Injected Ports
        Self {
            get_blockchain_info_uc: GetBlockchainInfoUseCase::new(blockchain_adapter),
            get_wallet_info_uc: GetWalletInfoUseCase::new(wallet_adapter.clone()),
            get_balance_uc: GetBalanceUseCase::new(wallet_adapter.clone()),
            generate_address_uc: GenerateAddressUseCase::new(wallet_adapter),
            execute_raw_rpc_uc: ExecuteRawRpcUseCase::new(raw_rpc_adapter),
        }
    }
}