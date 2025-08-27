# rxtui-macros

[![Crates.io](https://img.shields.io/crates/v/rxtui-macros?style=for-the-badge&logo=rust&logoColor=white)](https://crates.io/crates/rxtui-macros)
[![docs.rs](https://img.shields.io/badge/docs.rs-rxtui--macros-blue?style=for-the-badge&logo=docs.rs)](https://docs.rs/rxtui-macros)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=for-the-badge)](https://github.com/microsandbox/rxtui/blob/main/LICENSE)

Procedural macros for the [RxTUI](https://crates.io/crates/rxtui) terminal UI framework.

> **Note**: This crate provides procedural macros for RxTUI. You should use the main `rxtui` crate directly, which re-exports these macros.

## Usage

Add RxTUI to your project (not this crate directly):

```toml
[dependencies]
rxtui = "0.1"
```

## Provided Macros

### `#[derive(Component)]`

Automatically implements the `Component` trait for your struct:

```rust
use rxtui::prelude::*;

#[derive(Component)]
struct MyApp;

impl MyApp {
    // Add #[update] and #[view] methods
}
```

### `#[update]`

Marks a method as the component's update handler. Automatically handles message downcasting and state management:

```rust
#[update]
fn update(&self, ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
    match msg {
        MyMsg::Increment => {
            state.count += 1;
            Action::update(state)
        }
        MyMsg::Exit => Action::exit()
    }
}
```

The macro automatically:
- Downcasts incoming messages to your message type
- Retrieves and updates component state
- Handles multiple message types if needed

#### Advanced: Multiple Message Types

```rust
#[update]
fn update(&self, ctx: &Context, msg: impl Message, state: State) -> Action {
    // Handle different message types
    match msg {
        AppMsg(m) => { /* ... */ }
        DialogMsg(m) => { /* ... */ }
        _ => Action::none()
    }
}
```

#### Topic-Based Updates

```rust
#[update("navigation")]
fn handle_nav(&self, ctx: &Context, msg: NavMsg, state: State) -> Action {
    // Handle navigation messages
}

#[update("user")]
fn handle_user(&self, ctx: &Context, msg: UserMsg, state: State) -> Action {
    // Handle user messages
}
```

### `#[view]`

Marks a method as the component's view renderer:

```rust
#[view]
fn view(&self, ctx: &Context, state: MyState) -> Node {
    node! {
        div(bg: black, pad: 2) [
            text(format!("Count: {}", state.count))
        ]
    }
}
```

The macro automatically:
- Retrieves the current component state
- Passes it to your view function
- Returns the rendered node tree

### `#[effect]`

Creates async background tasks (requires `effects` feature):

```rust
#[effect]
async fn tick(&self, ctx: &Context) {
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        ctx.send("tick");
    }
}
```

Effects run automatically when the component is mounted and are cancelled when unmounted.

## Complete Example

```rust
use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
    Reset,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

#[derive(Component)]
struct Counter;

impl Counter {
    #[update]
    fn update(&self, _ctx: &Context, msg: CounterMsg, mut state: CounterState) -> Action {
        match msg {
            CounterMsg::Increment => {
                state.count += 1;
                Action::update(state)
            }
            CounterMsg::Decrement => {
                state.count -= 1;
                Action::update(state)
            }
            CounterMsg::Reset => {
                state.count = 0;
                Action::update(state)
            }
            CounterMsg::Exit => Action::exit()
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        node! {
            div(
                bg: black,
                pad: 2,
                @key(up): ctx.handler(CounterMsg::Increment),
                @key(down): ctx.handler(CounterMsg::Decrement),
                @key(r): ctx.handler(CounterMsg::Reset),
                @key(esc): ctx.handler(CounterMsg::Exit)
            ) [
                text(format!("Count: {}", state.count), color: white, bold),
                text("↑/↓: change | r: reset | esc: exit", color: bright_black)
            ]
        }
    }

    #[cfg(feature = "effects")]
    #[effect]
    async fn auto_increment(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            ctx.send(CounterMsg::Increment);
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(Counter)
}
```

## Documentation

For complete documentation and more examples, see the main [RxTUI documentation](https://docs.rs/rxtui).

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](https://github.com/microsandbox/rxtui/blob/main/LICENSE) file for details.
