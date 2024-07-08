use termion::terminal_size;

pub fn get_terminal_length() -> usize {
    let (length, _) = {
        let (x, y) = terminal_size().unwrap();
        (usize::from(x), usize::from(y))
    };

    return length;
}

pub fn get_padding_for_author(author_length: usize, sentence_padding: usize) -> usize {
    let small_padding = if sentence_padding > 2 { 10 } else { 5 };

    return get_terminal_length() - author_length - small_padding - sentence_padding;
}

pub fn get_sentences_according_to_terminal_and_padding(
    quote_length: usize,
    padding: usize,
) -> usize {
    let terminal_length = get_terminal_length();
    let terminal_length_with_padding = terminal_length - (padding * 2) + 1;

    let mut sentences: usize = quote_length / terminal_length_with_padding + 1;
    println!("rem={}", quote_length.rem_euclid(terminal_length));
    if quote_length.rem_euclid(terminal_length) > 0 {
        sentences += 1;
    };

    return sentences;
}

pub fn get_lines_of_quote_according_to_terminal_and_padding(
    quote: String,
    padding: usize,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push("".to_string());

    let mut index = 0;
    let terminal_length = get_terminal_length();
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

    return lines;
}
