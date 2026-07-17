use crate::application::dtos::blockchain_dto::BlockchainSummaryDto;
use crate::application::dtos::wallet_dto::WalletSummaryDto;
use colored::*;
use serde_json::Value;
use tabled::{builder::Builder, settings::Style};


/// Helper to build a standard, clean key-value CLI dashboard table
fn create_dashboard_table() -> Builder {
    let mut builder = Builder::new();
    // Headers are optional, but a clean border layout works best with a baseline structure
    builder.push_record(["Property", "Value"]);
    builder
}

/// Helper to apply style and print our key-value pairs
fn finalize_and_print_table(title: &str, builder: Builder) {
    println!("\n{}", title.bold().cyan());
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{}", table);
}

pub fn print_blockchain_info(dto: &BlockchainSummaryDto) {
    let mut builder = create_dashboard_table();
    
    builder.push_record(["Chain", &dto.chain.green().to_string()]);
    builder.push_record(["Blocks", &dto.blocks.to_string().yellow().to_string()]);
    builder.push_record(["Headers", &dto.headers.to_string().yellow().to_string()]);
    builder.push_record(["Difficulty", &dto.difficulty.to_string()]);
    builder.push_record([
        "Verification Progress",
        &format!("{:.2}%", dto.verification_progress * 100.0)
    ]);

    finalize_and_print_table("=== BITCOIN BLOCKCHAIN INFO ===", builder);
}

pub fn print_wallet_info(dto: &WalletSummaryDto) {
    let mut builder = create_dashboard_table();
    
    builder.push_record(["Wallet Name", &dto.name.green().to_string()]);
    builder.push_record(["Balance", &format!("{} BTC", dto.balance_btc.to_string().bold().green())]);
    builder.push_record(["Unconfirmed Balance", &format!("{} BTC", dto.unconfirmed_balance_btc)]);
    builder.push_record(["Transaction Count", &dto.tx_count.to_string()]);

    finalize_and_print_table("=== BITCOIN WALLET SUMMARY ===", builder);
}

pub fn print_balance(balance_btc: f64) {
    let mut builder = create_dashboard_table();
    builder.push_record(["Wallet Balance", &format!("{} BTC", balance_btc.to_string().bold().green())]);
    
    finalize_and_print_table("=== BALANCE ===", builder);
}

pub fn print_address(address: &str, address_type: &str) {
    let mut builder = create_dashboard_table();
    builder.push_record(["Type", &address_type.yellow().to_string()]);
    builder.push_record(["Generated Address", &address.bold().green().to_string()]);
    
    finalize_and_print_table("=== NEW ADDRESS ===", builder);
}

pub fn print_author(author: &str) {
    let mut builder = Builder::new();
    builder.push_record(["Core Contributor", &author.bold().white().to_string()]);
    
    finalize_and_print_table("=== SYSTEM AUTHOR ===", builder);
}


pub fn print_raw_rpc(value: &Value) {
    let mut builder = Builder::new();

    match value {
        // 1. If it's a standard JSON Object, build our multi-row key-value table
        Value::Object(obj) => {
            builder.push_record(["Property", "Value"]);
            for (key, val) in obj {
                let display_value = match val {
                    Value::Array(arr) => {
                        if arr.is_empty() { "[]".to_string() }
                        else if arr.len() == 1 { arr[0].to_string().replace('"', "") }
                        else { format!("[{} items] ...", arr.len()) }
                    }
                    Value::Object(_) => "[Nested Object]".to_string(),
                    Value::String(s) => s.clone(),
                    _ => val.to_string(),
                };
                builder.push_record([key.bold().to_string(), display_value]);
            }
        }
        // 2. If it's a single primitive value (like getblockcount returning 4)
        _ => {
            builder.push_record(["RPC Output"]);
            // Strip structural string quotes if it's a raw string return
            let clean_val = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            builder.push_record([clean_val]);
        }
    }

    let mut table = builder.build();
    table.with(Style::rounded());

    println!("\n{}", table);
}