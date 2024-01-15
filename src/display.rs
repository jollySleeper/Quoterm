use crate::error::QuotermError;
use crate::print;
use crate::quotes::Quote;
use crate::terminal;
use crate::config;
use termion::color;

pub fn print_divider(terminal_length: usize, config: &config::Config) -> Result<(), QuotermError> {
    let div_line = "─".repeat(terminal_length);
    print::print_colored_message(&div_line, color::Fg(config.get_divider_color()))
}

pub fn print_quote(quote: &Quote, terminal_length: usize, config: &config::Config) -> Result<(), QuotermError> {
    let quote_content = quote.get_content();
    let quote_length = quote_content.len();
    let mut quote_padding = 2;

    if quote_length <= (terminal_length - (quote_padding * 2) - 1) {
        quote_padding = (terminal_length - quote_length - (quote_padding * 2)) / 2;
        print::print_colored_message_with_padding(
            quote_padding,
            quote_content,
            color::Fg(config.get_quote_color()),
        )?;
    } else {
        print_multiline_quote(quote_content, &mut quote_padding, config)?;
    }

    print_author(quote, quote_padding, config)?;
    Ok(())
}

fn print_multiline_quote(content: &str, quote_padding: &mut usize, config: &config::Config) -> Result<(), QuotermError> {
    let lines = terminal::get_lines_of_quote_according_to_terminal_and_padding(
        content.to_string(),
        *quote_padding + 1,
    )?;

    for line in lines {
        print::print_colored_message_with_padding(
            *quote_padding,
            &line,
            color::Fg(config.get_quote_color()),
        )?;
        *quote_padding = if *quote_padding < 3 {
            *quote_padding + 1
        } else {
            *quote_padding
        };
    }
    Ok(())
}

fn print_author(quote: &Quote, quote_padding: usize, config: &config::Config) -> Result<(), QuotermError> {
    let quote_author = quote.get_author();
    let quote_author_string = format!("~ {}", quote_author);
    let quote_author_string_len = quote_author_string.len();

    let author_padding = terminal::get_padding_for_author(quote_author_string_len, quote_padding)?;
    print::print_colored_message_with_padding_in_bold(
        author_padding,
        &quote_author_string,
        color::Fg(config.get_author_color()),
    )
}
