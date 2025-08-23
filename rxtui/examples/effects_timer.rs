//! Example demonstrating async effects with a timer
//!
//! This example shows how to use effects to run background tasks
//! that update the UI through messages.
//!
//! Run with: cargo run --example effects_timer --features effects

use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Messages for the timer component
#[derive(Debug, Clone)]
enum TimerMsg {
    Tick,
    Reset,
    Stop,
    Start,
    Exit,
}

/// State for the timer component
#[derive(Debug, Clone, Default)]
struct TimerState {
    seconds: u32,
    running: bool,
}

/// Timer component that demonstrates effects
#[derive(Component, Clone, Default)]
struct TimerComponent;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl TimerComponent {
    #[update]
    fn update(&self, _ctx: &Context, msg: TimerMsg, mut state: TimerState) -> Action {
        match msg {
            TimerMsg::Tick => {
                if state.running {
                    state.seconds += 1;
                }
                Action::update(state)
            }
            TimerMsg::Reset => {
                state.seconds = 0;
                Action::update(state)
            }
            TimerMsg::Stop => {
                state.running = false;
                Action::update(state)
            }
            TimerMsg::Start => {
                state.running = true;
                Action::update(state)
            }
            TimerMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: TimerState) -> Node {
        let minutes = state.seconds / 60;
        let seconds = state.seconds % 60;
        let time_str = format!("{:02}:{:02}", minutes, seconds);

        let status_color = if state.running {
            Color::Green
        } else {
            Color::Red
        };
        let status_text = if state.running { "Running" } else { "Stopped" };

        node! {
            div(bg: black, dir: vertical, pad: 2, w: 60, h: 20) [
                // Title
                div(bg: "#1a1a1a", pad: 1, w_pct: 1.0, h: 3) [
                    text("⏱️  Timer with Effects", color: bright_white, bold),
                    spacer(1),
                    text("(Press q to quit)", color: "#666666")
                ],

                spacer(1),

                // Timer display
                div(bg: "#2a2a2a", pad: 2, w_pct: 1.0, h: 7, border: white) [
                    div(dir: vertical) [
                        text(&time_str, color: bright_cyan, bold),
                        spacer(1),
                        div(dir: horizontal) [
                            text("Status: ", color: white),
                            text(status_text, color: status_color, bold)
                        ]
                    ]
                ],

                spacer(1),

                // Controls
                div(dir: horizontal, pad: 1, w_pct: 1.0) [
                    div(bg: green, pad: 1, w: 12, h: 3, border: green) [
                        text("▶ Start", color: white, bold),
                        @click: ctx.handler(TimerMsg::Start),
                        @key(Char('s')): ctx.handler(TimerMsg::Start)
                    ],
                    spacer(1),
                    div(bg: red, pad: 1, w: 12, h: 3, border: red) [
                        text("■ Stop", color: white, bold),
                        @click: ctx.handler(TimerMsg::Stop),
                        @key(Char('x')): ctx.handler(TimerMsg::Stop)
                    ],
                    spacer(1),
                    div(bg: blue, pad: 1, w: 12, h: 3, border: blue) [
                        text("↺ Reset", color: white, bold),
                        @click: ctx.handler(TimerMsg::Reset),
                        @key(Char('r')): ctx.handler(TimerMsg::Reset)
                    ]
                ],

                spacer(1),

                // Instructions
                div(bg: "#1a1a1a", pad: 1, w_pct: 1.0) [
                    text("Keyboard shortcuts:", color: "#888888"),
                    text("  s - Start  |  x - Stop  |  r - Reset  |  q - Quit", color: "#666666")
                ],

                // Exit handler
                @key(Char('q')): ctx.handler(TimerMsg::Exit),
                @key(Esc): ctx.handler(TimerMsg::Exit)
            ]
        }
    }

    /// Define effects for this component
    /// This timer effect runs continuously and sends tick messages
    #[cfg(feature = "effects")]
    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        vec![
            // Timer effect that ticks every second
            Box::pin({
                let ctx = ctx.clone();
                async move {
                    loop {
                        // Sleep for 1 second
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                        // Send tick message
                        ctx.send(TimerMsg::Tick);
                    }
                }
            }),
        ]
    }
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    // Check if effects feature is enabled
    #[cfg(not(feature = "effects"))]
    {
        eprintln!("This example requires the 'effects' feature to be enabled.");
        eprintln!("Run with: cargo run --example effects_timer --features effects");
        return Ok(());
    }

    // Create and run the app
    let mut app = App::new()?;
    app.run(TimerComponent)?;
    Ok(())
}
