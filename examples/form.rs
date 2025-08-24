use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    NameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    Submit,
    Clear,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct FormState {
    name: String,
    email: String,
    password: String,
    submitted: bool,
}

#[derive(Component)]
struct Form;

impl Form {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: FormState) -> Action {
        match msg {
            Msg::NameChanged(value) => {
                state.name = value;
                state.submitted = false;
                Action::update(state)
            }
            Msg::EmailChanged(value) => {
                state.email = value;
                state.submitted = false;
                Action::update(state)
            }
            Msg::PasswordChanged(value) => {
                state.password = value;
                state.submitted = false;
                Action::update(state)
            }
            Msg::Submit => {
                if !state.name.is_empty() && !state.email.is_empty() && !state.password.is_empty() {
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
            div(bg: black, pad: 2, w_pct: 1.0, @key(Esc): ctx.handler(Msg::Exit), @char('c'): ctx.handler(Msg::Clear), @char('C'): ctx.handler(Msg::Clear)) [
                // Title
                text("Form Example with Callbacks", color: cyan, bold),
                spacer(1),

                text("tab to navigate | enter to submit | esc to exit", color: bright_black),
                spacer(2),

                // Form fields with callbacks
                vstack [
                    text(" Name:", color: white, bold),
                    input(
                        placeholder: "Enter your full name...",
                        border: (if state.name.is_empty() { Color::White } else { Color::Green }),
                        focusable,
                        w: 40,
                        @change: ctx.handler_with_value(Msg::NameChanged),
                        @submit: ctx.handler(Msg::Submit)
                    )
                ],
                spacer(1),

                vstack [
                    text(" Email:", color: white, bold),
                    input(
                        placeholder: "your.email@example.com",
                        border: (if state.email.is_empty() { Color::White } else { Color::Green }),
                        focusable,
                        w: 40,
                        @change: ctx.handler_with_value(Msg::EmailChanged),
                        @submit: ctx.handler(Msg::Submit)
                    )
                ],
                spacer(1),

                vstack [
                    text(" Password:", color: white, bold),
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
                    bg: (if state.name.is_empty() || state.email.is_empty() || state.password.is_empty() {
                        Color::BrightBlack
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
                        text("Name: ", color: cyan),
                        text(&state.name, color: white)
                    ],
                    richtext [
                        text("Email: ", color: cyan),
                        text(&state.email, color: white)
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
                        div(border: green, pad: 2, bg: "#001e00", w: 40) [
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
