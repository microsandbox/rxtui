use rxtui::prelude::*;
use rxtui::style::Direction;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
struct AlignmentState {
    text_align: TextAlign,
    justify: JustifyContent,
    align: AlignItems,
    vertical: bool,
    show_align_self: bool,
}

#[derive(Component)]
pub struct Page16TextAlignmentDemo;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page16TextAlignmentDemo {
    #[update]
    fn update(&self, _ctx: &Context, event: &str, mut state: AlignmentState) -> Action {
        match event {
            // Text alignment
            "text_left" => state.text_align = TextAlign::Left,
            "text_center" => state.text_align = TextAlign::Center,
            "text_right" => state.text_align = TextAlign::Right,
            // Justify content (using T,Y,U,I,O,P keys)
            "justify_start" => state.justify = JustifyContent::Start,
            "justify_center" => state.justify = JustifyContent::Center,
            "justify_end" => state.justify = JustifyContent::End,
            "justify_between" => state.justify = JustifyContent::SpaceBetween,
            "justify_around" => state.justify = JustifyContent::SpaceAround,
            "justify_evenly" => state.justify = JustifyContent::SpaceEvenly,
            // Align items (using Z,X,C keys)
            "align_start" => state.align = AlignItems::Start,
            "align_center" => state.align = AlignItems::Center,
            "align_end" => state.align = AlignItems::End,
            // Other controls
            "toggle_direction" => state.vertical = !state.vertical,
            "toggle_align_self" => state.show_align_self = !state.show_align_self,
            _ => return Action::update(state),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: AlignmentState) -> Node {
        let dir = if state.vertical {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };

        // Build div alignment demo children
        let div_children = if state.show_align_self {
            vec![
                node! { div(bg: red, w: 10, h: 3) [text("1", color: white)] },
                node! { div(bg: green, w: 12, h: 5, align_self: end) [text("2(end)", color: white)] },
                node! { div(bg: blue, w: 14, h: 7) [text("3", color: white)] },
                node! { div(bg: yellow, w: 10, h: 4, align_self: start) [text("4(start)", color: black)] },
            ]
        } else {
            vec![
                node! { div(bg: red, w: 10, h: 3) [text("1", color: white)] },
                node! { div(bg: green, w: 12, h: 5) [text("2", color: white)] },
                node! { div(bg: blue, w: 14, h: 7) [text("3", color: white)] },
            ]
        };

        node! {
            div(
                pad: 2,
                overflow: scroll,
                // Text alignment controls
                @char_global('a'): ctx.handler("text_left"),
                @char_global('s'): ctx.handler("text_center"),
                @char_global('d'): ctx.handler("text_right"),
                // Justify controls
                @char_global('t'): ctx.handler("justify_start"),
                @char_global('y'): ctx.handler("justify_center"),
                @char_global('u'): ctx.handler("justify_end"),
                @char_global('i'): ctx.handler("justify_between"),
                @char_global('o'): ctx.handler("justify_around"),
                @char_global('p'): ctx.handler("justify_evenly"),
                // Align controls
                @char_global('z'): ctx.handler("align_start"),
                @char_global('x'): ctx.handler("align_center"),
                @char_global('c'): ctx.handler("align_end"),
                // Other controls
                @char_global('v'): ctx.handler("toggle_direction"),
                @char_global('b'): ctx.handler("toggle_align_self")
            ) [
                // Title
                text("Text & Div Alignment Demo", color: cyan, bold, align: center),
                spacer(1),

                // TEXT ALIGNMENT SECTION
                text("=== TEXT ALIGNMENT ===", color: yellow, bold, align: center),
                text(format!("Current: {:?}", state.text_align), color: bright_black),
                spacer(1),

                // Simple text alignment
                div(border: white, pad: 1, w: 50) [
                    text("Short", color: white, align: (state.text_align)),
                    text("Medium length text", color: green, align: (state.text_align)),
                    text("This is a longer line to show alignment", color: cyan, align: (state.text_align))
                ],
                spacer(1),

                // Wrapped text with alignment
                div(border: white, pad: 1, w: 50) [
                    text(
                        "This is a long piece of text that will wrap to multiple lines. Each line will be aligned according to the current alignment setting.",
                        color: white,
                        wrap: word,
                        align: (state.text_align)
                    )
                ],
                spacer(2),

                // DIV ALIGNMENT SECTION
                text("=== DIV ALIGNMENT ===", color: yellow, bold, align: center),
                text(format!(
                    "Dir: {} | Justify: {:?} | Align: {:?} | AlignSelf: {}",
                    if state.vertical { "Vertical" } else { "Horizontal" },
                    state.justify,
                    state.align,
                    if state.show_align_self { "ON" } else { "OFF" }
                ), color: bright_black),
                spacer(1),

                // Div alignment demo container
                div(
                    bg: "#333",
                    border: white,
                    dir: (dir),
                    justify: (state.justify),
                    align: (state.align),
                    w: 50,
                    h: 15
                ) [...(div_children)],
                spacer(1),

                // Instructions
                div(border: bright_black, pad: 1, w: 50) [
                    text("TEXT:", color: yellow),
                    text("[A] Left  [S] Center  [D] Right", color: white),
                    spacer(1),
                    text("JUSTIFY (Main Axis):", color: yellow),
                    text("[T] Start  [Y] Center  [U] End", color: white),
                    text("[I] Between  [O] Around  [P] Evenly", color: white),
                    spacer(1),
                    text("ALIGN (Cross Axis):", color: yellow),
                    text("[Z] Start [X] Center [C] End", color: white),
                    spacer(1),
                    text("OTHER:", color: yellow),
                    text("[V] Toggle Direction  [B] Toggle AlignSelf", color: white)
                ]
            ]
        }
    }
}
