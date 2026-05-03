// 说明：此行用于实现下面这条 Rust 语句对应的功能。
mod gamebot {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub use api_demo::gamebot::*;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::action::DesktopInput;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::bot::BotEngine;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::capture::DesktopCapturer;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::flow::{BattleFlow, FlowState};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::runner::{TaskPolicy, TaskRunner, TemplateTask};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use gamebot::template::UiTemplate;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
fn task(name: &str, path: &str, threshold: f32, timeout_ms: u64, retry: u32) -> TemplateTask {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    TemplateTask {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        name: name.to_string(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        template: UiTemplate {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            name: name.to_string(),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            image_path: path.to_string(),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            threshold,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            search_region: None,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        },
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        policy: TaskPolicy {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            max_retry: retry,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            cooldown_ms: 300,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            timeout_ms,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        },
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

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
    let mut flow = BattleFlow {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        runner,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        prepare_task: task("prepare", "assets/templates/prepare.png", 0.91, 4_000, 20),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        battle_task: task("battle", "assets/templates/battle.png", 0.92, 6_000, 30),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        claim_task: task("claim", "assets/templates/claim.png", 0.90, 5_000, 25),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        recover_task: task("recover", "assets/templates/recover.png", 0.88, 4_000, 20),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        state: FlowState::Prepare,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        max_rounds: 500,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    };

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    let final_state = flow.run()?;
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    println!("flow finished: {:?}", final_state);
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Ok(())
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
