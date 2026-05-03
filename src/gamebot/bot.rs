use crate::gamebot::action::InputController;
use crate::gamebot::capture::ScreenCapturer;
use crate::gamebot::detector::TemplateDetector;
use crate::gamebot::error::BotError;
use crate::gamebot::template::UiTemplate;
use std::{thread, time::Duration};

pub struct BotEngine<C, I>
where
    C: ScreenCapturer,
    I: InputController,
{
    capturer: C,
    detector: TemplateDetector,
    input: I,
    tick_ms: u64,
}

impl<C, I> BotEngine<C, I>
where
    C: ScreenCapturer,
    I: InputController,
{
    pub fn new(capturer: C, input: I) -> Self {
        Self {
            capturer,
            detector: TemplateDetector::new(),
            input,
            tick_ms: 100,
        }
    }

    pub fn set_tick_ms(&mut self, tick_ms: u64) {
        self.tick_ms = tick_ms.max(16);
    }

    pub fn run_once_click_template(&mut self, tpl: &UiTemplate) -> Result<bool, BotError> {
        let frame = self.capturer.capture()?;
        if let Some(found) = self.detector.locate(&frame, tpl)? {
            let center_x = found.rect.x + found.rect.width / 2;
            let center_y = found.rect.y + found.rect.height / 2;
            self.input.click(center_x, center_y)?;
            return Ok(true);
        }
        Ok(false)
    }

    pub fn loop_click_template(
        &mut self,
        tpl: &UiTemplate,
        max_loops: u32,
    ) -> Result<(), BotError> {
        for _ in 0..max_loops {
            let _ = self.run_once_click_template(tpl)?;
            thread::sleep(Duration::from_millis(self.tick_ms));
        }
        Ok(())
    }
}
