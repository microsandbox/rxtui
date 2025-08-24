# RxTUI

A reactive terminal UI framework for Rust that makes building TUIs as easy as writing a web app.

```rust
node! {
    div(bg: blue, pad: 2) [
        text("Hello, Terminal!", color: white, bold),
        text("Press any key...", color: gray)
    ]
}
```

## Why RxTUI?

Terminal UIs have traditionally been painful to build. You either work with low-level escape sequences (error-prone and tedious) or use immediate-mode libraries that require you to manage all state manually. RxTUI takes a different approach.

We bring the retained-mode, component-based architecture that revolutionized web development to the terminal. This means:

- **Declarative UI** - Describe what your UI should look like, not how to change it. The framework handles all the transitions
- **Zero manual optimization** - Automatic diffing, dirty tracking, and minimal redraws. Your app stays fast without effort
- **True composability** - Build complex apps from simple, reusable components that encapsulate their own logic
- **Proven patterns** - Combines the best of React's components with Elm's message-based architecture, perfectly adapted for terminals

## Quick Example

Here's a complete counter app in 30 lines:

```rust
use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    Increment,
    Decrement,
    Exit,
}

#[derive(Component)]
struct Counter;

impl Counter {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut count: i32) -> Action {
        match msg {
            Msg::Increment => Action::update(count + 1),
            Msg::Decrement => Action::update(count - 1),
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, count: i32) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Count: {}", count), color: white, bold),
                text("Use +/- to change, Esc to exit", color: gray),

                @char_global('+'): ctx.handler(Msg::Increment),
                @char_global('-'): ctx.handler(Msg::Decrement),
                @key_global(Esc): ctx.handler(Msg::Exit)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Counter)
}
```

That's it. No manual rendering, no state management boilerplate, no event loop. Just your logic.

## Core Concepts

### 1. Components

Everything is a component. Think of them as self-contained UI pieces that know how to manage their own state and behavior. Each component has two main jobs: handling events (through `update`) and rendering UI (through `view`):

```rust
#[derive(Component)]
struct TodoList;

impl TodoList {
    #[update]
    fn update(&self, ctx: &Context, msg: TodoMsg, mut state: TodoState) -> Action {
        // Messages come here from events in your view
        // You update state, then return Action::update(state) to re-render
    }

    #[view]
    fn view(&self, ctx: &Context, state: TodoState) -> Node {
        // This renders your UI using the current state
        // Uses the node! macro to build the UI tree
    }
}
```

### 2. The node! Macro

This is how you actually build your UI. The `node!` macro gives you a clean, declarative syntax that lives inside your component's `view` method. Instead of imperatively creating and configuring widgets, you describe what the UI should look like:

```rust
#[view]
fn view(&self, ctx: &Context, state: AppState) -> Node {
    node! {
        div(bg: blue, pad: 2, border: white) [
            text(format!("Count: {}", state.count), color: yellow),

            hstack(gap: 2) [
                text("Click me!"),
                // Events here trigger messages that go to update()
                @click: ctx.handler(Msg::Increment),
            ],

            @key_global(Esc): ctx.handler(Msg::Exit)
        ]
    }
}
```

### 3. Messages & State

These are the heart of your component's logic. State is just your data - what your component needs to remember. Messages are the things that can happen - user clicks, key presses, timers firing. When a message arrives, you update your state, and the UI automatically re-renders:

```rust
// Your state - the data your component needs
#[derive(Debug, Clone, Default)]
struct TodoState {
    items: Vec<String>,
    selected: usize,
}

// Messages - what can happen in your component
#[derive(Debug, Clone)]
enum TodoMsg {
    AddItem(String),
    RemoveItem(usize),
    SelectItem(usize),
}

// In update(), messages modify state
#[update]
fn update(&self, ctx: &Context, msg: TodoMsg, mut state: TodoState) -> Action {
    match msg {
        TodoMsg::AddItem(text) => {
            state.items.push(text);
            Action::update(state)  // This triggers view() to re-render
        }
        // ... handle other messages
    }
}
```

The flow: **Event** (click) → **Message** (AddItem) → **Update** (modify state) → **View** (re-render with new state)

### 4. Layout System

Terminal sizes vary wildly - from tiny SSH windows to full-screen terminal apps. RxTUI's layout system adapts automatically. Use percentages for responsive design, fixed sizes for specific elements, and absolute positioning for overlays. All of this happens right in your `node!` macro:

```rust
node! {
    // Percentage-based for responsive design
    div(w_pct: 0.5, h_pct: 0.8) [
        // This takes 50% width, 80% height of parent

        // Direction-based layouts
        div(dir: horizontal, gap: 2) [
            text("Left"),
            text("Right")
        ],

        // Absolute positioning for overlays
        div(absolute, top: 5, right: 5, z: 100) [
            text("Floating notification")
        ]
    ]
}
```

### 5. Cross-Component Communication

Sometimes components need to talk to each other - a sidebar needs to tell the main content what to display, or a notification system needs to listen for alerts from anywhere in the app. Topics make this easy without tight coupling:

```rust
// Send a message to a topic
ctx.send_to_topic("notifications", Alert::new("Hello!"));

// Listen to topics
#[update(msg = MyMsg, topics = ["notifications" => Alert])]
fn update(&self, ctx: &Context, messages: Messages, state: MyState) -> Action {
    // Handle both regular messages and topic messages
}
```

## Getting Started

Add RxTUI to your `Cargo.toml`:

```toml
[dependencies]
rxtui = "0.1"
```

If you need async effects (timers, network requests, streams), add tokio:

```toml
[dependencies]
rxtui = "0.1"
tokio = { version = "1", features = ["full"] }
```

Check out the examples:

- `simple` - Interactive color demo
- `timer` - Async timer with effects
- `components` - Multi-component communication
- `demo` - Full feature showcase

Run an example:

```bash
cargo run --example simple
```

## Features at a Glance

- **Virtual DOM** - Efficient diffing and patching
- **Component System** - Reusable, composable UI pieces
- **Reactive State** - Automatic UI updates
- **Event Handling** - Keyboard, mouse, and focus events
- **Async Effects** - Timers, network requests, streams
- **Rich Styling** - Colors, borders, padding, text styles
- **Layout Engine** - Flexible sizing and positioning
- **Topic Messaging** - Cross-component communication
- **Text Input** - Built-in input component with editing
- **Scrolling** - Automatic scrollbar and keyboard navigation
- **No Flicker** - Double-buffered rendering
- **Cross-Platform** - Windows, macOS, Linux

## Documentation

- **[Tutorial](TUTORIAL.md)** - Step-by-step guide to learning RxTUI
- **[Documentation](DOCS.md)** - Complete framework documentation
- **[API Reference](API_REFERENCE.md)** - Detailed API documentation
- **[Quick Reference](QUICK_REFERENCE.md)** - Handy cheat sheet

## Philosophy

RxTUI believes that terminal UIs shouldn't be harder than web UIs. We've taken the best ideas from modern frontend frameworks and adapted them for the terminal's unique constraints.

The result? A framework that's powerful enough for complex applications yet simple enough that you can build your first TUI in minutes.

## License

Apache 2.0 - Build amazing TUIs!
