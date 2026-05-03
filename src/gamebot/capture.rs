// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use opencv::core::Mat;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub trait ScreenCapturer {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn capture(&mut self) -> Result<Mat, BotError>;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct DesktopCapturer;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl DesktopCapturer {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    pub fn new() -> Self {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Self
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[cfg(target_os = "windows")]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl ScreenCapturer for DesktopCapturer {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn capture(&mut self) -> Result<Mat, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        use screenshots::Screen;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let screens = Screen::all().map_err(|e| BotError::Capture(format!("读取屏幕失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let screen = screens
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .first()
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .ok_or_else(|| BotError::Capture("未找到可用屏幕".into()))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let image = screen
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .capture()
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Capture(format!("截图失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let width = image.width() as i32;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let height = image.height() as i32;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let rgba = image.into_raw();

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mat_1d =
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            Mat::from_slice(&rgba).map_err(|e| BotError::Capture(format!("构建 Mat 失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let rgba_mat = mat_1d
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .reshape(4, height)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Capture(format!("reshape 失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut bgr = Mat::default();
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        opencv::imgproc::cvt_color(&rgba_mat, &mut bgr, opencv::imgproc::COLOR_RGBA2BGR, 0)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Capture(format!("颜色转换失败: {e}")))?;

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        if bgr.cols() != width || bgr.rows() != height {
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            return Err(BotError::Capture("截图尺寸异常".into()));
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        }

        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Ok(bgr)
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[cfg(not(target_os = "windows"))]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl ScreenCapturer for DesktopCapturer {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn capture(&mut self) -> Result<Mat, BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Err(BotError::Capture(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "DesktopCapturer 目前示例实现仅支持 Windows；请按平台替换截图实现".into(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
