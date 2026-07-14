use rfb_assesment::presentation::cli::{Cli, Commands};
use clap::Parser;

#[test]
fn test_cli_default_new_address_type_is_bech32() {
    let args = vec!["btc-cli", "new-address"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse CLI args");

    match cli.command {
        Commands::NewAddress { address_type } => {
            assert_eq!(address_type, "bech32");
        }
        _ => panic!("Expected NewAddress command"),
    }
}

#[test]
fn test_cli_custom_address_type() {
    let args = vec!["btc-cli", "new-address", "--address-type", "legacy"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse CLI args");

    match cli.command {
        Commands::NewAddress { address_type } => {
            assert_eq!(address_type, "legacy");
        }
        _ => panic!("Expected NewAddress command"),
    }
}