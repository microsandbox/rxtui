use rxtui::prelude::*;

#[derive(Component)]
struct Stopwatch;

#[component]
impl Stopwatch {
    #[update]
    fn update(&self, _ctx: &Context, tick: bool, state: u64) -> Action {
        if !tick {
            return Action::exit();
        }

        Action::update(state + 10) // Increment by 10ms
    }

    #[view]
    fn view(&self, ctx: &Context, state: u64) -> Node {
        let seconds = state / 1000;
        let centiseconds = (state % 1000) / 10;

        node! {
            div(bg: black, pad: 2, @key(esc): ctx.handler(false)) [
                text(format!("Elapsed: {}.{:02}s", seconds, centiseconds), color: white, bold),
                text("press esc to exit", color: bright_black)
            ]
        }
    }

    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            ctx.send(true);
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.fast_polling().run(Stopwatch)
}
