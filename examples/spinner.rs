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

        let purple = Color::hex("#9370DB"); // Medium purple
        let light_purple = Color::hex("#B19CD9"); // Light purple

        node! {
            div(
                w_pct: 1.0,
                h_pct: 1.0,
                align: center,
                justify: center,
                @key(esc): ctx.handler(Msg::Exit)
            ) [
                div(
                    pad: 3,
                    gap: 2,
                    border_style: rounded,
                    border_color: purple,
                ) [
                    div(dir: horizontal, gap: 2, align: center) [
                        text(frame, color: purple, bold),
                        text("Loading...", color: light_purple, bold)
                    ],
                    text("press ESC to exit", color: (Color::hex("#666666")))
                ]
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
