use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuotermError {
    #[error("Terminal error: {0}")]
    TerminalError(String),
    
    #[error("Failed to read quotes file: {0}")]
    QuotesFileError(String),
    
    #[error("Failed to parse quotes: {0}")]
    QuotesParseError(String),
    
    #[error("Invalid quote index: {0}")]
    QuoteIndexError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error)
}
