use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Msg {
    InputChanged(String),
    InputSubmitted,
    PasswordChanged(String),
    PasswordSubmitted,
    SearchChanged(String),
    SearchSubmitted,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct TextInputTestState {
    input_value: String,
    input_submit_count: usize,
    password_value: String,
    password_submit_count: usize,
    search_value: String,
    search_history: Vec<String>,
    last_action: String,
}

#[derive(Component)]
struct TextInputTest;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl TextInputTest {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: TextInputTestState) -> Action {
        match msg {
            Msg::InputChanged(value) => {
                state.input_value = value;
                state.last_action = format!("Input changed to: '{}'", state.input_value);
            }
            Msg::InputSubmitted => {
                state.input_submit_count += 1;
                state.last_action =
                    format!("Input submitted! (count: {})", state.input_submit_count);
            }
            Msg::PasswordChanged(value) => {
                state.password_value = value;
                state.last_action =
                    format!("Password changed (length: {})", state.password_value.len());
            }
            Msg::PasswordSubmitted => {
                state.password_submit_count += 1;
                state.last_action = format!(
                    "Password submitted! (count: {})",
                    state.password_submit_count
                );
            }
            Msg::SearchChanged(value) => {
                state.search_value = value;
                state.last_action = format!("Search changed to: '{}'", state.search_value);
            }
            Msg::SearchSubmitted => {
                if !state.search_value.is_empty() {
                    state.search_history.push(state.search_value.clone());
                    if state.search_history.len() > 5 {
                        state.search_history.remove(0);
                    }
                    let submitted = state.search_value.clone();
                    state.search_value.clear();
                    state.last_action = format!("Search submitted: '{}'", submitted);
                } else {
                    state.last_action = "Search submitted but was empty".to_string();
                }
            }
            Msg::Exit => return Action::exit(),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: TextInputTestState) -> Node {
        let action_color = if state.last_action.contains("submitted") {
            Color::Green
        } else {
            Color::Cyan
        };
        // Build search history text

        node! {
            div(
                bg: black,
                pad: 2,
                w_pct: 1.0,
                h: 45,
                dir: vertical,
                @key(esc): ctx.handler(Msg::Exit)
            ) [
                // Title
                text("TextInput Component Test", color: bright_white, bold),
                text("Press ESC to exit | Press ENTER in any field to submit", color: bright_black),
                spacer(1),

                // Last action indicator
                text("Last Action:", color: yellow),
                text(
                    if state.last_action.is_empty() {
                        "  No action yet".to_string()
                    } else {
                        format!("  {}", state.last_action)
                    },
                    color: action_color
                ),
                spacer(2),

                // Basic input with submit counter
                text("1. Basic Input (tracks Enter presses):", color: cyan),
                input(
                    placeholder: "Type and press Enter...",
                    border: cyan,
                    w: 40,
                    focusable,
                    @change: ctx.handler_with_value(Msg::InputChanged),
                    @submit: ctx.handler(Msg::InputSubmitted)
                ),
                text(
                    format!("  Value: '{}' | Submit count: {}",
                        state.input_value,
                        state.input_submit_count
                    ),
                    color: bright_black
                ),
                spacer(2),

                // Password input with submit
                text("2. Password Input (masked, with submit):", color: magenta),
                input(
                    placeholder: "Enter password and press Enter...",
                    password,
                    border: magenta,
                    w: 40,
                    focusable,
                    @change: ctx.handler_with_value(Msg::PasswordChanged),
                    @submit: ctx.handler(Msg::PasswordSubmitted)
                ),
                text(
                    format!("  Length: {} chars | Submit count: {}",
                        state.password_value.len(),
                        state.password_submit_count
                    ),
                    color: bright_black
                ),
                spacer(2),

                // Search input that clears on submit
                text("3. Search Input (clears on Enter, keeps history):", color: green),
                input(
                    placeholder: "Search and press Enter...",
                    border: green,
                    w: 40,
                    focusable,
                    @change: ctx.handler_with_value(Msg::SearchChanged),
                    @submit: ctx.handler(Msg::SearchSubmitted)
                ),
                text(
                    format!("  Current: '{}'", state.search_value),
                    color: bright_black
                ),
                spacer(1),

                // Search history
                text("Search History (last 5):", color: yellow),
                text(
                    state.search_history.iter().enumerate()
                        .map(|(i, search)| format!("  {}. {}", i + 1, search))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    color: bright_black
                ),
                text(
                    if state.search_history.is_empty() {
                        "  No searches yet".to_string()
                    } else {
                        "".to_string()
                    },
                    color: bright_black
                ),
                spacer(2),

                // Instructions
                text("Keyboard Shortcuts:", color: white, bold),
                text("  • Enter: Submit the current field", color: green, bold),
                text("  • Ctrl+W / Alt+Backspace: Delete word backward", color: bright_black),
                text("  • Alt+D: Delete word forward", color: bright_black),
                text("  • Ctrl+U: Delete to line start", color: bright_black),
                text("  • Ctrl+K: Delete to line end", color: bright_black),
                text("  • Ctrl+A / Home: Move to start", color: bright_black),
                text("  • Ctrl+E / End: Move to end", color: bright_black),
                text("  • Alt+B: Move word backward", color: bright_black),
                text("  • Alt+F: Move word forward", color: bright_black)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;
    app.run(TextInputTest)?;
    Ok(())
}
