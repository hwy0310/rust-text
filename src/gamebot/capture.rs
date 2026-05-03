use crate::gamebot::error::BotError;
use opencv::core::Mat;

pub trait ScreenCapturer {
    fn capture(&mut self) -> Result<Mat, BotError>;
}

pub struct DesktopCapturer;

impl DesktopCapturer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "windows")]
impl ScreenCapturer for DesktopCapturer {
    fn capture(&mut self) -> Result<Mat, BotError> {
        use screenshots::Screen;

        let screens = Screen::all().map_err(|e| BotError::Capture(format!("读取屏幕失败: {e}")))?;
        let screen = screens
            .first()
            .ok_or_else(|| BotError::Capture("未找到可用屏幕".into()))?;

        let image = screen
            .capture()
            .map_err(|e| BotError::Capture(format!("截图失败: {e}")))?;

        let width = image.width() as i32;
        let height = image.height() as i32;
        let rgba = image.into_raw();

        let mat_1d =
            Mat::from_slice(&rgba).map_err(|e| BotError::Capture(format!("构建 Mat 失败: {e}")))?;
        let rgba_mat = mat_1d
            .reshape(4, height)
            .map_err(|e| BotError::Capture(format!("reshape 失败: {e}")))?;

        let mut bgr = Mat::default();
        opencv::imgproc::cvt_color(&rgba_mat, &mut bgr, opencv::imgproc::COLOR_RGBA2BGR, 0)
            .map_err(|e| BotError::Capture(format!("颜色转换失败: {e}")))?;

        if bgr.cols() != width || bgr.rows() != height {
            return Err(BotError::Capture("截图尺寸异常".into()));
        }

        Ok(bgr)
    }
}

#[cfg(not(target_os = "windows"))]
impl ScreenCapturer for DesktopCapturer {
    fn capture(&mut self) -> Result<Mat, BotError> {
        Err(BotError::Capture(
            "DesktopCapturer 目前示例实现仅支持 Windows；请按平台替换截图实现".into(),
        ))
    }
}
