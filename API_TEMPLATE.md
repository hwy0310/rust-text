# API 调用模板（Rust 版：新手友好 + 便于维护）

> 目标：给新手一个可以直接复制的 Rust API 调用模板，并让后续维护更轻松。

---

## 1) 推荐目录结构

```text
project/
  ├─ src/
  │   ├─ main.rs
  │   ├─ api/
  │   │   ├─ mod.rs
  │   │   ├─ client.rs      # 统一请求入口（超时、重试、鉴权）
  │   │   └─ user_api.rs    # 按业务拆分接口
  │   └─ error.rs           # 统一错误定义
  ├─ .env.example
  ├─ Cargo.toml
  └─ README.md
```

---

## 2) `Cargo.toml` 依赖示例

```toml
[package]
name = "api-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
dotenvy = "0.15"
```

---

## 3) 环境变量模板（`.env.example`）

```env
API_BASE_URL=https://api.example.com
API_KEY=replace_me
API_TIMEOUT_SECS=15
API_RETRY=2
```

---

## 4) 统一错误（`src/error.rs`）

```rust
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
```

---

## 5) 通用 API Client（`src/api/client.rs`）

```rust
use crate::error::ApiError;
use reqwest::{Client, Method};
use serde_json::Value;
use std::{env, time::Duration};
use tokio::time::sleep;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    api_key: String,
    timeout_secs: u64,
    max_retries: u32,
    client: Client,
}

impl ApiClient {
    pub fn from_env() -> Result<Self, ApiError> {
        let base_url = env::var("API_BASE_URL")
            .map_err(|_| ApiError::Config("API_BASE_URL 未配置".into()))?
            .trim_end_matches('/')
            .to_string();

        let api_key = env::var("API_KEY")
            .map_err(|_| ApiError::Config("API_KEY 未配置".into()))?;

        let timeout_secs: u64 = env::var("API_TIMEOUT_SECS")
            .unwrap_or_else(|_| "15".into())
            .parse()
            .map_err(|_| ApiError::Config("API_TIMEOUT_SECS 不是合法数字".into()))?;

        let max_retries: u32 = env::var("API_RETRY")
            .unwrap_or_else(|_| "2".into())
            .parse()
            .map_err(|_| ApiError::Config("API_RETRY 不是合法数字".into()))?;

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| ApiError::Http(format!("创建 HTTP 客户端失败: {e}")))?;

        Ok(Self {
            base_url,
            api_key,
            timeout_secs,
            max_retries,
            client,
        })
    }

    pub async fn get(&self, path: &str) -> Result<Value, ApiError> {
        self.request(Method::GET, path, None).await
    }

    pub async fn post(&self, path: &str, body: Value) -> Result<Value, ApiError> {
        self.request(Method::POST, path, Some(body)).await
    }

    async fn request(&self, method: Method, path: &str, body: Option<Value>) -> Result<Value, ApiError> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let mut last_err = String::new();

        for attempt in 0..=self.max_retries {
            let mut req = self
                .client
                .request(method.clone(), &url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json");

            if let Some(ref b) = body {
                req = req.json(b);
            }

            match req.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp
                        .text()
                        .await
                        .map_err(|e| ApiError::Http(format!("读取响应失败: {e}")))?;

                    if !status.is_success() {
                        last_err = format!("status={} body={}", status, text);
                    } else {
                        return serde_json::from_str::<Value>(&text)
                            .map_err(|e| ApiError::Parse(format!("JSON 解析失败: {e}; body={}", text)));
                    }
                }
                Err(e) => {
                    last_err = format!("请求失败: {e}");
                }
            }

            if attempt < self.max_retries {
                sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
            }
        }

        Err(ApiError::Http(format!("重试后仍失败: {last_err}")))
    }
}
```

---

## 6) 业务接口封装（`src/api/user_api.rs`）

```rust
use crate::api::client::ApiClient;
use crate::error::ApiError;
use serde_json::{json, Value};

pub struct UserApi {
    client: ApiClient,
}

impl UserApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    pub async fn get_user_profile(&self, user_id: &str) -> Result<Value, ApiError> {
        self.client.get(&format!("/v1/users/{user_id}")).await
    }

    pub async fn create_user(&self, name: &str, email: &str) -> Result<Value, ApiError> {
        let body = json!({
            "name": name,
            "email": email,
        });
        self.client.post("/v1/users", body).await
    }
}
```

---

## 7) 模块导出（`src/api/mod.rs`）

```rust
pub mod client;
pub mod user_api;
```

---

## 8) 运行示例（`src/main.rs`）

```rust
mod api;
mod error;

use api::client::ApiClient;
use api::user_api::UserApi;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let client = ApiClient::from_env()?;
    let user_api = UserApi::new(client);

    let profile = user_api.get_user_profile("123").await?;
    println!("用户信息: {profile}");

    let created = user_api.create_user("Alice", "alice@example.com").await?;
    println!("创建结果: {created}");

    Ok(())
}
```

---

## 9) 新手维护清单

1. 先复制 `.env.example` 为 `.env` 并填入真实密钥。  
2. 业务代码**不要直接**写 `reqwest::Client` 请求，统一走 `ApiClient`。  
3. 新增接口时，按 `user_api.rs` 方式封装为明确方法。  
4. 保持错误可读（状态码 + 响应片段 + 接口路径）。  
5. 后续可加：结构化日志、按错误类型区分重试、响应结构体校验（Serde）。

---

## 10) 最小可复制版本（仅验证密钥）

```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = "https://api.example.com";
    let key = "replace_me";

    let resp = Client::new()
        .get(format!("{base}/v1/ping"))
        .header("Authorization", format!("Bearer {key}"))
        .send()
        .await?;

    println!("status = {}", resp.status());
    println!("body = {}", resp.text().await?);
    Ok(())
}
```
