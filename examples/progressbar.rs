use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    SetProgress(f32),
    Exit,
}

#[derive(Debug, Clone)]
struct ProgressState {
    progress: f32, // 0.0 to 1.0
}

impl Default for ProgressState {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}

#[derive(Component)]
struct ProgressBar;

#[component]
impl ProgressBar {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: ProgressState) -> Action {
        match msg {
            Msg::SetProgress(value) => {
                state.progress = value;
                Action::update(state)
            }
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: ProgressState) -> Node {
        let percentage = (state.progress * 100.0) as u32;
        let bar_width = 50;
        let filled = ((state.progress * bar_width as f32) as usize).min(bar_width);
        let empty = bar_width - filled;

        node! {
            div(
                pad: 2,
                @key_global(esc): ctx.handler(Msg::Exit),
                @char_global('q'): ctx.handler(Msg::Exit)
            ) [
                hstack(gap: 2) [
                    // Progress bar with smooth gradient
                    hstack [
                        ...((0..filled).map(|i| {
                            // Calculate gradient from purple to cyan
                            let progress = i as f32 / bar_width as f32;

                            // Interpolate RGB values for smooth gradient
                            // Start: purple (100, 50, 200)
                            // End: cyan (50, 200, 200)
                            let r = (100.0 - (50.0 * progress)) as u8;
                            let g = (50.0 + (150.0 * progress)) as u8;
                            let b = 200;

                            node! {
                                text("█", color: (Color::Rgb(r, g, b)))
                            }
                        }).collect::<Vec<Node>>())
                    ],

                    text("·".repeat(empty), color: (Color::Rgb(50, 50, 50))),
                    text(format!("{:>3}%", percentage), color: white, bold)
                ],

                spacer(2),

                // Instructions
                text("Press Esc or q to exit", color: bright_black)
            ]
        }
    }

    #[effect]
    async fn animate_progress(&self, ctx: &Context) {
        // Continuously animate the progress bar
        loop {
            for i in 0..=100 {
                tokio::time::sleep(std::time::Duration::from_millis(30)).await;
                ctx.send(Msg::SetProgress(i as f32 / 100.0));
            }
            // Reset and loop
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(ProgressBar)
}
