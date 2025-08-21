use termtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types: Counter Component
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

//--------------------------------------------------------------------------------------------------
// Types: Topic Messages
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct ResetSignal;

#[derive(Component, Clone)]
struct Counter {
    id: Option<ComponentId>,
    topic_name: String,
    label: String,
    color: Color,
}

//--------------------------------------------------------------------------------------------------
// Types: Dashboard Component
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum DashboardMsg {
    ResetAll,
    Exit,
}

#[derive(Debug, Clone)]
struct DashboardState {
    title: String,
}

#[derive(Component, Clone)]
struct Dashboard {
    id: Option<ComponentId>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            title: "RGB Counter Dashboard".to_string(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: Counter
//--------------------------------------------------------------------------------------------------

impl Counter {
    fn new(topic: impl Into<String>, label: impl Into<String>, color: Color) -> Self {
        Self {
            id: None,
            topic_name: topic.into(),
            label: label.into(),
            color,
        }
    }

    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        if let Some(topic) = topic {
            if topic == self.topic_name && msg.downcast::<ResetSignal>().is_some() {
                return Action::Update(Box::new(CounterState::default()));
            }

            return Action::None;
        }

        if let Some(msg) = msg.downcast::<CounterMsg>() {
            let mut state = ctx.get_state::<CounterState>();

            match msg {
                CounterMsg::Increment => state.count += 1,
                CounterMsg::Decrement => state.count -= 1,
            }

            return Action::Update(Box::new(state));
        }

        Action::None
    }

    fn view(&self, ctx: &Context) -> Node {
        let state = ctx.get_state::<CounterState>();

        tui! {
            div(
                bg: black,
                border: white,
                pad: 1,
                w: 25,
                dir: vertical,
                focusable,
                focus_style: ({
                    Style::default()
                        .background(self.color)
                        .border(self.color)
                        .padding(Spacing::all(1))
                })
            ) [
                text(&self.label, color: white),
                text(format!("Count: {}", state.count), color: bright_white),

                hstack(gap: 2) [
                    div(bg: black, border: white, pad_h: 1) [
                        text("-", color: white),
                        @click: ctx.handler(CounterMsg::Decrement)
                    ],
                    div(bg: black, border: white, pad_h: 1) [
                        text("+", color: white),
                        @click: ctx.handler(CounterMsg::Increment)
                    ]
                ],

                @key(Char('-')): ctx.handler(CounterMsg::Decrement),
                @key(Char('+')): ctx.handler(CounterMsg::Increment),
                @key(Down): ctx.handler(CounterMsg::Decrement),
                @key(Up): ctx.handler(CounterMsg::Increment)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: Dashboard
//--------------------------------------------------------------------------------------------------

impl Dashboard {
    fn new() -> Self {
        Self { id: None }
    }

    fn update(&self, ctx: &Context, msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        if let Some(msg) = msg.downcast::<DashboardMsg>() {
            match msg {
                DashboardMsg::ResetAll => {
                    ctx.send_to_topic("counter_r", Box::new(ResetSignal));
                    ctx.send_to_topic("counter_g", Box::new(ResetSignal));
                    ctx.send_to_topic("counter_b", Box::new(ResetSignal));

                    let mut state = ctx.get_state::<DashboardState>();
                    state.title = "RGB Counter Dashboard (Reset!)".to_string();
                    return Action::Update(Box::new(state));
                }
                DashboardMsg::Exit => {
                    return Action::Exit;
                }
            }
        }
        Action::None
    }

    fn view(&self, ctx: &Context) -> Node {
        let state = ctx.get_state::<DashboardState>();

        tui! {
            div(bg: black, pad: 2, dir: vertical) [
                div(bg: blue, pad: 1, w_pct: 1.0) [
                    text(&state.title, color: bright_white)
                ],

                spacer(1),

                text(
                    "Use Tab to focus counters. Press +/- or ↑/↓ to change values. Click buttons also work.",
                    color: bright_yellow
                ),
                text("Press 'r' to reset all counters, 'q' to quit.", color: bright_cyan),

                spacer(1),

                hstack(gap: 2) [
                    node(Counter::new("counter_r", "Counter R", Color::Red)),
                    node(Counter::new("counter_g", "Counter G", Color::Green)),
                    node(Counter::new("counter_b", "Counter B", Color::Blue))
                ],

                spacer(1),

                div(
                    bg: black,
                    border: white,
                    pad: 1,
                    w: 20,
                    focusable,
                    focus_style: ({
                        Style::default()
                            .background(Color::Yellow)
                            .border(Color::Yellow)
                            .padding(Spacing::all(1))
                    })
                ) [
                    text("Reset All (R)", color: white),
                    @click: ctx.handler(DashboardMsg::ResetAll),
                    @char('r'): ctx.handler(DashboardMsg::ResetAll)
                ],

                @char_global('q'): ctx.handler(DashboardMsg::Exit),
                @key_global(Esc): ctx.handler(DashboardMsg::Exit)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    let root = Dashboard::new();
    app.run(root)
}
