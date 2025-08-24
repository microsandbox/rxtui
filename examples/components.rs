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

#[derive(Component)]
struct Dashboard;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

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
                border: white,
                pad_h: 1,
                w: 25,
                dir: vertical,
                focusable,
                focus_style: (Style::default().background(self.color)),
                @key(down): ctx.handler(CounterMsg::Decrement),
                @key(up): ctx.handler(CounterMsg::Increment)
            ) [
                text(&self.label, color: white),
                text(format!("Count: {}", state.count), color: bright_white),
                spacer(1),
                hstack(gap: 2) [
                    div(bg: "#8b0a0a", pad_h: 2, @click: ctx.handler(CounterMsg::Decrement)) [
                        text("-", color: white)
                    ],
                    div(bg: "#0a29a4", pad_h: 2, @click: ctx.handler(CounterMsg::Increment)) [
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
    fn update(&self, ctx: &Context, msg: DashboardMsg) -> Action {
        match msg {
            DashboardMsg::ResetAll => {
                ctx.send_to_topic("counter_r", ResetSignal);
                ctx.send_to_topic("counter_g", ResetSignal);
                ctx.send_to_topic("counter_b", ResetSignal);
                Action::none()
            }
            DashboardMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(pad: 2, dir: vertical, @char_global('q'): ctx.handler(DashboardMsg::Exit), @key_global(esc): ctx.handler(DashboardMsg::Exit)) [
                spacer(1),

                text(
                    "Use Tab to focus counters. Press ↑/↓ to change values. Click buttons also work.",
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
                    pad_h: 1,
                    focusable,
                    focus_style: (Style::default().background(Color::Magenta)),
                    @click: ctx.handler(DashboardMsg::ResetAll),
                    @char('r'): ctx.handler(DashboardMsg::ResetAll)
                ) [
                    text("Reset All (R)", color: white)
                ]
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
