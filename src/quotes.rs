use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod quotes_json;

#[derive(Deserialize, Serialize, Debug, Clone)]
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

pub fn get_quotes_as_objects() -> Vec<Quote> {
    // Reading JSON File
    let quotes_json = &quotes_json::get_all_quotes();
    let quotes: Vec<Quote> = serde_json::from_str(quotes_json).unwrap();

    return quotes;
}

pub fn get_random_quote(quotes: Vec<Quote>) -> Quote {
    let entries: usize = quotes.len();
    let mut rng = rand::thread_rng();

    let index: usize = rng.gen_range(0..entries);
    let random_quote: &Quote = quotes.get(index).unwrap();

    return random_quote.clone();
}
