// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::fmt::{Display, Formatter};

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub enum BotError {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Capture(String),
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Vision(String),
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Action(String),
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Config(String),
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl Display for BotError {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            BotError::Capture(msg) => write!(f, "Capture error: {msg}"),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            BotError::Vision(msg) => write!(f, "Vision error: {msg}"),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            BotError::Action(msg) => write!(f, "Action error: {msg}"),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            BotError::Config(msg) => write!(f, "Config error: {msg}"),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl std::error::Error for BotError {}
