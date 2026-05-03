use crate::gamebot::flow::{BattleFlow, FlowState};
use crate::gamebot::runner::{TaskPolicy, TaskRunner, TemplateTask};
use crate::gamebot::template::UiTemplate;

fn mk_task(
    name: &str,
    image_path: &str,
    threshold: f32,
    timeout_ms: u64,
    retry: u32,
) -> TemplateTask {
    TemplateTask {
        name: name.to_string(),
        template: UiTemplate {
            name: name.to_string(),
            image_path: image_path.to_string(),
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

/// 三国志战略版流程模板（可按你的界面改图和阈值）
pub fn build_sanguo_daily_flow<C, I>(runner: TaskRunner<C, I>) -> BattleFlow<C, I>
where
    C: crate::gamebot::capture::ScreenCapturer,
    I: crate::gamebot::action::InputController,
{
    BattleFlow {
        runner,
        // 进入战斗准备界面（例如“出征/攻打”按钮）
        prepare_task: mk_task(
            "prepare",
            "assets/templates/sanguo/prepare.png",
            0.92,
            5_000,
            25,
        ),
        // 点击战斗开始（例如“攻打/确认”）
        battle_task: mk_task(
            "battle",
            "assets/templates/sanguo/battle.png",
            0.93,
            8_000,
            35,
        ),
        // 战斗结束领奖（例如“领取/确定”）
        claim_task: mk_task(
            "claim",
            "assets/templates/sanguo/claim.png",
            0.91,
            6_000,
            30,
        ),
        // 异常恢复（例如“返回主城/关闭弹窗”）
        recover_task: mk_task(
            "recover",
            "assets/templates/sanguo/recover.png",
            0.88,
            4_000,
            20,
        ),
        state: FlowState::Prepare,
        // 最大状态迁移次数，避免意外死循环
        max_rounds: 800,
    }
}
