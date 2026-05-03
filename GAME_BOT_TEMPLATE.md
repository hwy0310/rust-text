# Rust + OpenCV 游戏图色自动化模板（桌面端）

适用场景：你玩的是**三国志战略版桌面端**，希望做图色识别自动化，并且要求**高效、低错误、逻辑清晰、可复用**。

## 一、核心设计建议（先看）

1. **分层设计**（不要把所有逻辑写在一个文件）：
   - `capture`：截图
   - `detector`：图色识别/模板匹配
   - `action`：鼠标键盘动作
   - `bot`：业务流程编排
2. **固定帧循环**：建议 50~120ms 一个 tick，不要忙等。
3. **局部 ROI 识别**：先缩小搜索区域再匹配，速度提升很明显。
4. **模板缓存**：模板图只加载一次，常驻内存。
5. **阈值分级**：
   - 关键按钮（如确认、战斗）阈值高一些（0.92+）
   - 非关键 UI（提示角标）阈值低一些（0.85 左右）
6. **双重确认防误点**：连续 2 帧都匹配到再点击。
7. **日志与截图回放**：识别失败时保存现场图，方便调参。

## 二、已给你的可复用模板代码

本仓库新增了这些模块（可直接复用）：

- `src/gamebot/capture.rs`：截图接口 `ScreenCapturer`
- `src/gamebot/detector.rs`：OpenCV 模板匹配 `TemplateDetector`
- `src/gamebot/action.rs`：输入控制接口 `InputController`
- `src/gamebot/bot.rs`：自动化引擎 `BotEngine`
- `src/gamebot/template.rs`：模板定义 `UiTemplate`
- `src/gamebot/error.rs`：统一错误 `BotError`

你只需要把 `DesktopCapturer` 和 `DesktopInput` 的占位实现替换成你本机可用实现即可。

## 三、最小调用示例

```rust
mod gamebot;

use gamebot::action::DesktopInput;
use gamebot::bot::BotEngine;
use gamebot::capture::DesktopCapturer;
use gamebot::template::UiTemplate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capturer = DesktopCapturer::new();
    let input = DesktopInput::new();
    let mut bot = BotEngine::new(capturer, input);

    let attack_btn = UiTemplate {
        name: "attack_btn".to_string(),
        image_path: "assets/templates/attack_btn.png".to_string(),
        threshold: 0.92,
        search_region: None,
    };

    bot.set_tick_ms(80);
    bot.loop_click_template(&attack_btn, 200)?;
    Ok(())
}
```

## 四、效率优化清单

- 优先 ROI（例如按钮只在右下角出现，就只扫右下角）
- 灰度匹配（若彩色不必要）
- 降采样匹配 + 原图二次确认
- 多模板按优先级排序，先匹配最关键模板
- 定时清理日志，避免 IO 拖慢

## 五、稳定性建议

- 为每个动作加冷却（防止连续误触）
- 关键操作前后都做状态检测（例如“已进入战斗界面”）
- 对网络波动场景加超时回退策略
- 每次版本更新后重新截图模板（UI 微调会导致识别率下降）

> 提醒：请确认自动化行为符合游戏条款与当地法律法规，避免账号风险。
