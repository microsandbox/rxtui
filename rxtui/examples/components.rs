use rxtui::prelude::*;

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

#[derive(Component)]
struct Counter {
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

#[derive(Component)]
struct Dashboard;

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
            topic_name: topic.into(),
            label: label.into(),
            color,
        }
    }

    /// Using the new #[update] macro with dynamic topic support!
    #[update(msg = CounterMsg, topics = [self.topic_name => ResetSignal])]
    fn update(&self, _ctx: &Context, messages: Messages, mut state: CounterState) -> Action {
        match messages {
            Messages::CounterMsg(msg) => {
                match msg {
                    CounterMsg::Increment => state.count += 1,
                    CounterMsg::Decrement => state.count -= 1,
                }
                Action::update(state)
            }
            Messages::ResetSignal(_) => Action::update(CounterState::default()),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        node! {
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
                }),
                @key(Char('-')): ctx.handler(CounterMsg::Decrement),
                @key(Char('+')): ctx.handler(CounterMsg::Increment),
                @key(down): ctx.handler(CounterMsg::Decrement),
                @key(up): ctx.handler(CounterMsg::Increment)
            ) [
                text(&self.label, color: white),
                text(format!("Count: {}", state.count), color: bright_white),

                hstack(gap: 2) [
                    div(bg: black, border: white, pad_h: 1, @click: ctx.handler(CounterMsg::Decrement)) [
                        text("-", color: white)
                    ],
                    div(bg: black, border: white, pad_h: 1, @click: ctx.handler(CounterMsg::Increment)) [
                        text("+", color: white)
                    ]
                ]
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods: Dashboard
//--------------------------------------------------------------------------------------------------

impl Dashboard {
    #[update]
    fn update(&self, ctx: &Context, msg: DashboardMsg, mut state: DashboardState) -> Action {
        match msg {
            DashboardMsg::ResetAll => {
                ctx.send_to_topic("counter_r", ResetSignal);
                ctx.send_to_topic("counter_g", ResetSignal);
                ctx.send_to_topic("counter_b", ResetSignal);

                state.title = "RGB Counter Dashboard (Reset!)".to_string();
                Action::update(state)
            }
            DashboardMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: DashboardState) -> Node {
        node! {
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
                    @click: ctx.handler(DashboardMsg::ResetAll),
                    @char('r'): ctx.handler(DashboardMsg::ResetAll)
                ) [
                    text("Reset All (R)", color: white)
                ],

                @char_global('q'): ctx.handler(DashboardMsg::Exit),
                @key_global(esc): ctx.handler(DashboardMsg::Exit)
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    App::new()?.run(Dashboard)
}
