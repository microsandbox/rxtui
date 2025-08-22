use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Messages for the color demo component
#[derive(Debug, Clone)]
enum ColorDemoMsg {
    ClickLeft,
    ClickMiddle,
    ClickRight,
    Exit,
}

/// State for the color demo component
#[derive(Debug, Clone)]
struct ColorDemoState {
    left_color: Color,
    middle_color: Color,
    right_color: Color,
}

/// Color demo component showing interactive color cycling
#[derive(Component, Clone, Default)]
struct ColorDemo {}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ColorDemoState {
    fn default() -> Self {
        Self {
            left_color: Color::Red,
            middle_color: Color::Green,
            right_color: Color::Yellow,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ColorDemo {
    #[update]
    fn update(
        &self,
        context: &Context,
        message: ColorDemoMsg,
        mut colors: ColorDemoState,
    ) -> Action {
        match message {
            ColorDemoMsg::ClickLeft => {
                colors.left_color = next_color(colors.left_color);
                Action::update(colors)
            }
            ColorDemoMsg::ClickMiddle => {
                colors.middle_color = next_color(colors.middle_color);
                Action::update(colors)
            }
            ColorDemoMsg::ClickRight => {
                colors.right_color = next_color(colors.right_color);
                Action::update(colors)
            }
            ColorDemoMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: ColorDemoState) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 1) [
                // Title bar
                hstack(bg: "#333333", pad: 1, w: 80, h: 3) [
                    text("Radical TUI - Color Demo", color: "#00DDFF"),
                    div(w: 10) [],
                    text("(Press 'q' or ESC to quit)", color: "#FFD700")
                ],

                // Main content area with three clickable boxes
                hstack(bg: blue, pad: 1, w: 80, h: 10) [
                    // Left box
                    div(bg: (state.left_color), w: 20, h: 8) [
                        text("Click me!", color: white),
                        @click: ctx.handler(ColorDemoMsg::ClickLeft)
                    ],

                    // Middle box
                    div(bg: (state.middle_color), w: 20, h: 8) [
                        text("Click me!", color: black),
                        @click: ctx.handler(ColorDemoMsg::ClickMiddle)
                    ],

                    // Right box
                    div(bg: (state.right_color), w: 20, h: 8) [
                        text("Click me!", color: bright_blue),
                        @click: ctx.handler(ColorDemoMsg::ClickRight)
                    ]
                ],

                // Global event handlers
                @char_global('q'): ctx.handler(ColorDemoMsg::Exit),
                @key_global(Esc): ctx.handler(ColorDemoMsg::Exit)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    let root = ColorDemo::default();
    app.run(root)
}

fn next_color(color: Color) -> Color {
    match color {
        Color::Red => Color::Green,
        Color::Green => Color::Yellow,
        Color::Yellow => Color::Magenta,
        Color::Magenta => Color::Cyan,
        Color::Cyan => Color::Red,
        _ => Color::Red,
    }
}
