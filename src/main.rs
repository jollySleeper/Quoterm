use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use termion::{color, style, terminal_size};

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

    let random_quote: &Quote = quotes.get(index).unwrap();

    return random_quote.clone();
}

pub fn print_colored_line<C: color::Color>(length: usize, color: color::Fg<C>) {
    println!("{}{}{:^length$}{}", style::Bold, color, "", style::Reset);
    io::stdout().flush().unwrap();
}

fn main() {
    // Reading JSON File
    let quotes: Vec<Quote> = get_quotes_as_objects();

    let quote = get_random_quote(quotes);

    let (length, _height) = {
        let (x, y) = terminal_size().unwrap();
        (usize::from(x), usize::from(y))
    };

    print_colored_line(length, color::Fg(color::Yellow));
    println!(
        "{}{}{:^length$}{}",
        style::Bold,
        color::Fg(color::Yellow),
        "",
        style::Reset
    );
}
