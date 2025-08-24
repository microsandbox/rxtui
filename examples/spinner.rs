use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    Tick,
    Exit,
}

#[derive(Component)]
struct Spinner;

#[component]
impl Spinner {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, state: usize) -> Action {
        match msg {
            Msg::Tick => Action::update((state + 1) % 8),
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: usize) -> Node {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
        let frame = frames[state];

        node! {
            div(bg: black, pad: 2) [
                richtext [
                    text(format!("Spinner: {frame}"), color: cyan, bold),
                    text(" Press Esc to exit", color: bright_black, italic)
                ],
                @key(Esc): ctx.handler(Msg::Exit)
            ]
        }
    }

    #[effect]
    async fn animate(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(80)).await;
            ctx.send(Msg::Tick);
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Spinner)
}
