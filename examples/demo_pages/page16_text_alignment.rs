use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
enum AlignmentMode {
    Left,
    #[default]
    Center,
    Right,
}

#[derive(Component)]
pub struct Page16TextAlignmentDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page16TextAlignmentDemo {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str, mut state: AlignmentMode) -> Action {
        match msg {
            "left" => Action::update(AlignmentMode::Left),
            "center" => Action::update(AlignmentMode::Center),
            "right" => Action::update(AlignmentMode::Right),
            _ => Action::update(state),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: AlignmentMode) -> Node {
        let current_align = match state {
            AlignmentMode::Left => TextAlign::Left,
            AlignmentMode::Center => TextAlign::Center,
            AlignmentMode::Right => TextAlign::Right,
        };

        node! {
            div(
                bg: "#1a1a1a",
                pad: 2,
                @char_global('a'): ctx.handler("left"),
                @char_global('s'): ctx.handler("center"),
                @char_global('d'): ctx.handler("right")
            ) [
                // Title
                text("Text Alignment Demo", color: cyan, bold, align: center),
                text(format!("Current Mode: {:?}", state), color: yellow, align: center),

                text("", color: white),

                // Simple text alignment
                div(border: white, pad: 1, w: 50) [
                    text("Short", color: white, align: current_align),
                    text("Medium length text", color: green, align: current_align),
                    text("This is a longer line to show alignment", color: cyan, align: current_align)
                ],

                text("", color: white),

                // Rich text alignment
                div(border: white, pad: 1, w: 50) [
                    richtext(align: current_align) [
                        text("Rich ", color: red, bold),
                        text("Text ", color: yellow),
                        text("Example", color: green)
                    ]
                ],

                text("", color: white),

                // Wrapped text with alignment
                div(border: white, pad: 1, w: 50) [
                    text(
                        "This is a long piece of text that will wrap to multiple lines. Each line will be aligned according to the current alignment setting.",
                        color: white,
                        wrap: word,
                        align: current_align
                    )
                ],

                text("", color: white),
                text("[a] Left  [s] Center  [d] Right", color: bright_black, align: center)
            ]
        }
    }
}
