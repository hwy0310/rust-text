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
