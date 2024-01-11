use std::fmt;

#[derive(Debug)]
pub enum QuotermError {
    TerminalError(String),
}

impl std::error::Error for QuotermError {}

impl fmt::Display for QuotermError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuotermError::TerminalError(msg) => write!(f, "Terminal error: {}", msg),
        }
    }
}
