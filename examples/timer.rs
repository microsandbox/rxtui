use rxtui::prelude::*;

#[derive(Component)]
struct Timer;

#[component]
impl Timer {
    #[update]
    fn update(&self, _ctx: &Context, tick: bool, state: u64) -> Action {
        if !tick {
            return Action::exit();
        }

        Action::update(state + 1)
    }

    #[view]
    fn view(&self, ctx: &Context, state: u64) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Timer: {} seconds [Press Esc to exit]", state), color: white),
                @key(Esc): ctx.handler(false)
            ]
        }
    }

    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            ctx.send(true);
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Timer)
}
