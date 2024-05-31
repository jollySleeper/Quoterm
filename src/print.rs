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
