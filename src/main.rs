use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod quotes;

#[derive(Deserialize, Serialize, Debug)]
struct Quote {
    content: String,
    author: String,
}

fn main() {
    // Reading JSON File
    let json_file: Vec<Quote> = serde_json::from_str(&quotes::get_quotes()).unwrap();

    let entries: usize = json_file.len();
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..entries);

    let random_quote: &Quote = match json_file.get(index) {
        Some(_x) => json_file.get(index).unwrap(),
        None => panic!("woops"),
    };

    println!("{:?}", random_quote);
}
