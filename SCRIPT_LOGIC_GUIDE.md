# 辅助脚本运行逻辑建议（高效 + 稳定）

你现在的模块已经有 `BotEngine`，建议再按“任务编排层”设计，避免脚本越写越乱。

## 1. 推荐的 4 层运行结构

1. **感知层**：截图 + 模板识别（`capture` + `detector`）
2. **动作层**：鼠标键盘执行（`action`）
3. **任务层**：单任务成功/超时/重试（新增 `runner`）
4. **流程层**：日常流程状态机（征兵 -> 扫荡 -> 领奖 -> 返回）

> 关键点：任务层和流程层分开，你后续维护会轻松很多。

## 2. 单任务标准模板（已提供）

新增了 `TaskRunner`：
- 输入：`TemplateTask`（模板 + 重试策略 + 冷却 + 超时）
- 输出：`TaskResult`（`Success` / `Timeout` / `RetryExceeded`）

这样你可以统一写：
- 点按钮任务
- 打开面板任务
- 关闭弹窗任务

## 3. 建议的流程状态机

```text
Idle
 └─> Prepare
      └─> ExecuteBattle
           ├─(成功)─> ClaimReward
           ├─(超时)─> Recover
           └─(重试超限)─> Abort
```

每个状态只做一件事：
- 先识别关键 UI
- 再动作
- 最后做结果校验（是否进入预期界面）

## 4. 稳定性关键建议

- **双帧确认**：同一模板连续命中两帧再点
- **动作冷却**：点击后 sleep 200~500ms
- **全局 watchdog**：N 秒没有状态推进就重启流程
- **失败截图**：超时时保存帧，便于调阈值

## 5. 性能关键建议

- 模板匹配优先 ROI
- 每轮最多识别 K 个关键模板（不要全量扫描）
- 常用模板常驻缓存（你现在已支持）
- 长循环里控制 tick（50~120ms）

## 6. 你可以直接复用的代码入口

- `src/gamebot/runner.rs`
  - `TaskPolicy`
  - `TemplateTask`
  - `TaskRunner::run_click_task`

后续你只需要再加一个 `flow.rs` 做状态机即可。

## 7. 已补充可直接套用的 flow 模板

新增 `src/gamebot/flow.rs`，包含：
- `FlowState`：`Prepare -> ExecuteBattle -> ClaimReward -> Recover/Abort/Done`
- `BattleFlow::run()`：主循环驱动状态迁移
- `handle_prepare/battle/claim/recover`：每个状态的明确处理函数

这份模板的意义：
- 把“识别 + 点击 + 判定”封装成统一任务
- 把“任务怎么串”变成可读状态机
- 方便你后续扩展到征兵、屯田、攻城等流程

## 8. 可直接改参数就用的 flow 示例

已新增 `examples/flow_demo.rs`，你只需要改四个模板图片路径：
- `assets/templates/prepare.png`
- `assets/templates/battle.png`
- `assets/templates/claim.png`
- `assets/templates/recover.png`

以及阈值与超时参数，就能直接复用整套流程。

> 注意：当前 `DesktopCapturer` / `DesktopInput` 仍是占位实现，先替换成你本机实现后再运行。

## 9. 选3：三国志战略版业务流程模板（已加）

已新增：
- `src/gamebot/flow_sanguo.rs`：`build_sanguo_daily_flow(...)`
- `examples/sanguo_daily.rs`：直接运行的流程示例入口

默认流程：
1. `prepare`（出征/攻打入口）
2. `battle`（开始战斗/确认）
3. `claim`（战斗后领奖）
4. `recover`（异常时返回主界面）

你只需要替换这 4 张图：
- `assets/templates/sanguo/prepare.png`
- `assets/templates/sanguo/battle.png`
- `assets/templates/sanguo/claim.png`
- `assets/templates/sanguo/recover.png`

然后根据你分辨率微调阈值：
- `battle` 推荐 0.93 起
- `claim` 推荐 0.91 起
- `recover` 推荐 0.88 起

## 10. Windows 实机接入（已实现基础版）

已在代码中接入：
- `DesktopCapturer`：使用 `screenshots` 截图并转 OpenCV `Mat`
- `DesktopInput`：使用 `enigo` 进行鼠标移动、点击、键盘按键

依赖：
- `enigo = "0.2"`
- `screenshots = "0.8"`

说明：
- 当前实现默认取第一个屏幕。
- 如你是多屏用户，建议扩展为“按窗口名/屏幕索引选择”。
