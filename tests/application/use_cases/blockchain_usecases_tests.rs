// use std::sync::Arc;
// use serde_json::json;

// use rfb_assesment::application::use_cases::blockchain_usecases::get_blockchain_info_usecase::GetBlockchainInfoUseCase;
// use rfb_assesment::application::use_cases::execute_raw_rpc_usecase::ExecuteRawRpcUseCase;
// use rfb_assesment::application::use_cases::wallet_usecases::generate_wallet_address_usecase::GenerateAddressUseCase;
// use rfb_assesment::application::use_cases::wallet_usecases::get_wallet_balance_usecase::GetBalanceUseCase;
// use rfb_assesment::application::use_cases::wallet_usecases::get_wallet_info_usecase::GetWalletInfoUseCase;

// use rfb_assesment::core::exceptions::AppError;
// use rfb_assesment::domain::enums::address_types_enums::AddressType;
// use rfb_assesment::domain::entities::blockchain::ChainState;

// use rfb_assesment::domain::entities::wallet::Wallet;
 
// use rfb_assesment::domain::value_objects::address::Address;

// use rfb_assesment::domain::value_objects::balance::Balance;
// use rfb_assesment::domain::value_objects::difficulty::Difficulty;

// use rfb_assesment::domain::exceptions::DomainError;



// use rfb_assesment::domain::ports::blockchain_ports::MockBlockchainPort;
// use rfb_assesment::domain::ports::raw_rpc_ports::MockRawRpcPort;
// use rfb_assesment::domain::ports::wallet_ports::MockWalletPort;





// use mockall::mock;



// mock! {
//     pub BlockchainPort {}

//     #[async_trait]
//     impl BlockchainPort for BlockchainPort {
//         async fn get_chain_state(&self) -> Result<ChainState, DomainError>;
//     }
// }

// #[tokio::test]
// async fn test_get_blockchain_info_success() {
//     let mut mock_port = MockBlockchainPort::new();

//     mock_port
//         .expect_get_chain_state()
//         .times(1)
//         .returning(|| {
//             Ok(ChainState::new(
//                 "regtest".to_string(),
//                 100,
//                 100,
//                 "0000000000".to_string(),
//                 0.0,
//             ))
//         });

//     let use_case = GetBlockchainInfoUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_ok());
//     let dto = result.unwrap();
//     assert_eq!(dto.chain, "regtest");
//     assert_eq!(dto.blocks, 100);
// }

// #[tokio::test]
// async fn test_get_blockchain_info_failure() {
//     let mut mock_port = MockBlockchainPort::new();

//     mock_port
//         .expect_get_chain_state()
//         .times(1)
//         .returning(|| Err(DomainError::Infrastructure("Connection refused".to_string())));

//     let use_case = GetBlockchainInfoUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_err());
// }