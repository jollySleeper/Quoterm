use std::process;

pub mod print;
pub mod quotes;
pub mod terminal;
pub mod error;
pub mod display;
pub mod config;
pub mod colors;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), error::QuotermError> {
    // Load configuration
    let config = config::Config::load()?;
    
    // Get terminal dimensions
    let terminal_length = terminal::get_terminal_length()?;
    
    // Print divider
    display::print_divider(terminal_length, &config)?;

    // Get and display random quote
    let quotes = quotes::get_quotes_as_objects()?;
    let quote = quotes::get_random_quote(quotes)?;
    display::print_quote(&quote, terminal_length, &config)?;

    Ok(())
}
