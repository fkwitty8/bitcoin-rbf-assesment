

use clap::CommandFactory;
use crate::core::exceptions::AppError;
use crate::presentation::formatters;
use crate::presentation::cli::Cli;

pub async fn handle_author() -> Result<(), AppError> {
    // 1. Retrieve the author string from the clap metadata
    let cmd = Cli::command();
    let author_str = cmd.get_author().unwrap_or("Author not found");

    // 2. Pass it to your presentation formatter
    formatters::print_author(author_str);
    
    Ok(())
}