// 说明：此行用于实现下面这条 Rust 语句对应的功能。
use crate::gamebot::error::BotError;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub trait InputController {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn click(&mut self, x: i32, y: i32) -> Result<(), BotError>;
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn move_to(&mut self, x: i32, y: i32) -> Result<(), BotError>;
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn key_press(&mut self, key: &str) -> Result<(), BotError>;
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
pub struct DesktopInput;

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl DesktopInput {
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
impl InputController for DesktopInput {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn click(&mut self, x: i32, y: i32) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut enigo = Enigo::new(&Settings::default())
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        enigo
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .move_mouse(x, y, Coordinate::Abs)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("移动鼠标失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        enigo
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .button(Button::Left, Direction::Click)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("点击失败: {e}")))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn move_to(&mut self, x: i32, y: i32) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        use enigo::{Coordinate, Enigo, Mouse, Settings};
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut enigo = Enigo::new(&Settings::default())
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        enigo
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .move_mouse(x, y, Coordinate::Abs)
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("移动鼠标失败: {e}")))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn key_press(&mut self, key: &str) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        let mut enigo = Enigo::new(&Settings::default())
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        enigo
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .key(
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                Key::Unicode(key.chars().next().unwrap_or(' ')),
                // 说明：此行用于实现下面这条 Rust 语句对应的功能。
                Direction::Click,
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            )
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            .map_err(|e| BotError::Action(format!("按键失败: {e}")))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}

// 说明：此行用于实现下面这条 Rust 语句对应的功能。
#[cfg(not(target_os = "windows"))]
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
impl InputController for DesktopInput {
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn click(&mut self, _x: i32, _y: i32) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Err(BotError::Action(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn move_to(&mut self, _x: i32, _y: i32) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Err(BotError::Action(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }

    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    fn key_press(&mut self, _key: &str) -> Result<(), BotError> {
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        Err(BotError::Action(
            // 说明：此行用于实现下面这条 Rust 语句对应的功能。
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        // 说明：此行用于实现下面这条 Rust 语句对应的功能。
        ))
    // 说明：此行用于实现下面这条 Rust 语句对应的功能。
    }
// 说明：此行用于实现下面这条 Rust 语句对应的功能。
}
