use crate::gamebot::error::BotError;

pub trait InputController {
    fn click(&mut self, x: i32, y: i32) -> Result<(), BotError>;
    fn move_to(&mut self, x: i32, y: i32) -> Result<(), BotError>;
    fn key_press(&mut self, key: &str) -> Result<(), BotError>;
}

pub struct DesktopInput;

impl DesktopInput {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "windows")]
impl InputController for DesktopInput {
    fn click(&mut self, x: i32, y: i32) -> Result<(), BotError> {
        use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| BotError::Action(format!("移动鼠标失败: {e}")))?;
        enigo
            .button(Button::Left, Direction::Click)
            .map_err(|e| BotError::Action(format!("点击失败: {e}")))
    }

    fn move_to(&mut self, x: i32, y: i32) -> Result<(), BotError> {
        use enigo::{Coordinate, Enigo, Mouse, Settings};
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| BotError::Action(format!("移动鼠标失败: {e}")))
    }

    fn key_press(&mut self, key: &str) -> Result<(), BotError> {
        use enigo::{Direction, Enigo, Key, Keyboard, Settings};
        let mut enigo = Enigo::new(&Settings::default())
            .map_err(|e| BotError::Action(format!("初始化输入失败: {e}")))?;
        enigo
            .key(
                Key::Unicode(key.chars().next().unwrap_or(' ')),
                Direction::Click,
            )
            .map_err(|e| BotError::Action(format!("按键失败: {e}")))
    }
}

#[cfg(not(target_os = "windows"))]
impl InputController for DesktopInput {
    fn click(&mut self, _x: i32, _y: i32) -> Result<(), BotError> {
        Err(BotError::Action(
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        ))
    }

    fn move_to(&mut self, _x: i32, _y: i32) -> Result<(), BotError> {
        Err(BotError::Action(
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        ))
    }

    fn key_press(&mut self, _key: &str) -> Result<(), BotError> {
        Err(BotError::Action(
            "DesktopInput 目前示例实现仅支持 Windows；请按平台替换输入实现".into(),
        ))
    }
}
