<div align="center">
  <img width="450" alt="RxTUI Logo" src="https://github.com/user-attachments/assets/4d8cc0f6-9f17-43bf-8368-9ad616c1f91f" />

  <br />
  <br />

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

<br />

# <sub>WHY RXTUI?</sub>

Terminal UIs have traditionally been painful to build. You either work with low-level escape sequences (error-prone and tedious) or use immediate-mode libraries that require you to manage all state manually. **RxTUI** takes a different approach.

We bring the retained-mode, component-based architecture that revolutionized web development to the terminal:

- [x] **Declarative UI** - Describe what your UI should look like, not how to change it
- [x] **TUI Optimizations** - Automatic diffing, dirty tracking, and minimal redraws
- [x] **True Composability** - Build complex apps from simple, reusable components
- [x] **Best of Both Worlds** - Combines React's components with Elm's message architecture

<div align='center'>• • •</div>

# <sub>QUICK START</sub>

### <span>1</span>&nbsp;&nbsp;Install RxTUI

Add to your `Cargo.toml`:

```toml
[dependencies]
rxtui = "0.1"
tokio = { version = "1", features = ["full"] }  # For async effects
```

### <span>2</span>&nbsp;&nbsp;Create Your First App

Complete counter app in 30 lines of code:

```rust
use rxtui::prelude::*;

#[derive(Component)]
struct Counter;

impl Counter {
    #[update]
    fn update(&self, _ctx: &Context, msg: &str, mut count: i32) -> Action {
        match msg {
            "inc" => Action::update(count + 1),
            "dec" => Action::update(count - 1),
            _ => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, count: i32) -> Node {
        node! {
            div(@key(up): ctx.handler("inc"), @key(down): ctx.handler("dec"), @key(esc): ctx.handler("exit")) [
                text(format!("Count: {count}"), color: white, bold),
                text("use ↑/↓ to change, esc to exit", color: bright_black)
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

<img width="100%" alt="counter demo" src="https://github.com/user-attachments/assets/286fd51c-6be5-426c-8fc2-315b518d41eb" />

<div align='center'>• • •</div>

# <sub>DOCUMENTATION</sub>

| Document                                  | Description                                |
| ----------------------------------------- | ------------------------------------------ |
| **[Examples](./examples)**                | Collection of example apps                 |
| **[Documentation](DOCS.md)**              | Complete framework documentation           |
| **[Tutorial](TUTORIAL.md)**               | Step-by-step guide from basics to advanced |
| **[API Reference](API_REFERENCE.md)**     | Detailed API documentation                 |
| **[Quick Reference](QUICK_REFERENCE.md)** | Handy cheat sheet for common patterns      |
| **[Implementation](IMPLEMENTATION.md)**   | Internal architecture details              |

<div align='center'>• • •</div>

# <sub>DEVELOPMENT</sub>

Want to contribute? We'd love to have you!

- **[Development Guide](DEVELOPMENT.md)** - Set up your dev environment
- **[Contributing](CONTRIBUTING.md)** - Contribution guidelines
- **[GitHub Issues](https://github.com/yourusername/rxtui/issues)** - Report bugs or request features

<div align='center'>• • •</div>

# <sub>LICENSE</sub>

This project is licensed under the [Apache License 2.0](./LICENSE).
