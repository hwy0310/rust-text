// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::error::ApiError;
// 引入统一错误类型，所有失败都转换成 ApiError。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use reqwest::{Client, Method};
// 引入 reqwest 的 HTTP 客户端与请求方法枚举。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use serde_json::Value;
// 使用动态 JSON 类型承载请求体/响应体。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::{env, time::Duration};
// 引入环境变量读取与时间间隔类型。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use tokio::time::sleep;
// 引入异步 sleep，用于重试退避。

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Clone)]
// 允许 ApiClient 被克隆，方便在多处复用。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct ApiClient {
    // 定义通用 API 客户端。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    base_url: String,
    // API 基础地址，例如 https://api.example.com。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    api_key: String,
    // 鉴权密钥（Bearer Token）。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    max_retries: u32,
    // 最大重试次数。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    client: Client,
    // reqwest 底层客户端实例。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl ApiClient {
    // 为 ApiClient 实现构造与请求方法。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn from_env() -> Result<Self, ApiError> {
        // 从环境变量构建客户端。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let base_url = env::var("API_BASE_URL")
            // 读取 API_BASE_URL 环境变量。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|_| ApiError::Config("API_BASE_URL 未配置".into()))?
            // 若缺失则返回配置错误。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .trim_end_matches('/')
            // 去掉末尾 /，避免拼接重复斜杠。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .to_string();
        // 转为 String。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let api_key = env::var("API_KEY")
            // 读取 API_KEY 环境变量。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|_| ApiError::Config("API_KEY 未配置".into()))?;
        // 若缺失则返回配置错误。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let timeout_secs: u64 = env::var("API_TIMEOUT_SECS")
            // 读取请求超时时间（秒）。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .unwrap_or_else(|_| "15".into())
            // 未设置时默认 15 秒。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .parse()
            // 字符串转数字。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|_| ApiError::Config("API_TIMEOUT_SECS 不是合法数字".into()))?;
        // 非法数字则返回配置错误。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let max_retries: u32 = env::var("API_RETRY")
            // 读取最大重试次数。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .unwrap_or_else(|_| "2".into())
            // 未设置时默认重试 2 次。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .parse()
            // 字符串转数字。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|_| ApiError::Config("API_RETRY 不是合法数字".into()))?;
        // 非法数字则返回配置错误。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let client = Client::builder()
            // 创建 reqwest 客户端构建器。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .timeout(Duration::from_secs(timeout_secs))
            // 设置统一请求超时。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .build()
            // 构建客户端实例。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| ApiError::Http(format!("创建 HTTP 客户端失败: {e}")))?;
        // 构建失败则转换为 HTTP 类错误。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(Self {
            // 返回构建好的 ApiClient。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            base_url,
            // 保存基础地址。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            api_key,
            // 保存密钥。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            max_retries,
            // 保存重试次数。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            client,
            // 保存 reqwest 客户端。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        })
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub async fn get(&self, path: &str) -> Result<Value, ApiError> {
        // 对外暴露 GET 请求接口。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.request(Method::GET, path, None).await
        // 复用统一 request 方法。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub async fn post(&self, path: &str, body: Value) -> Result<Value, ApiError> {
        // 对外暴露 POST 请求接口。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.request(Method::POST, path, Some(body)).await
        // 复用统一 request 方法并传入请求体。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    async fn request(
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        &self,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        method: Method,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        path: &str,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        body: Option<Value>,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    ) -> Result<Value, ApiError> {
        // 私有统一请求方法：封装鉴权、重试、解析等逻辑。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        // 拼接完整 URL，并去掉 path 前导 /。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut last_err = String::new();
        // 记录最后一次失败原因。

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        for attempt in 0..=self.max_retries {
            // 按配置次数重试（含首次请求）。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            let mut req = self
                // 创建可变请求构建器。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                .client
                // 使用内部 reqwest 客户端。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                .request(method.clone(), &url)
                // 设置 HTTP 方法与 URL。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                .header("Authorization", format!("Bearer {}", self.api_key))
                // 设置 Bearer 鉴权头。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                .header("Accept", "application/json")
                // 声明接受 JSON 响应。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                .header("Content-Type", "application/json");
            // 声明请求体为 JSON。

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            if let Some(ref b) = body {
                // 如果有请求体，则设置 JSON body。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                req = req.json(b);
                // 将 body 序列化后放入请求。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            }

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            match req.send().await {
                // 发送请求并等待响应。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                Ok(resp) => {
                    // 请求成功发出并拿到响应对象。
                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    let status = resp.status();
                    // 提取 HTTP 状态码。
                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    let text = resp
                        // 读取响应文本内容。
                        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                        .text()
                        // 异步读取响应 body 为字符串。
                        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                        .await
                        // 等待读取完成。
                        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                        .map_err(|e| ApiError::Http(format!("读取响应失败: {e}")))?;
                    // 读取失败时转换为 HTTP 错误。

                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    if !status.is_success() {
                        // 若状态码非 2xx，视为失败。
                        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                        last_err = format!("status={} body={}", status, text);
                        // 记录失败状态和响应体片段。
                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    } else {
                        // 状态码成功，尝试解析 JSON。
                        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                        return serde_json::from_str::<Value>(&text)
                            // 将响应文本反序列化为 JSON。
                            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                            .map_err(|e| {
                                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                                ApiError::Parse(format!("JSON 解析失败: {e}; body={}", text))
                            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                            });
                        // 解析失败时返回解析错误。
                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    }
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                }
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                Err(e) => {
                    // 请求发送失败（网络、超时等）。
                    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                    last_err = format!("请求失败: {e}");
                    // 记录错误信息以便重试后返回。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                }
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            }

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            if attempt < self.max_retries {
                // 若还没到最后一次尝试，执行退避等待。
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                // 简单线性退避：500ms、1000ms、1500ms...
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            }
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Err(ApiError::Http(format!("重试后仍失败: {last_err}")))
        // 所有尝试都失败后返回最终错误。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
