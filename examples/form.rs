use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    UsernameChanged(String),
    PasswordChanged(String),
    Submit,
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
            }
            Msg::PasswordChanged(value) => {
                state.password = value;
                state.submitted = false;
            }
            Msg::Submit => {
                state.submitted = !state.username.is_empty() && !state.password.is_empty();
            }
            Msg::Exit => return Action::exit(),
        }
        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: FormState) -> Node {
        node! {
            div(
                pad: 2,
                w_pct: 1.0,
                h_pct: 1.0,
                align: center,
                @key(esc): ctx.handler(Msg::Exit)
            ) [
                text(
                    "tab to navigate | enter to submit | esc to exit",
                    color: bright_black
                ),
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
                    focus_style: (Style::default().border(Color::hex("#ffffff"))),
                    @click: ctx.handler(Msg::Submit)
                ) [
                    hstack [
                        div(w_pct: 0.9, h: 1)[],
                        text("Submit", color: black, bold),
                    ]
                ],

                spacer(2),

                // Form state / submission acknowledgement
                div(
                    border: (if state.submitted { Color::Green } else { Color::White }),
                    bg: (if state.submitted { Color::hex("#001e00") } else { Color::Black }),
                    pad: 1,
                    w: 40,
                    align: (if state.submitted { AlignItems::Center } else { AlignItems::Start })
                ) [
                    (if state.submitted {
                        node! {
                            text("✓ Form Submitted!", color: green, bold)
                        }
                    } else {
                        node! {
                            div [
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
                            ]
                        }
                    })
                ]
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Form)
}
