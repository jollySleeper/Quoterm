/**
 * Print Module
 * All Printing to Terminal in this App is Done by This Module
 */
use std::io;
use std::io::Write;
use termion::{color, style};

fn flush_output() {
    io::stdout().flush().unwrap();
}

pub fn print_colored_message<C: color::Color>(message: &str, color: color::Fg<C>) {
    println!("{}{}{}", color, message, style::Reset);
    flush_output();
}

pub fn print_colored_message_with_padding<C: color::Color>(
    padding: usize,
    message: &str,
    color: color::Fg<C>,
) {
    println!("{:padding$}{}{}{}", "", color, message, style::Reset);
    flush_output();
}

fn set_bold_style() {
    print!("{}", style::Bold);
}

pub fn print_colored_message_in_bold<C: color::Color>(message: &str, color: color::Fg<C>) {
    set_bold_style();
    print_colored_message(message, color);
}

pub fn print_colored_message_with_padding_in_bold<C: color::Color>(
    padding: usize,
    message: &str,
    color: color::Fg<C>,
) {
    set_bold_style();
    print_colored_message_with_padding(padding, message, color);
}
