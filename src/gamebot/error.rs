use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BotError {
    Capture(String),
    Vision(String),
    Action(String),
    Config(String),
}

impl Display for BotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotError::Capture(msg) => write!(f, "Capture error: {msg}"),
            BotError::Vision(msg) => write!(f, "Vision error: {msg}"),
            BotError::Action(msg) => write!(f, "Action error: {msg}"),
            BotError::Config(msg) => write!(f, "Config error: {msg}"),
        }
    }
}

impl std::error::Error for BotError {}
