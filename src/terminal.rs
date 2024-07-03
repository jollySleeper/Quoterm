use termion::terminal_size;

pub fn get_terminal_length() -> usize {
    let (length, _) = {
        let (x, y) = terminal_size().unwrap();
        (usize::from(x), usize::from(y))
    };

    return length;
}

pub fn get_padding_for_author(author_length: usize, sentence_padding: usize) -> usize {
    let small_padding = if sentence_padding > 0 { 10 } else { 5 };

    return get_terminal_length() - author_length - small_padding - sentence_padding;
}

pub fn get_sentences_according_to_terminal(quote_length: usize) -> usize {
    let terminal_length = get_terminal_length();
    let mut sentences: usize = quote_length / terminal_length;
    if quote_length.rem_euclid(terminal_length) > 0 {
        sentences += 1;
    };

    return sentences;
}

pub fn get_lines_of_quote_according_to_terminal(quote: String) -> Vec<String> {
    let sentences_number = get_sentences_according_to_terminal(quote.len());
    let mut lines: Vec<String> = vec![String::from(""); sentences_number];

    let mut index = 0;
    let terminal_length = get_terminal_length();
    // Splitting Content According to Terminal Lenght
    for word in quote.split_whitespace() {
        if lines[index].len() + word.len() > terminal_length - 3 {
            index += 1;
        }
        lines[index] = format!("{} {}", lines[index], word);
    }

    return lines;
}
