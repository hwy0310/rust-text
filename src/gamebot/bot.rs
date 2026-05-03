// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::action::InputController;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::capture::ScreenCapturer;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::detector::TemplateDetector;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::template::UiTemplate;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use std::{thread, time::Duration};

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct BotEngine<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    capturer: C,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    detector: TemplateDetector,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    input: I,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    tick_ms: u64,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl<C, I> BotEngine<C, I>
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
where
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    C: ScreenCapturer,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    I: InputController,
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
{
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn new(capturer: C, input: I) -> Self {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Self {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            capturer,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            detector: TemplateDetector::new(),
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            input,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            tick_ms: 100,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn set_tick_ms(&mut self, tick_ms: u64) {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        self.tick_ms = tick_ms.max(16);
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn run_once_click_template(&mut self, tpl: &UiTemplate) -> Result<bool, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let frame = self.capturer.capture()?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if let Some(found) = self.detector.locate(&frame, tpl)? {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            let center_x = found.rect.x + found.rect.width / 2;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            let center_y = found.rect.y + found.rect.height / 2;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            self.input.click(center_x, center_y)?;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Ok(true);
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(false)
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn loop_click_template(
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        &mut self,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        tpl: &UiTemplate,
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        max_loops: u32,
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    ) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        for _ in 0..max_loops {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            let _ = self.run_once_click_template(tpl)?;
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            thread::sleep(Duration::from_millis(self.tick_ms));
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(())
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
