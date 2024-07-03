use crate::quotes::Quote;
use std::sync::LazyLock;
use termion::{color, terminal_size};

pub mod print;
pub mod quotes;

pub fn get_terminal_length() -> usize {
    let (length, _) = {
        let (x, y) = terminal_size().unwrap();
        (usize::from(x), usize::from(y))
    };

    return length;
}

static TERMINAL_LENGTH: LazyLock<usize> = LazyLock::new(|| {
    let len = get_terminal_length();
    len
});

pub fn get_padding_for_author(author_length: usize, sentence_padding: usize) -> usize {
    let small_padding = if sentence_padding > 0 { 10 } else { 5 };

    return get_terminal_length() - author_length - small_padding - sentence_padding;
}

pub fn get_sentences_according_to_terminal(quote_length: usize) -> usize {
    let mut sentences: usize = quote_length / *TERMINAL_LENGTH;
    if quote_length.rem_euclid(*TERMINAL_LENGTH) > 0 {
        sentences += 1;
    };

    return sentences;
}

pub fn get_lines_of_quote(quote: String) -> Vec<String> {
    let sentences_number = get_sentences_according_to_terminal(quote.len());
    let mut lines: Vec<String> = vec![String::from(""); sentences_number];

    let mut index = 0;
    // Splitting Content According to Terminal Lenght
    for word in quote.split_whitespace() {
        if lines[index].len() + word.len() > *TERMINAL_LENGTH - 3 {
            index += 1;
        }
        lines[index] = format!("{} {}", lines[index], word);
    }

    return lines;
}

fn main() {
    let div_line = "─".repeat(*TERMINAL_LENGTH);
    let _ = &print::print_colored_message(&div_line, color::Fg(color::Yellow));

    let quotes: Vec<Quote> = quotes::get_quotes_as_objects();
    let quote = &quotes::get_random_quote(quotes);

    let quote_content = quote.get_content();
    let quote_length = quote_content.len();

    let mut quote_padding = 0;
    if quote_length <= *TERMINAL_LENGTH {
        quote_padding = (*TERMINAL_LENGTH - quote_length - 4) / 2;
        let _ = &print::print_colored_message_with_padding(
            quote_padding,
            &quote_content,
            color::Fg(color::Blue),
        );
    } else {
        let lines: Vec<String> = get_lines_of_quote(quote_content.to_string());
        for line in lines {
            let _ = &print::print_colored_message(&line, color::Fg(color::Blue));
        }
    }

    let quote_author = quote.get_author();
    let quote_author_str_len = quote_author.len();
    let quote_author_string = format!("~ {}", &quote_author);

    let _ = &print::print_colored_message_with_padding_in_bold(
        get_padding_for_author(quote_author_str_len, quote_padding),
        &quote_author_string,
        color::Fg(color::Red),
    );
}
