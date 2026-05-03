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
