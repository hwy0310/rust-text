use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApiError {
    Config(String),
    Http(String),
    Parse(String),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Config(msg) => write!(f, "Config error: {msg}"),
            ApiError::Http(msg) => write!(f, "HTTP error: {msg}"),
            ApiError::Parse(msg) => write!(f, "Parse error: {msg}"),
        }
    }
}

impl std::error::Error for ApiError {}
