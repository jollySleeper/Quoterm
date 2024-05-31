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

pub fn get_terminal_length() -> usize {
    let (length, _) = {
        let (x, y) = terminal_size().unwrap();
        (usize::from(x), usize::from(y))
    };

    return length;
}

pub fn print_colored_message<C: color::Color>(message: &str, color: color::Fg<C>) {
    print!("{}{}{}", color, message, style::Reset);
    io::stdout().flush().unwrap();
}

pub fn print_colored_message_in_bold<C: color::Color>(message: &str, color: color::Fg<C>) {
    print!("{}", style::Bold);
    print_colored_message(message, color);
}

pub fn print_colored_message_with_padding<C: color::Color>(
    padding: usize,
    message: &str,
    color: color::Fg<C>,
) {
    println!("{:padding$}{}{}{}", "", color, message, style::Reset);
    io::stdout().flush().unwrap();
}

pub fn print_colored_message_with_padding_in_bold<C: color::Color>(
    padding: usize,
    message: &str,
    color: color::Fg<C>,
) {
    print!("{}", style::Bold);
    print_colored_message_with_padding(padding, message, color);
}

fn main() {
    // Reading JSON File
    let quotes: Vec<Quote> = get_quotes_as_objects();

    let quote = get_random_quote(quotes);

    let length = get_terminal_length();
    let line = "─".repeat(length);
    print_colored_message(&line, color::Fg(color::Yellow));

    let mut padding = 0;
    let quote_content = quote.content;
    let quote_author = quote.author;
    let quote_length = quote_content.len();

    if quote_content.len() > length {
        let mut sentences: usize = quote_length / length;
        if quote_length.rem_euclid(length) > 0 {
            sentences += 1;
        };

        let mut lines: Vec<String> = vec![String::from(""); usize::from(sentences)];
        let mut index = 0;
        for word in quote_content.split_whitespace() {
            if lines[index].len() + word.len() > length - 3 {
                index += 1;
            }
            lines[index] = format!("{} {}", lines[index], word);
        }

        for line in lines {
            println!("{}{}", color::Fg(color::Blue), line);
        }
    } else {
        padding = (length - quote_length - 4) / 2;
        println!("{:padding$}{}{}", "", color::Fg(color::Blue), quote_content);
    }

    padding = if padding > 0 {
        length - quote_author.len() - 10 - padding
    } else {
        length - quote_author.len() - 5
    };

    let quote_author_string = format!("~ {}", &quote_author);
    print_colored_message_with_padding_in_bold(
        padding,
        &quote_author_string,
        color::Fg(color::Red),
    );
}
