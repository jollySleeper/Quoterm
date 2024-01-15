use serde::{Deserialize, Serialize};
use termion::color;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    #[serde(untagged)]
    Rgb(u8, u8, u8),
}

impl Color {
    pub fn to_termion_color(self) -> color::Rgb {
        match self {
            Color::Black => color::Rgb(0x00, 0x00, 0x00),
            Color::Red => color::Rgb(0xCD, 0x31, 0x31),      // #CD3131
            Color::Green => color::Rgb(0x0D, 0xBC, 0x79),    // #0DBC79
            Color::Yellow => color::Rgb(0xE5, 0xE5, 0x10),   // #E5E510
            Color::Blue => color::Rgb(0x2A, 0x84, 0xD2),     // #2A84D2
            Color::Magenta => color::Rgb(0xBC, 0x3F, 0xBC),  // #BC3FBC
            Color::Cyan => color::Rgb(0x11, 0xA8, 0xCD),     // #11A8CD
            Color::White => color::Rgb(0xE5, 0xE5, 0xE5),    // #E5E5E5
            Color::Rgb(r, g, b) => color::Rgb(r, g, b),
        }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            _ => Color::White, // default to white for unknown colors
        }
    }
}
