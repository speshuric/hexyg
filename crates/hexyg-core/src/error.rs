//! Error types for hexyg-core


/// Result type alias for hexyg operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for hexyg operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Invalid hex character
    #[error("Invalid hex character '{0}' at position {1}")]
    InvalidHexChar(char, usize),

    /// Invalid hex sequence (odd length)
    #[error("Invalid hex sequence: odd number of hex digits")]
    OddHexLength,

    /// UTF-8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}
