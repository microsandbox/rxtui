use rxtui::prelude::*;

#[derive(Component)]
struct RxTuiLogo;

impl RxTuiLogo {
    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(bg: black, w_pct: 1.0, h_pct: 1.0, dir: vertical, pad: 2, gap: 1) [
                vstack(bg: black) [
                    text("██████    ██    ██  ██████████  ██    ██  ██", color: "#8B4FB3"),
                    text("██████    ██    ██  ██████████  ██    ██  ██", color: "#B06FA8"),
                    text("██████    ██    ██  ██████████  ██    ██  ██", color: "#D68F9E"),
                    text("██████    ██    ██  ██████████  ██    ██  ██", color: "#e9dada"),
                    text("██    ██      ██        ██      ██    ██  ██", color: "#e9dada"),
                    text("██████      ██          ██      ██    ██  ██", color: "#e9dada"),
                    text("██    ██  ██    ██      ██      ████████  ██", color: "#e9dada")
                ],

                text(format!("v{} \"appcypher\"", env!("CARGO_PKG_VERSION")), color: "#969696"),

                @char_global('q'): ctx.handler(()),
                @key_global(Esc): ctx.handler(())
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(RxTuiLogo)
}
