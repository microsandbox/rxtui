use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
    Clear,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct FormState {
    username: String,
    password: String,
    submitted: bool,
}

#[derive(Component)]
struct Form;

impl Form {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: FormState) -> Action {
        match msg {
            Msg::UsernameChanged(value) => {
                state.username = value;
                state.submitted = false;
                Action::update(state)
            }
            Msg::PasswordChanged(value) => {
                state.password = value;
                state.submitted = false;
                Action::update(state)
            }
            Msg::Submit => {
                if !state.username.is_empty() && !state.password.is_empty() {
                    state.submitted = true;
                }
                Action::update(state)
            }
            Msg::Clear => Action::update(FormState::default()),
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: FormState) -> Node {
        node! {
            div(align: center, pad: 2, w_pct: 1.0, @key(esc): ctx.handler(Msg::Exit), @char('c'): ctx.handler(Msg::Clear), @char('C'): ctx.handler(Msg::Clear)) [
                text("tab to navigate | enter to submit | esc to exit", color: bright_black),
                spacer(1),

                // Form fields with callbacks
                vstack [
                    text("Username:", color: white, bold),
                    input(
                        placeholder: "Enter your username...",
                        border: (if state.username.is_empty() { Color::White } else { Color::Green }),
                        focusable,
                        w: 40,
                        @change: ctx.handler_with_value(Msg::UsernameChanged),
                        @submit: ctx.handler(Msg::Submit)
                    )
                ],
                spacer(1),

                vstack [
                    text("Password:", color: white, bold),
                    input(
                        placeholder: "Enter secure password...",
                        password,
                        border: (if state.password.is_empty() { Color::White } else { Color::Green }),
                        focusable,
                        w: 40,
                        @change: ctx.handler_with_value(Msg::PasswordChanged),
                        @submit: ctx.handler(Msg::Submit)
                    )
                ],
                spacer(1),

                // Buttons
                div(
                    bg: (if state.username.is_empty() || state.password.is_empty() {
                        Color::White
                    } else {
                        Color::Green
                    }),
                    w: 40,
                    border: white,
                    focusable,
                    @click: ctx.handler(Msg::Submit)
                ) [
                    hstack [
                        div(w_pct: 0.9, h: 1)[],
                        text("Submit", color: black, bold),
                    ]
                ],

                spacer(2),

                // Live state display
                div(border: white, pad: 1, w: 40) [
                    text("Current Form State:", color: yellow, bold),
                    spacer(1),
                    richtext [
                        text("Username: ", color: cyan),
                        text(&state.username, color: white)
                    ],
                    richtext [
                        text("Password: ", color: cyan),
                        text(&"•".repeat(state.password.len()), color: white)
                    ]
                ],
                spacer(1),

                // Display submission status
                (if state.submitted {
                    node! {
                        div(border: green, pad: 2, bg: "#001e00", w: 40, align: center) [
                            text("✓ Form submitted successfully!", color: green, bold),
                            spacer(1),
                            text("All fields have been validated and processed.", color: white, wrap: word),
                        ]
                    }
                } else {
                    node! { spacer(0) }
                })
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Form)
}
