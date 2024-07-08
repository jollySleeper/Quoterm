use crate::quotes::Quote;
use termion::color;

pub mod print;
pub mod quotes;
pub mod terminal;

fn main() {
    let terminal_length = terminal::get_terminal_length();
    let div_line = "─".repeat(terminal_length);
    let _ = &print::print_colored_message(&div_line, color::Fg(color::Yellow));

    let quotes: Vec<Quote> = quotes::get_quotes_as_objects();
    let quote = &quotes::get_random_quote(quotes);

    let quote_content = quote.get_content();
    let quote_length = quote_content.len();

    let mut quote_padding = 2;
    if quote_length <= (terminal_length - (quote_padding * 2) - 1) {
        quote_padding = (terminal_length - quote_length - (quote_padding * 2)) / 2;
        let _ = &print::print_colored_message_with_padding(
            quote_padding,
            &quote_content,
            color::Fg(color::Blue),
        );
    } else {
        let lines: Vec<String> =
            terminal::get_lines_of_quote_according_to_terminal(quote_content.to_string());
        for line in lines {
            let _ = &print::print_colored_message_with_padding(
                quote_padding,
                &line,
                color::Fg(color::Blue),
            );
            quote_padding = if quote_padding < 3 {
                quote_padding + 1
            } else {
                quote_padding
            };
        }
    }

    let quote_author = quote.get_author();
    let quote_author_string = format!("~ {}", &quote_author);
    let quote_author_string_len = quote_author_string.len();

    let _ = &print::print_colored_message_with_padding_in_bold(
        terminal::get_padding_for_author(quote_author_string_len, quote_padding),
        &quote_author_string,
        color::Fg(color::Red),
    );
}
