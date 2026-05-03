use crate::error::ApiError;
use reqwest::{Client, Method};
use serde_json::Value;
use std::{env, time::Duration};
use tokio::time::sleep;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    api_key: String,
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
