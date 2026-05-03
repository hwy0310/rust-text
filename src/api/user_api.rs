// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::api::client::ApiClient;
// 引入通用 API 客户端。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::error::ApiError;
// 引入统一错误类型。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use serde_json::{json, Value};
// 引入 JSON 宏和动态 JSON 值类型。

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct UserApi {
    // 定义用户业务接口封装结构体。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    client: ApiClient,
    // 持有通用客户端，所有请求通过它发出。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl UserApi {
    // 为 UserApi 实现具体业务方法。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn new(client: ApiClient) -> Self {
        // 构造函数：注入通用客户端。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Self { client }
        // 返回 UserApi 实例。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub async fn get_user_profile(&self, user_id: &str) -> Result<Value, ApiError> {
        // 获取用户资料：异步返回 JSON 或 ApiError。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.client.get(&format!("/v1/users/{user_id}")).await
        // 调用通用 GET 方法请求 /v1/users/{id}。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub async fn create_user(&self, name: &str, email: &str) -> Result<Value, ApiError> {
        // 创建用户：异步提交 name/email 并返回 JSON 或 ApiError。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let body = json!({
        // 组装请求体 JSON。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "name": name,
            // 请求体字段：用户名。
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "email": email,
            // 请求体字段：邮箱。
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        });
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.client.post("/v1/users", body).await
        // 调用通用 POST 方法请求 /v1/users。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
