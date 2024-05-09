use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod quotes;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quote {
    content: String,
    author: String,
}

pub fn get_quotes_as_objects() -> Vec<Quote> {
    // Reading JSON File
    let quotes_json = &quotes::get_quotes();
    let quotes: Vec<Quote> = serde_json::from_str(quotes_json).unwrap();

    return quotes;
}

pub fn get_random_quote(quotes: Vec<Quote>) -> Quote {
    let entries: usize = quotes.len();
    let mut rng = rand::thread_rng();

    let index: usize = rng.gen_range(0..entries);

    // Use `unwrap` directly since we know the index is valid
    let random_quote: &Quote = quotes.get(index).unwrap();

    // Return a clone of the random quote since we are returning by value
    return random_quote.clone();
}

fn main() {
    // Reading JSON File
    let quotes: Vec<Quote> = get_quotes_as_objects();

    let quote = get_random_quote(quotes);

    println!("{:?}", quote);
}
