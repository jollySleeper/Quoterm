use rand::Rng;
use crate::error::QuotermError;

pub mod quotes_json;
pub mod models;

pub use models::Quote;

pub fn get_quotes_as_objects() -> Result<Vec<Quote>, QuotermError> {
    // Reading JSON File
    let quotes_json = &quotes_json::get_all_quotes();
    let quotes: Vec<Quote> = serde_json::from_str(quotes_json)
        .map_err(|e| QuotermError::QuotesParseError(e.to_string()))?;

    Ok(quotes)
}

pub fn get_random_quote(quotes: Vec<Quote>) -> Result<Quote, QuotermError> {
    let entries: usize = quotes.len();
    if entries == 0 {
        return Err(QuotermError::QuoteIndexError("No quotes available".to_string()));
    }

    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..entries);
    
    quotes.get(index)
        .cloned()
        .ok_or_else(|| QuotermError::QuoteIndexError(format!("Invalid quote index: {}", index)))
}
