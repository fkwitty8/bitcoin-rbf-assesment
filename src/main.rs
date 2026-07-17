use rfb_assesment::core::container::AppContainer;
use rfb_assesment::core::exceptions::AppError;
use rfb_assesment::core::logger;
use rfb_assesment::presentation::cli::{Cli, Commands};
use rfb_assesment::presentation::handlers::{blockchain_handler, raw_rpc_handler, wallet_handler,util_handler};
use clap::Parser;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Load environment variables from .env if present
    let _ = dotenv();

    // Initialize application logger
    logger::init_logger();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize Dependency Injection Container
    let container = AppContainer::new(&cli.config);

    // Dispatch subcommands
    match &cli.command {
        Commands::BlockchainInfo => {
            blockchain_handler::handle_blockchain_info(&container).await?;
        }
        Commands::WalletInfo => {
            wallet_handler::handle_wallet_info(&container).await?;
        }
        Commands::Balance => {
            wallet_handler::handle_balance(&container).await?;
        }
        Commands::NewAddress { address_type } => {
            wallet_handler::handle_new_address(&container, address_type).await?;
        }
        Commands::Rpc { method, params } => {
            raw_rpc_handler::handle_raw_rpc(&container, method, params).await?;
        }
        Commands::Author => {
            util_handler::handle_author().await?;
        }
    }

    Ok(())
}