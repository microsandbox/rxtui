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

## Documentation

For comprehensive documentation including core concepts, component architecture, and advanced features, see [DOCS.md](DOCS.md).

- **[Tutorial](TUTORIAL.md)** - Step-by-step guide from basics to advanced features
- **[API Reference](API_REFERENCE.md)** - Complete API documentation
- **[Quick Reference](QUICK_REFERENCE.md)** - Handy cheat sheet for common patterns

## Getting Started

Add RxTUI to your `Cargo.toml`:

```toml
[dependencies]
rxtui = "0.1"
tokio = { version = "1", features = ["full"] }  # Required for async effects
```

The `effects` feature is enabled by default. If you want to disable it (for smaller binary size when not using async):

```toml
[dependencies]
rxtui = { version = "0.1", default-features = false }
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
