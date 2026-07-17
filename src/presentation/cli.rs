use crate::infrastructure::config::Config;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "btc-cli",
    author = "@fkwitty8",
    version = "v.0.0.1 by @fkwitty8",
    about = "Bitcoin Core JSON-RPC CLI "
)]
pub struct Cli {
    /// Print the author information
    #[arg(short = 'a', long)]
    pub author: bool,

    #[command(flatten)]
    pub config: Config,

    #[command(subcommand)]
    pub command:Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fetch blockchain status and metrics
    BlockchainInfo,

    /// Fetch general information for the connected wallet
    WalletInfo,

    /// Query current balance in BTC
    Balance,

    /// Generate a new receiving address
    NewAddress {
        /// Address type: bech32, legacy, p2sh-segwit, or bech32m
        #[arg(short, long, default_value = "bech32")]
        address_type: String,
    },

    /// Execute an arbitrary raw JSON-RPC method against the Bitcoin node
    Rpc {
        /// The JSON-RPC method name (e.g., getblockhash, getmempoolinfo)
        method: String,

        /// Optional JSON arguments passed to the method
        #[arg(num_args = 0..)]
        params: Vec<String>,
    },
}