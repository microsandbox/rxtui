//! Simple timer example with effects
//!
//! Run with: cargo run --example timer --features effects

use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    Tick,
    Toggle,
    Reset,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct State {
    seconds: u32,
    running: bool,
}

#[derive(Component, Clone)]
struct Timer;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

#[component]
impl Timer {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: State) -> Action {
        match msg {
            Msg::Tick => {
                if state.running {
                    state.seconds += 1;
                }
            }
            Msg::Toggle => state.running = !state.running,
            Msg::Reset => {
                state.seconds = 0;
                state.running = false;
            }
            Msg::Exit => return Action::exit(),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: State) -> Node {
        let time = format!("{:02}:{:02}", state.seconds / 60, state.seconds % 60);
        let status = if state.running { "Running" } else { "Paused" };

        node! {
            div(bg: black, dir: vertical, pad: 2, h: 12, gap: 2) [
                // Timer display
                div(bg: "#1a1a1a", border: white, pad: 1, w_pct: 1.0) [
                    div(dir: vertical) [
                        text(&time, color: bright_cyan, bold),
                        text(status, color: (if state.running { Color::Green } else { Color::Yellow }))
                    ]
                ],

                // Help text
                text("Space: Start/Pause | R: Reset | Q: Quit", color: "#666"),

                // Key handlers
                @key(Char(' ')): ctx.handler(Msg::Toggle),
                @key(Char('r')): ctx.handler(Msg::Reset),
                @key(Char('q')): ctx.handler(Msg::Exit),
                @key(Esc): ctx.handler(Msg::Exit)
            ]
        }
    }

    #[cfg(feature = "effects")]
    #[effects]
    async fn tick_timer(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            ctx.send(Msg::Tick);
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    #[cfg(not(feature = "effects"))]
    {
        eprintln!("This example requires the 'effects' feature.");
        eprintln!("Run with: cargo run --example timer --features effects");
        return Ok(());
    }

    let mut app = App::new()?;
    app.run(Timer)?;
    Ok(())
}
