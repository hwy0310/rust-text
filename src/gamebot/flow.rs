// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::runner::{TaskResult, TaskRunner, TemplateTask};

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub enum FlowState {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Prepare,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    ExecuteBattle,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    ClaimReward,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Recover,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Abort,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Done,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct BattleFlow<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: crate::gamebot::capture::ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: crate::gamebot::action::InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub runner: TaskRunner<C, I>,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub prepare_task: TemplateTask,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub battle_task: TemplateTask,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub claim_task: TemplateTask,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub recover_task: TemplateTask,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub state: FlowState,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub max_rounds: u32,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl<C, I> BattleFlow<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: crate::gamebot::capture::ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: crate::gamebot::action::InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn run(&mut self) -> Result<FlowState, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut rounds = 0;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        while rounds < self.max_rounds {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            self.state = match self.state {
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::Prepare => self.handle_prepare()?,
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::ExecuteBattle => self.handle_battle()?,
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::ClaimReward => self.handle_claim()?,
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::Recover => self.handle_recover()?,
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::Abort => return Ok(FlowState::Abort),
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                FlowState::Done => return Ok(FlowState::Done),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            };

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            rounds += 1;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(FlowState::Abort)
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn handle_prepare(&mut self) -> Result<FlowState, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self.runner.run_click_task(&self.prepare_task)? {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Success => Ok(FlowState::ExecuteBattle),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn handle_battle(&mut self) -> Result<FlowState, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self.runner.run_click_task(&self.battle_task)? {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Success => Ok(FlowState::ClaimReward),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn handle_claim(&mut self) -> Result<FlowState, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self.runner.run_click_task(&self.claim_task)? {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Success => Ok(FlowState::Done),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn handle_recover(&mut self) -> Result<FlowState, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        match self.runner.run_click_task(&self.recover_task)? {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Success => Ok(FlowState::Prepare),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Abort),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
