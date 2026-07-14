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
//     pub WalletPort {}

//     #[async_trait]
//     impl WalletPort for WalletPort {
//         async fn get_wallet_info(&self) -> Result<Wallet, DomainError>;
//         async fn get_balance(&self) -> Result<Balance, DomainError>;
//         async fn generate_address(&self, address_type: AddressType) -> Result<Address, DomainError>;
//     }
// }

// // ==========================================
// // GenerateAddressUseCase Tests
// // ==========================================

// #[tokio::test]
// async fn test_generate_address_success() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_generate_address()
//         .withf(|addr_type| matches!(addr_type, AddressType::Bech32))
//         .times(1)
//         .returning(|_| {
//             Ok(Address::new(
//                 "bcrt1qtestaddress123".to_string(),
//                 AddressType::Bech32,
//             ))
//         });

//     let use_case = GenerateAddressUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute(AddressType::Bech32).await;

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), "bcrt1qtestaddress123");
// }

// #[tokio::test]
// async fn test_generate_address_failure() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_generate_address()
//         .times(1)
//         .returning(|_| Err(DomainError::Infrastructure("Wallet locked".to_string())));

//     let use_case = GenerateAddressUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute(AddressType::Legacy).await;

//     assert!(result.is_err());
// }

// // ==========================================
// // GetBalanceUseCase Tests
// // ==========================================

// #[tokio::test]
// async fn test_get_balance_success() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_get_balance()
//         .times(1)
//         .returning(|| Ok(Balance::new(2.5).unwrap()));

//     let use_case = GetBalanceUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_ok());
//     assert_eq!(result.unwrap(), 2.5);
// }

// #[tokio::test]
// async fn test_get_balance_failure() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_get_get_balance() // or expect_get_balance depending on mockall method generation
//         .times(1)
//         .returning(|| Err(DomainError::Infrastructure("Node unreachable".to_string())));

//     let use_case = GetBalanceUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_err());
// }

// // ==========================================
// // GetWalletInfoUseCase Tests
// // ==========================================

// #[tokio::test]
// async fn test_get_wallet_info_success() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_get_wallet_info()
//         .times(1)
//         .returning(|| {
//             let balance = Balance::new(10.0).unwrap();
//             let unconfirmed = Balance::new(0.0).unwrap();
//             Ok(Wallet::new(
//                 "default".to_string(),
//                 balance,
//                 unconfirmed,
//                 5,
//             ))
//         });

//     let use_case = GetWalletInfoUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_ok());
//     let dto = result.unwrap();
//     assert_eq!(dto.name, "default");
//     assert_eq!(dto.balance, 10.0);
//     assert_eq!(dto.tx_count, 5);
// }

// #[tokio::test]
// async fn test_get_wallet_info_failure() {
//     let mut mock_port = MockWalletPort::new();

//     mock_port
//         .expect_get_wallet_info()
//         .times(1)
//         .returning(|| Err(DomainError::Infrastructure("Wallet not found".to_string())));

//     let use_case = GetWalletInfoUseCase::new(Arc::new(mock_port));
//     let result = use_case.execute().await;

//     assert!(result.is_err());
// }