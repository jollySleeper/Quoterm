use crate::error::QuotermError;
use termion::terminal_size;

pub fn get_terminal_length() -> Result<usize, QuotermError> {
    terminal_size()
        .map(|(x, _)| usize::from(x))
        .map_err(|e| QuotermError::TerminalError(e.to_string()))
}

pub fn get_padding_for_author(author_length: usize, sentence_padding: usize) -> Result<usize, QuotermError> {
    let small_padding = if sentence_padding > 2 { 10 } else { 5 };
    let terminal_length = get_terminal_length()?;
    Ok(terminal_length - author_length - small_padding - sentence_padding)
}

pub fn get_sentences_according_to_terminal_and_padding(
    quote_length: usize,
    padding: usize,
) -> Result<usize, QuotermError> {
    let terminal_length = get_terminal_length()?;
    let terminal_length_with_padding = terminal_length - (padding * 2) + 1;

    let mut sentences: usize = quote_length / terminal_length_with_padding + 1;
    if quote_length.rem_euclid(terminal_length) > 0 {
        sentences += 1;
    };

    Ok(sentences)
}

pub fn get_lines_of_quote_according_to_terminal_and_padding(
    quote: String,
    padding: usize,
) -> Result<Vec<String>, QuotermError> {
    let mut lines: Vec<String> = Vec::new();
    lines.push("".to_string());

    let mut index = 0;
    let terminal_length = get_terminal_length()?;
    let terminal_length_with_padding = terminal_length - (padding * 2) + 1;

    // Splitting Content According to Terminal Length And Padding
    for word in quote.split_whitespace() {
        if lines[index].len() + word.len() > terminal_length_with_padding {
            lines.push("".to_string());
            index += 1;
        }

        if lines[index] == "" {
            lines[index] = word.to_string();
        } else {
            lines[index] = format!("{} {}", lines[index], word);
        }
    }

    Ok(lines)
}
