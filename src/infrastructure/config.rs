use clap::Args;


#[derive(Args, Debug, Clone)]
pub struct Config {
    #[arg(
        long,
        env = "BTC_RPC_URL",
        default_value = "http://127.0.0.1:18443"
    )]
    pub rpc_url: String,

    #[arg(
        long, 
        env = "BTC_RPC_USER", 
        default_value = "polaruser",
        hide_env_values = true,     // Protects active terminal env value
        hide_default_value = true   // Masks fallback default from screen
    )]
    pub rpc_user: String,

    #[arg(
        long, 
        env = "BTC_RPC_PASS", 
        default_value = "polarpass",
        hide_env_values = true,     // Protects active terminal env value
        hide_default_value = true   // Masks fallback default from screen
    )]
    pub rpc_pass: String,

    #[arg(long, env = "BTC_WALLET_NAME")]
    pub wallet: Option<String>,
}

impl Config {
    /// Constructs the target RPC endpoint URL.
    /// If a wallet is specified, appends `/wallet/<name>` to route requests to that loaded wallet.
    pub fn get_endpoint_url(&self) -> String {
        let base = self.rpc_url.trim_end_matches('/');
        match &self.wallet {
            Some(w) if !w.trim().is_empty() => format!("{}/wallet/{}", base, w.trim()),
            _ => base.to_string(),
        }
    }
}