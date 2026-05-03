use crate::gamebot::bot::BotEngine;
use crate::gamebot::error::BotError;
use crate::gamebot::template::UiTemplate;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TaskPolicy {
    pub max_retry: u32,
    pub cooldown_ms: u64,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct TemplateTask {
    pub name: String,
    pub template: UiTemplate,
    pub policy: TaskPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskResult {
    Success,
    Timeout,
    RetryExceeded,
}

pub struct TaskRunner<C, I>
where
    C: crate::gamebot::capture::ScreenCapturer,
    I: crate::gamebot::action::InputController,
{
    pub engine: BotEngine<C, I>,
}

impl<C, I> TaskRunner<C, I>
where
    C: crate::gamebot::capture::ScreenCapturer,
    I: crate::gamebot::action::InputController,
{
    pub fn new(engine: BotEngine<C, I>) -> Self {
        Self { engine }
    }

    pub fn run_click_task(&mut self, task: &TemplateTask) -> Result<TaskResult, BotError> {
        let begin = Instant::now();
        let mut retry = 0;

        while begin.elapsed() < Duration::from_millis(task.policy.timeout_ms) {
            let clicked = self.engine.run_once_click_template(&task.template)?;
            if clicked {
                std::thread::sleep(Duration::from_millis(task.policy.cooldown_ms));
                return Ok(TaskResult::Success);
            }

            retry += 1;
            if retry > task.policy.max_retry {
                return Ok(TaskResult::RetryExceeded);
            }

            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(TaskResult::Timeout)
    }
}
