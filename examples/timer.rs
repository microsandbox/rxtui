use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum TimerMsg {
    Tick,
    Exit,
}

#[derive(Component, Clone)]
struct Timer;

#[component]
impl Timer {
    #[update]
    fn update(&self, _ctx: &Context, msg: TimerMsg, mut state: u64) -> Action {
        match msg {
            TimerMsg::Tick => {
                state += 1;
                Action::update(state)
            }
            TimerMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: u64) -> Node {
        node! {
            div(bg: black, pad: 2, gap: 2) [
                text(format!("Timer: {} seconds", state), color: white),
                text("Press Esc to exit", color: bright_black),
                @key_global(Esc): ctx.handler(TimerMsg::Exit)
            ]
        }
    }

    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            ctx.send(TimerMsg::Tick);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(Timer)
}
