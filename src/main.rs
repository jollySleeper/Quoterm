use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use termion::{color, style, terminal_size};

pub mod print;
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

const TERMINAL_LENGHT: usize = get_terminal_length();

pub fn get_padding_for_author(author_length: usize, sentence_padding: usize) -> usize {
    let small_padding = if padding > 0 { 10 } else { 5 };

    return get_terminal_length() - author_length - small_padding - sentence_padding;
}

pub fn get_sentences_according_to_terminal(quote_length: usize) -> usize {
    let mut sentences: usize = quote_length / TERMINAL_LENGHT;
    if quote_length.rem_euclid(TERMINAL_LENGHT) > 0 {
        sentences += 1;
    };

    return sentences;
}

fn main() {
    let div_line = "─".repeat(TERMINAL_LENGTH);
    &print::print_colored_message(&div_line, color::Fg(color::Yellow));

    // Reading JSON File
    let quotes: Vec<Quote> = get_quotes_as_objects();
    let quote = get_random_quote(quotes);

    let quote_content = quote.content;
    let quote_author = quote.author;
    let quote_length = quote_content.len();

    let mut quote_padding = 0;
    if quote_length <= TERMINAL_LENGTH {
        quote_padding = (TERMINAL_LENGTH - quote_length - 4) / 2;
        &print::print_colored_message_with_padding(
            quote_padding,
            &quote_content,
            color::Fg(color::Blue),
        );
    } else {
        let sentences = get_sentences_according_to_terminal(quote_length);
        let mut lines: Vec<String> = vec![String::from(""); usize::from(sentences)];
        let mut index = 0;
        for word in quote_content.split_whitespace() {
            if lines[index].len() + word.len() > TERMINAL_LENGTH - 3 {
                index += 1;
            }
            lines[index] = format!("{} {}", lines[index], word);
        }

        for line in lines {
            &print::print_colored_message(&line, color::Fg(color::Blue));
        }
    }

    let quote_author_str_len = quote_author.len();
    let quote_author_string = format!("~ {}", &quote_author);

    &print::print_colored_message_with_padding_in_bold(
        get_padding_for_author(quote_author_str_len, quote_padding),
        &quote_author_string,
        color::Fg(color::Red),
    );
}
