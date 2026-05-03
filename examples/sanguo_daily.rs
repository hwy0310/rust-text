// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api_demo::gamebot::action::DesktopInput;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api_demo::gamebot::bot::BotEngine;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api_demo::gamebot::capture::DesktopCapturer;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api_demo::gamebot::flow_sanguo::build_sanguo_daily_flow;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use api_demo::gamebot::runner::TaskRunner;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let capturer = DesktopCapturer::new();
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let input = DesktopInput::new();

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let mut engine = BotEngine::new(capturer, input);
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    engine.set_tick_ms(80);

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let runner = TaskRunner::new(engine);
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let mut flow = build_sanguo_daily_flow(runner);

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let state = flow.run()?;
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    println!("三国志战略版日常流程结束状态: {:?}", state);
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Ok(())
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
