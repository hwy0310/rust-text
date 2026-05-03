// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::flow::{BattleFlow, FlowState};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::runner::{TaskPolicy, TaskRunner, TemplateTask};
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::template::UiTemplate;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
fn mk_task(
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    name: &str,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    image_path: &str,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    threshold: f32,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    timeout_ms: u64,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    retry: u32,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
) -> TemplateTask {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    TemplateTask {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        name: name.to_string(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        template: UiTemplate {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            name: name.to_string(),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            image_path: image_path.to_string(),
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

/// 三国志战略版流程模板（可按你的界面改图和阈值）
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub fn build_sanguo_daily_flow<C, I>(runner: TaskRunner<C, I>) -> BattleFlow<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: crate::gamebot::capture::ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: crate::gamebot::action::InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    BattleFlow {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        runner,
        // 进入战斗准备界面（例如“出征/攻打”按钮）
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        prepare_task: mk_task(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "prepare",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "assets/templates/sanguo/prepare.png",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            0.92,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            5_000,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            25,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ),
        // 点击战斗开始（例如“攻打/确认”）
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        battle_task: mk_task(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "battle",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "assets/templates/sanguo/battle.png",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            0.93,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            8_000,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            35,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ),
        // 战斗结束领奖（例如“领取/确定”）
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        claim_task: mk_task(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "claim",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "assets/templates/sanguo/claim.png",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            0.91,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            6_000,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            30,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ),
        // 异常恢复（例如“返回主城/关闭弹窗”）
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        recover_task: mk_task(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "recover",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "assets/templates/sanguo/recover.png",
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            0.88,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            4_000,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            20,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        state: FlowState::Prepare,
        // 最大状态迁移次数，避免意外死循环
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        max_rounds: 800,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
