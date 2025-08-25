use rxtui::prelude::*;

#[derive(Debug, Clone)]
pub struct DemoState {
    justify: JustifyContent,
    align: AlignItems,
    vertical: bool,
    show_align_self: bool,
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            justify: JustifyContent::Start,
            align: AlignItems::Start,
            vertical: false,
            show_align_self: false,
        }
    }
}

#[derive(Component)]
pub struct DivAlignmentDemo;

impl DivAlignmentDemo {
    #[update]
    fn update(&self, _ctx: &Context, event: &str, mut state: DemoState) -> Action {
        match event {
            // Justify content controls
            "justify_start" => {
                state.justify = JustifyContent::Start;
                Action::update(state)
            }
            "justify_center" => {
                state.justify = JustifyContent::Center;
                Action::update(state)
            }
            "justify_end" => {
                state.justify = JustifyContent::End;
                Action::update(state)
            }
            "justify_space_between" => {
                state.justify = JustifyContent::SpaceBetween;
                Action::update(state)
            }
            "justify_space_around" => {
                state.justify = JustifyContent::SpaceAround;
                Action::update(state)
            }
            "justify_space_evenly" => {
                state.justify = JustifyContent::SpaceEvenly;
                Action::update(state)
            }
            // Align items controls
            "align_start" => {
                state.align = AlignItems::Start;
                Action::update(state)
            }
            "align_center" => {
                state.align = AlignItems::Center;
                Action::update(state)
            }
            "align_end" => {
                state.align = AlignItems::End;
                Action::update(state)
            }
            "align_stretch" => {
                state.align = AlignItems::Stretch;
                Action::update(state)
            }
            // Other controls
            "toggle_direction" => {
                state.vertical = !state.vertical;
                Action::update(state)
            }
            "toggle_align_self" => {
                state.show_align_self = !state.show_align_self;
                Action::update(state)
            }
            "exit" => Action::exit(),
            _ => Action::none(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: DemoState) -> Node {
        use rxtui::style::Direction;

        let dir = if state.vertical {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };
        let justify = state.justify;
        let align = state.align;
        let is_stretch = matches!(align, AlignItems::Stretch);

        // Build children - always show varying sizes to demonstrate both axes
        let children = if state.show_align_self {
            // Show align_self overrides
            if state.vertical {
                vec![
                    node! { div(bg: red, w: 5, h: 4) [text("1", color: white)] },
                    node! { div(bg: green, w: 8, h: 5, align_self: end) [text("2(end)", color: white)] },
                    node! { div(bg: blue, w: 10, h: 6) [text("3", color: white)] },
                    node! { div(bg: yellow, w: 6, h: 4, align_self: start) [text("4(start)", color: black)] },
                ]
            } else {
                vec![
                    node! { div(bg: red, w: 8, h: 3) [text("1", color: white)] },
                    node! { div(bg: green, w: 10, h: 5, align_self: end) [text("2(end)", color: white)] },
                    node! { div(bg: blue, w: 12, h: 7) [text("3", color: white)] },
                    node! { div(bg: yellow, w: 8, h: 4, align_self: start) [text("4(start)", color: black)] },
                ]
            }
        } else if is_stretch {
            // Stretch mode - no cross axis dimension
            if state.vertical {
                vec![
                    node! { div(bg: red, h: 4) [text("1", color: white)] },
                    node! { div(bg: green, h: 5) [text("2", color: white)] },
                    node! { div(bg: blue, h: 6) [text("3", color: white)] },
                ]
            } else {
                vec![
                    node! { div(bg: red, w: 8) [text("1", color: white)] },
                    node! { div(bg: green, w: 10) [text("2", color: white)] },
                    node! { div(bg: blue, w: 12) [text("3", color: white)] },
                ]
            }
        } else {
            // Normal mode - varying sizes in both dimensions
            if state.vertical {
                vec![
                    node! { div(bg: red, w: 6, h: 4) [text("1", color: white)] },
                    node! { div(bg: green, w: 10, h: 5) [text("2", color: white)] },
                    node! { div(bg: blue, w: 14, h: 6) [text("3", color: white)] },
                ]
            } else {
                vec![
                    node! { div(bg: red, w: 10, h: 3) [text("1", color: white)] },
                    node! { div(bg: green, w: 12, h: 5) [text("2", color: white)] },
                    node! { div(bg: blue, w: 14, h: 7) [text("3", color: white)] },
                ]
            }
        };

        // Build the container
        let demo_container: Node = Div::new()
            .background(Color::hex("#333"))
            .border(BorderStyle::Single)
            .border_color(Color::White)
            .direction(dir)
            .justify_content(justify)
            .align_items(align)
            .width(66)
            .height(30)
            .children(children)
            .into();

        let container = Div::new()
            .background(Color::hex("#1a1a1a"))
            .padding(Spacing::all(2))
            .width_percent(1.0)
            .height_percent(1.0)
            .on_char_global('1', ctx.handler("justify_start"))
            .on_char_global('2', ctx.handler("justify_center"))
            .on_char_global('3', ctx.handler("justify_end"))
            .on_char_global('4', ctx.handler("justify_space_between"))
            .on_char_global('5', ctx.handler("justify_space_around"))
            .on_char_global('6', ctx.handler("justify_space_evenly"))
            .on_char_global('q', ctx.handler("align_start"))
            .on_char_global('w', ctx.handler("align_center"))
            .on_char_global('e', ctx.handler("align_end"))
            .on_char_global('r', ctx.handler("align_stretch"))
            .on_char_global('a', ctx.handler("toggle_align_self"))
            .on_char_global('d', ctx.handler("toggle_direction"))
            .on_key_global(Key::Esc, ctx.handler("exit"))
            .children(vec![
                // Title
                Text::new("Div Alignment Demo - Mix & Match!")
                    .color(Color::Yellow)
                    .bold()
                    .into(),
                Text::new(
                    "Justify and Align work on perpendicular axes and can be freely combined",
                )
                .color(Color::BrightBlack)
                .into(),
                Div::new().height(1).into(), // spacer
                // Current settings display
                Text::new(format!(
                    "Direction: {} | Justify: {:?} | Align: {:?} | AlignSelf: {}",
                    if state.vertical {
                        "Vertical"
                    } else {
                        "Horizontal"
                    },
                    state.justify,
                    state.align,
                    if state.show_align_self { "ON" } else { "OFF" }
                ))
                .color(Color::Cyan)
                .into(),
                Div::new().height(1).into(), // spacer
                // Demo container
                demo_container,
                Div::new().height(1).into(), // spacer
                // Instructions
                Div::new()
                    .border(BorderStyle::Single)
                    .border_color(Color::BrightBlack)
                    .padding(Spacing::all(1))
                    .children(vec![
                        Text::new("JustifyContent (Main Axis):")
                            .color(Color::Yellow)
                            .bold()
                            .into(),
                        Text::new(if state.vertical {
                            "  Controls vertical distribution"
                        } else {
                            "  Controls horizontal distribution"
                        })
                        .color(Color::BrightBlack)
                        .into(),
                        Text::new("  [1] Start        [2] Center       [3] End")
                            .color(Color::White)
                            .into(),
                        Text::new("  [4] SpaceBetween [5] SpaceAround  [6] SpaceEvenly")
                            .color(Color::White)
                            .into(),
                        Div::new().height(1).into(), // spacer
                        Text::new("AlignItems (Cross Axis):")
                            .color(Color::Yellow)
                            .bold()
                            .into(),
                        Text::new(if state.vertical {
                            "  Controls horizontal alignment"
                        } else {
                            "  Controls vertical alignment"
                        })
                        .color(Color::BrightBlack)
                        .into(),
                        Text::new("  [Q] Start        [W] Center       [E] End        [R] Stretch")
                            .color(Color::White)
                            .into(),
                        Div::new().height(1).into(), // spacer
                        Text::new("Other Controls:")
                            .color(Color::Yellow)
                            .bold()
                            .into(),
                        Text::new("  [D] Toggle Direction - Switch horizontal/vertical")
                            .color(Color::White)
                            .into(),
                        Text::new("  [A] Toggle AlignSelf - Show per-child overrides")
                            .color(Color::White)
                            .into(),
                        Text::new("  [ESC] Exit").color(Color::White).into(),
                    ])
                    .into(),
            ]);

        container.into()
    }
}

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(DivAlignmentDemo)
}
