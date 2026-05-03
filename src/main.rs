// 说明：此行用于实现下面这条 Rust 语句对应的功能。
mod api;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
mod gamebot;
// 声明 api 模块（包含 client 和 user_api）。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
mod error;
// 声明 error 模块（统一错误类型）。

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api::client::ApiClient;
// 引入通用 API 客户端。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api::user_api::UserApi;
// 引入业务接口封装。

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[tokio::main]
// 使用 tokio 异步运行时作为程序入口。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
async fn main() -> anyhow::Result<()> {
    // 主函数返回 anyhow::Result，便于统一错误处理。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    dotenvy::dotenv().ok();
    // 读取 .env 文件中的环境变量（若不存在则忽略）。

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let client = ApiClient::from_env()?;
    // 从环境变量创建客户端。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let user_api = UserApi::new(client);
    // 用客户端构造用户业务 API。

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let profile = user_api.get_user_profile("123").await?;
    // 调用获取用户信息接口。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    println!("用户信息: {profile}");
    // 打印用户信息结果。

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let created = user_api.create_user("Alice", "alice@example.com").await?;
    // 调用创建用户接口。
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    println!("创建结果: {created}");
    // 打印创建结果。

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Ok(())
    // 正常结束程序。
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
