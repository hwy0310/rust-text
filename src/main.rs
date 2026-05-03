mod api;
mod gamebot;
// 声明 api 模块（包含 client 和 user_api）。
mod error;
// 声明 error 模块（统一错误类型）。

use api::client::ApiClient;
// 引入通用 API 客户端。
use api::user_api::UserApi;
// 引入业务接口封装。

#[tokio::main]
// 使用 tokio 异步运行时作为程序入口。
async fn main() -> anyhow::Result<()> {
    // 主函数返回 anyhow::Result，便于统一错误处理。
    dotenvy::dotenv().ok();
    // 读取 .env 文件中的环境变量（若不存在则忽略）。

    let client = ApiClient::from_env()?;
    // 从环境变量创建客户端。
    let user_api = UserApi::new(client);
    // 用客户端构造用户业务 API。

    let profile = user_api.get_user_profile("123").await?;
    // 调用获取用户信息接口。
    println!("用户信息: {profile}");
    // 打印用户信息结果。

    let created = user_api.create_user("Alice", "alice@example.com").await?;
    // 调用创建用户接口。
    println!("创建结果: {created}");
    // 打印创建结果。

    Ok(())
    // 正常结束程序。
}
