use crate::application::dtos::blockchain_dto::BlockchainSummaryDto;
use crate::application::dtos::wallet_dto::WalletSummaryDto;
use colored::*;
use serde_json::Value;

pub fn print_blockchain_info(dto: &BlockchainSummaryDto) {
    println!("{}", "=== BITCOIN BLOCKCHAIN INFO ===".bold().cyan());
    println!("{:<24} {}", "Chain:".bold(), dto.chain.green());
    println!("{:<24} {}", "Blocks:".bold(), dto.blocks.to_string().yellow());
    println!("{:<24} {}", "Headers:".bold(), dto.headers.to_string().yellow());
    println!("{:<24} {}", "Difficulty:".bold(), dto.difficulty);
    println!(
        "{:<24} {:.2}%",
        "Verification Progress:".bold(),
        dto.verification_progress * 100.0
    );
}

pub fn print_wallet_info(dto: &WalletSummaryDto) {
    println!("{}", "=== BITCOIN WALLET SUMMARY ===".bold().cyan());
    println!("{:<24} {}", "Wallet Name:".bold(), dto.name.green());
    println!("{:<24} {} BTC", "Balance:".bold(), dto.balance_btc.to_string().bold().green());
    println!("{:<24} {} BTC", "Unconfirmed Balance:".bold(), dto.unconfirmed_balance_btc);
    println!("{:<24} {}", "Transaction Count:".bold(), dto.tx_count);
}

pub fn print_balance(balance_btc: f64) {
    println!("{}: {} BTC", "Wallet Balance".bold().cyan(), balance_btc.to_string().bold().green());
}


pub fn print_address(address: &str, address_type: &str) {
    println!(
        "{} [{}]: {}",
        "Generated Address".bold().cyan(),
        address_type.yellow(),
        address.bold().green()
    );
}

pub fn print_raw_rpc(value: &Value) {
    match serde_json::to_string_pretty(value) {
        Ok(pretty) => println!("{}", pretty),
        Err(_) => println!("{:?}", value),
    }
}

pub fn print_author(author: &str) {
    println!("{}: {}", "Author".bold().magenta(), author.bold().white());
}



