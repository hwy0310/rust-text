use api_demo::gamebot::action::DesktopInput;
use api_demo::gamebot::bot::BotEngine;
use api_demo::gamebot::capture::DesktopCapturer;
use api_demo::gamebot::flow_sanguo::build_sanguo_daily_flow;
use api_demo::gamebot::runner::TaskRunner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capturer = DesktopCapturer::new();
    let input = DesktopInput::new();

    let mut engine = BotEngine::new(capturer, input);
    engine.set_tick_ms(80);

    let runner = TaskRunner::new(engine);
    let mut flow = build_sanguo_daily_flow(runner);

    let state = flow.run()?;
    println!("三国志战略版日常流程结束状态: {:?}", state);
    Ok(())
}
