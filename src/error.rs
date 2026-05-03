use std::fmt::{Display, Formatter};
// 引入标准库中的 Display 和 Formatter，用于自定义错误输出格式。

#[derive(Debug)]
// 为 ApiError 自动实现 Debug，方便调试打印。
pub enum ApiError {
    // 定义统一错误枚举，便于上层统一处理。
    Config(String),
    // 配置相关错误（如环境变量缺失）。
    Http(String),
    // HTTP 请求或响应相关错误。
    Parse(String),
    // JSON 等数据解析相关错误。
}

impl Display for ApiError {
    // 为 ApiError 实现 Display，使错误信息可读。
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 定义格式化输出逻辑。
        match self {
            // 根据不同错误类型输出不同前缀。
            ApiError::Config(msg) => write!(f, "Config error: {msg}"),
            // 配置错误的显示文案。
            ApiError::Http(msg) => write!(f, "HTTP error: {msg}"),
            // HTTP 错误的显示文案。
            ApiError::Parse(msg) => write!(f, "Parse error: {msg}"),
            // 解析错误的显示文案。
        }
    }
}

impl std::error::Error for ApiError {}
// 将 ApiError 标记为标准错误类型，支持 ? 传播与错误链。
