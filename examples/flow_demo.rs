mod gamebot {
    pub use api_demo::gamebot::*;
}

use gamebot::action::DesktopInput;
use gamebot::bot::BotEngine;
use gamebot::capture::DesktopCapturer;
use gamebot::flow::{BattleFlow, FlowState};
use gamebot::runner::{TaskPolicy, TaskRunner, TemplateTask};
use gamebot::template::UiTemplate;

fn task(name: &str, path: &str, threshold: f32, timeout_ms: u64, retry: u32) -> TemplateTask {
    TemplateTask {
        name: name.to_string(),
        template: UiTemplate {
            name: name.to_string(),
            image_path: path.to_string(),
            threshold,
            search_region: None,
        },
        policy: TaskPolicy {
            max_retry: retry,
            cooldown_ms: 300,
            timeout_ms,
        },
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capturer = DesktopCapturer::new();
    let input = DesktopInput::new();
    let mut engine = BotEngine::new(capturer, input);
    engine.set_tick_ms(80);

    let runner = TaskRunner::new(engine);

    let mut flow = BattleFlow {
        runner,
        prepare_task: task("prepare", "assets/templates/prepare.png", 0.91, 4_000, 20),
        battle_task: task("battle", "assets/templates/battle.png", 0.92, 6_000, 30),
        claim_task: task("claim", "assets/templates/claim.png", 0.90, 5_000, 25),
        recover_task: task("recover", "assets/templates/recover.png", 0.88, 4_000, 20),
        state: FlowState::Prepare,
        max_rounds: 500,
    };

    let final_state = flow.run()?;
    println!("flow finished: {:?}", final_state);
    Ok(())
}
