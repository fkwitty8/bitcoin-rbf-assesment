use crate::infrastructure::config::Config;
use clap::{Parser, Subcommand, ColorChoice, builder::styling};

// 1. Define your custom premium color palette here
pub fn get_styles() -> styling::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default().bold())
        .usage(styling::AnsiColor::Green.on_default().bold())
        .literal(styling::AnsiColor::Cyan.on_default().bold())
        .placeholder(styling::AnsiColor::BrightBlack.on_default())
}

#[derive(Parser, Debug)]
#[command(
    name = "btc-cli",
    author,
    version,
    about = "Bitcoin Core JSON-RPC CLI ",
    styles = get_styles(),
    color = ColorChoice::Always,
    help_template = "\n{before-help}{about} {version}\n\n{usage-heading}\n  {usage}\n\n{all-args}{after-help}"
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
    Author,
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