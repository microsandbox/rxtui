<div align="center">
  <h1>RxTUI</h1>
  <b>———&nbsp;&nbsp;&nbsp;reactive terminal UI framework for rust&nbsp;&nbsp;&nbsp;———</b>
</div>

<br />

<div align='center'>
  <a href="https://crates.io/crates/rxtui">
    <img src="https://img.shields.io/crates/v/rxtui?style=for-the-badge&logo=rust&logoColor=white" alt="crates.io version"/>
  </a>
  <a href="./LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue?style=for-the-badge" alt="license"/>
  </a>
  <a href="./DOCS.md">
    <img src="https://img.shields.io/badge/docs-comprehensive-%2300acee.svg?color=ff4500&style=for-the-badge&logo=gitbook&logoColor=white" alt="documentation"/>
  </a>
</div>

<br />

<div align='center'>• • •</div>

# <sub>WHY RXTUI?</sub>

Terminal UIs have traditionally been painful to build. You either work with low-level escape sequences (error-prone and tedious) or use immediate-mode libraries that require you to manage all state manually. **RxTUI** takes a different approach.

We bring the retained-mode, component-based architecture that revolutionized web development to the terminal:

- [x] **Declarative UI** - Describe what your UI should look like, not how to change it
- [x] **Zero Manual Optimization** - Automatic diffing, dirty tracking, and minimal redraws
- [x] **True Composability** - Build complex apps from simple, reusable components
- [x] **Proven Patterns** - Combines React's components with Elm's message architecture
- [x] **Async Effects** - Built-in support for timers, network requests, and streams
- [x] **Rich Ecosystem** - Comprehensive styling, layout, and input handling

<div align='center'>• • •</div>

# <sub>QUICK START</sub>

### <span>1</span>&nbsp;&nbsp;Install RxTUI

Add to your `Cargo.toml`:

```toml
[dependencies]
rxtui = "0.1"
tokio = { version = "1", features = ["full"] }  # For async effects
```

> [!NOTE]
> The `effects` feature is enabled by default. To disable it for smaller binary size:
> ```toml
> rxtui = { version = "0.1", default-features = false }
> ```

### <span>2</span>&nbsp;&nbsp;Create Your First App

Complete counter app in 30 lines:

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

### <span>3</span>&nbsp;&nbsp;Run Your App

```bash
cargo run
```

That's it! No manual rendering, no state management boilerplate, no event loop. Just your logic.

<div align='center'>• • •</div>

# <sub>FEATURES</sub>

<table>
<tr>
<td>

### Core Architecture
- [x] Virtual DOM with efficient diffing
- [x] Component-based architecture
- [x] Reactive state management
- [x] Message-based updates
- [x] Automatic re-rendering

</td>
<td>

### UI Components
- [x] Flexible layout system
- [x] Rich text styling
- [x] Built-in TextInput widget
- [x] Scrollable containers
- [x] Modal overlays

</td>
</tr>
<tr>
<td>

### Event Handling
- [x] Keyboard events (local & global)
- [x] Mouse click support
- [x] Focus management
- [x] Custom event handlers
- [x] Event bubbling

</td>
<td>

### Developer Experience
- [x] Declarative `node!` macro
- [x] Expression support `(expr)`
- [x] Spread operator `...(vec)`
- [x] Hot-reload friendly
- [x] Zero flicker rendering

</td>
</tr>
</table>

<div align='center'>• • •</div>

# <sub>DOCUMENTATION</sub>


| Document | Description |
|----------|-------------|
| **[Examples](./examples)** | Collection of example apps |
| **[Documentation](DOCS.md)** | Complete framework documentation |
| **[Tutorial](TUTORIAL.md)** | Step-by-step guide from basics to advanced |
| **[API Reference](API_REFERENCE.md)** | Detailed API documentation |
| **[Quick Reference](QUICK_REFERENCE.md)** | Handy cheat sheet for common patterns |
| **[Implementation](IMPLEMENTATION.md)** | Internal architecture details |

<div align='center'>• • •</div>

# <sub>DEVELOPMENT</sub>

Want to contribute? We'd love to have you!

- **[Development Guide](DEVELOPMENT.md)** - Set up your dev environment
- **[Contributing](CONTRIBUTING.md)** - Contribution guidelines
- **[GitHub Issues](https://github.com/yourusername/rxtui/issues)** - Report bugs or request features

<div align='center'>• • •</div>

# <sub>LICENSE</sub>

This project is licensed under the [Apache License 2.0](./LICENSE).
