use rxtui::prelude::*;

#[derive(Component)]
pub struct WrapAlignmentDemo;

impl WrapAlignmentDemo {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str, _state: ()) -> Action {
        match msg {
            "exit" => Action::exit(),
            _ => Action::none(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(
                bg: "#1a1a1a",
                pad: 2,
                w_pct: 1.0,
                h_pct: 1.0,
                @key_global(esc): ctx.handler("exit")
            ) [
                // Title
                text("Wrap + Alignment Demo", color: yellow, bold),
                text("Testing if wrapping and alignment work together", color: bright_black),
                spacer(1),

                // Test 1: Horizontal wrap with justify center
                text("Horizontal wrap + justify: center", color: cyan),
                div(
                    bg: "#333",
                    w: 40,
                    h: 15,
                    border: white,
                    dir: h,
                    wrap: wrap,
                    justify: center,
                    align: center,
                    gap: 1
                ) [
                    div(bg: red, w: 8, h: 3) [text("1", color: white)],
                    div(bg: green, w: 8, h: 3) [text("2", color: white)],
                    div(bg: blue, w: 8, h: 3) [text("3", color: white)],
                    // These should wrap to next row
                    div(bg: yellow, w: 8, h: 3) [text("4", color: white)],
                    div(bg: magenta, w: 8, h: 3) [text("5", color: white)],
                    div(bg: cyan, w: 8, h: 3) [text("6", color: white)]
                ],

                spacer(1),

                // Test 2: Horizontal wrap with space-between
                text("Horizontal wrap + justify: space-between", color: cyan),
                div(
                    bg: "#333",
                    w: 45,
                    h: 10,
                    border: white,
                    dir: h,
                    wrap: wrap,
                    justify: space_between,
                    align: end,
                    gap: 1
                ) [
                    div(bg: red, w: 10, h: 2) [text("Item 1", color: white)],
                    div(bg: green, w: 10, h: 3) [text("Item 2", color: white)],
                    div(bg: blue, w: 10, h: 2) [text("Item 3", color: white)],
                    div(bg: yellow, w: 10, h: 3) [text("Item 4", color: white)],
                    div(bg: magenta, w: 10, h: 2) [text("Item 5", color: white)]
                ],

                spacer(1),

                // Test 3: Vertical wrap with alignment
                text("Vertical wrap + align: center", color: cyan),
                div(
                    bg: "#333",
                    w: 30,
                    h: 10,
                    border: white,
                    dir: v,
                    wrap: wrap,
                    justify: center,
                    align: center,
                    gap: 1
                ) [
                    div(bg: red, w: 6, h: 2) [text("A", color: white)],
                    div(bg: green, w: 8, h: 2) [text("B", color: white)],
                    div(bg: blue, w: 6, h: 2) [text("C", color: white)],
                    div(bg: yellow, w: 8, h: 2) [text("D", color: white)],
                    div(bg: magenta, w: 6, h: 2) [text("E", color: white)]
                ],

                spacer(1),
                text("Press ESC to exit", color: bright_black)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(WrapAlignmentDemo)
}
