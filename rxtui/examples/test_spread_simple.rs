use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    AddItem,
    Clear,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct State {
    items: Vec<String>,
    counter: usize,
}

#[derive(Component)]
struct ListExample;

impl ListExample {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: State) -> Action {
        match msg {
            Msg::AddItem => {
                state.counter += 1;
                state.items.push(format!("Item {}", state.counter));
                Action::update(state)
            }
            Msg::Clear => {
                state.items.clear();
                state.counter = 0;
                Action::update(state)
            }
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: State) -> Node {
        // Create nodes for each item
        let item_nodes: Vec<Node> = state
            .items
            .iter()
            .map(|item| {
                node! {
                    div(pad: 1, border: white) [
                        text(item)
                    ]
                }
            })
            .collect();

        node! {
            div(bg: black, pad: 2) [
                // Title
                text("List Example - Testing Spread & Expression Syntax", bold, color: cyan),
                spacer(1),

                // Status using expression
                (if state.items.is_empty() {
                    node! {
                        text("No items yet. Press 'a' to add!", color: yellow)
                    }
                } else {
                    node! {
                        text(format!("Items: {}", state.items.len()), color: green)
                    }
                }),

                spacer(1),

                // List container
                div(
                    h: 10,
                    border: cyan,
                    overflow: scroll,
                    show_scrollbar: true,
                    focusable
                ) [
                    // Use spread to expand all items
                    ...(item_nodes)
                ],

                spacer(1),

                // Controls using spread
                ...(vec![
                    node! { text("Controls:", bold) },
                    node! { text("  a - Add item") },
                    node! { text("  c - Clear all") },
                    node! { text("  Esc - Exit") },
                ]),

                // Event handlers
                @char_global('a'): ctx.handler(Msg::AddItem),
                @char_global('c'): ctx.handler(Msg::Clear),
                @key_global(Esc): ctx.handler(Msg::Exit),
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(ListExample)
}
