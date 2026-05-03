// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::bot::BotEngine;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::template::UiTemplate;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::time::{Duration, Instant};

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct TaskPolicy {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub max_retry: u32,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub cooldown_ms: u64,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub timeout_ms: u64,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct TemplateTask {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub name: String,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub template: UiTemplate,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub policy: TaskPolicy,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub enum TaskResult {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Success,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    Timeout,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    RetryExceeded,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct TaskRunner<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: crate::gamebot::capture::ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: crate::gamebot::action::InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub engine: BotEngine<C, I>,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl<C, I> TaskRunner<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: crate::gamebot::capture::ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: crate::gamebot::action::InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn new(engine: BotEngine<C, I>) -> Self {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Self { engine }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn run_click_task(&mut self, task: &TemplateTask) -> Result<TaskResult, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let begin = Instant::now();
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut retry = 0;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        while begin.elapsed() < Duration::from_millis(task.policy.timeout_ms) {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            let clicked = self.engine.run_once_click_template(&task.template)?;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            if clicked {
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                std::thread::sleep(Duration::from_millis(task.policy.cooldown_ms));
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                return Ok(TaskResult::Success);
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            }

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            retry += 1;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            if retry > task.policy.max_retry {
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                return Ok(TaskResult::RetryExceeded);
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            }

            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            std::thread::sleep(Duration::from_millis(50));
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(TaskResult::Timeout)
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
