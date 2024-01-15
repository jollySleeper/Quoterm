use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::error::QuotermError;

pub mod quotes_json;
pub mod models;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    content: String,
    author: String,
}

impl Quote {
    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_author(&self) -> &String {
        &self.author
    }
}

pub fn get_quotes_as_objects() -> Result<Vec<Quote>, QuotermError> {
    let quotes_json = quotes_json::get_all_quotes()?;
    serde_json::from_str(&quotes_json)
        .map_err(|e| QuotermError::JsonError(e))
}

pub fn get_random_quote(quotes: Vec<Quote>) -> Result<Quote, QuotermError> {
    let entries = quotes.len();
    if entries == 0 {
        return Err(QuotermError::QuotesParseError("No quotes available".to_string()));
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..entries);
    
    quotes.get(index)
        .cloned()
        .ok_or_else(|| QuotermError::QuotesParseError(format!("Failed to get quote at index {}", index)))
}
