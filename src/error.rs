// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::fmt::{Display, Formatter};
// 引入标准库中的 Display 和 Formatter，用于自定义错误输出格式。

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug)]
// 为 ApiError 自动实现 Debug，方便调试打印。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub enum ApiError {
    // 定义统一错误枚举，便于上层统一处理。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Config(String),
    // 配置相关错误（如环境变量缺失）。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Http(String),
    // HTTP 请求或响应相关错误。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Parse(String),
    // JSON 等数据解析相关错误。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl Display for ApiError {
    // 为 ApiError 实现 Display，使错误信息可读。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 定义格式化输出逻辑。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self {
            // 根据不同错误类型输出不同前缀。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            ApiError::Config(msg) => write!(f, "Config error: {msg}"),
            // 配置错误的显示文案。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            ApiError::Http(msg) => write!(f, "HTTP error: {msg}"),
            // HTTP 错误的显示文案。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            ApiError::Parse(msg) => write!(f, "Parse error: {msg}"),
            // 解析错误的显示文案。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl std::error::Error for ApiError {}
// 将 ApiError 标记为标准错误类型，支持 ? 传播与错误链。
