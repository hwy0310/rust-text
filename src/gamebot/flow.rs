use crate::gamebot::error::BotError;
use crate::gamebot::runner::{TaskResult, TaskRunner, TemplateTask};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowState {
    Prepare,
    ExecuteBattle,
    ClaimReward,
    Recover,
    Abort,
    Done,
}

pub struct BattleFlow<C, I>
where
    C: crate::gamebot::capture::ScreenCapturer,
    I: crate::gamebot::action::InputController,
{
    pub runner: TaskRunner<C, I>,
    pub prepare_task: TemplateTask,
    pub battle_task: TemplateTask,
    pub claim_task: TemplateTask,
    pub recover_task: TemplateTask,
    pub state: FlowState,
    pub max_rounds: u32,
}

impl<C, I> BattleFlow<C, I>
where
    C: crate::gamebot::capture::ScreenCapturer,
    I: crate::gamebot::action::InputController,
{
    pub fn run(&mut self) -> Result<FlowState, BotError> {
        let mut rounds = 0;

        while rounds < self.max_rounds {
            self.state = match self.state {
                FlowState::Prepare => self.handle_prepare()?,
                FlowState::ExecuteBattle => self.handle_battle()?,
                FlowState::ClaimReward => self.handle_claim()?,
                FlowState::Recover => self.handle_recover()?,
                FlowState::Abort => return Ok(FlowState::Abort),
                FlowState::Done => return Ok(FlowState::Done),
            };

            rounds += 1;
        }

        Ok(FlowState::Abort)
    }

    fn handle_prepare(&mut self) -> Result<FlowState, BotError> {
        match self.runner.run_click_task(&self.prepare_task)? {
            TaskResult::Success => Ok(FlowState::ExecuteBattle),
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        }
    }

    fn handle_battle(&mut self) -> Result<FlowState, BotError> {
        match self.runner.run_click_task(&self.battle_task)? {
            TaskResult::Success => Ok(FlowState::ClaimReward),
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        }
    }

    fn handle_claim(&mut self) -> Result<FlowState, BotError> {
        match self.runner.run_click_task(&self.claim_task)? {
            TaskResult::Success => Ok(FlowState::Done),
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Recover),
        }
    }

    fn handle_recover(&mut self) -> Result<FlowState, BotError> {
        match self.runner.run_click_task(&self.recover_task)? {
            TaskResult::Success => Ok(FlowState::Prepare),
            TaskResult::Timeout | TaskResult::RetryExceeded => Ok(FlowState::Abort),
        }
    }
}
