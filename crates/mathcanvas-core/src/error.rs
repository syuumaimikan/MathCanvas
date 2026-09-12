use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Unknown error occurred")]
    Unknown,
    #[error("Parse error: {0}")]
    ParseError(String),
}
