# RxTUI Documentation

RxTUI is a reactive terminal user interface framework for Rust that brings modern component-based architecture to the terminal. It combines React-like patterns with efficient terminal rendering through virtual DOM diffing.

## Table of Contents

- [Getting Started](#getting-started)
- [Core Concepts](#core-concepts)
- [Components](#components)
- [The node! Macro](#the-node-macro)
- [State Management](#state-management)
- [Message Handling](#message-handling)
- [Topic-Based Communication](#topic-based-communication)
- [Layout System](#layout-system)
- [Styling](#styling)
- [Event Handling](#event-handling)
- [Built-in Components](#built-in-components)
- [Effects (Async)](#effects-async)
- [Examples](#examples)

## Getting Started

Add RxTUI to your `Cargo.toml`:

```toml
[dependencies]
rxtui = { path = "rxtui" }

# For async effects support:
rxtui = { path = "rxtui", features = ["effects"] }
tokio = { version = "1.0", features = ["full"] }
```

Create your first app:

```rust
use rxtui::prelude::*;

#[derive(Component)]
struct HelloWorld;

impl HelloWorld {
    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: blue, pad: 2) [
                text("Hello, Terminal!", color: white, bold)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(HelloWorld)
}
```

## Core Concepts

RxTUI follows a component-based architecture where your UI is composed of reusable components that manage their own state and handle messages.

### The Component Lifecycle

1. **Component** creates a **View** (UI tree)
2. **Events** trigger **Messages**
3. **Messages** are processed by **Update**
4. **Update** modifies **State**
5. **State** changes trigger re-render
6. Back to step 1

### Key Types

- **Component**: A reusable UI element with behavior
- **Node**: The UI tree structure (divs, text, components)
- **Message**: Events that trigger state changes
- **State**: Component's data
- **Action**: What to do after processing a message
- **Context**: Provides access to state and message dispatch

## Components

Components are the building blocks of your UI. They encapsulate state, handle messages, and render views.

### Basic Component

```rust
use rxtui::prelude::*;

// Define messages
#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
}

// Define state
#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

// Define component
#[derive(Component)]
struct Counter;

impl Counter {
    // Handle messages and update state
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
        }
    }

    // Render the UI
    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Count: {}", state.count)),
                hstack(gap: 2) [
                    div(border: white, pad: 1, focusable) [
                        text("-"),
                        @click: ctx.handler(CounterMsg::Decrement)
                    ],
                    div(border: white, pad: 1, focusable) [
                        text("+"),
                        @click: ctx.handler(CounterMsg::Increment)
                    ]
                ]
            ]
        }
    }
}
```

### Component Trait

The `#[derive(Component)]` macro automatically implements the Component trait. You can also implement it manually:

```rust
impl Component for MyComponent {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        // Handle messages
    }

    fn view(&self, ctx: &Context) -> Node {
        // Return UI tree
    }

    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        // Return async effects (optional)
    }
}
```

## The node! Macro

The `node!` macro provides a declarative syntax for building UI trees, inspired by modern UI frameworks like SwiftUI and Jetpack Compose.

### Basic Syntax

```rust
node! {
    // Root element (usually div)
    div(properties) [
        // Children
        text("content", properties),
        div(properties) [
            // Nested children
        ],
        // Event handlers
        @click: handler,
        @key(Enter): handler
    ]
}
```

### Elements

#### Div Container

```rust
node! {
    div(
        // Layout
        dir: vertical,      // or horizontal, v, h
        gap: 2,            // space between children
        wrap: wrap,        // wrap mode

        // Sizing
        w: 50,             // fixed width
        h: 20,             // fixed height
        w_pct: 0.5,        // 50% of parent width
        h_pct: 0.8,        // 80% of parent height
        w_auto,            // automatic width
        h_content,         // size to content

        // Styling
        bg: blue,          // background color
        pad: 2,            // padding all sides
        pad_h: 1,          // horizontal padding
        pad_v: 1,          // vertical padding

        // Borders
        border: white,     // border color
        border_style: (BorderStyle::Rounded, yellow),
        border_edges: BorderEdges::TOP | BorderEdges::BOTTOM,

        // Interaction
        focusable,         // can receive focus
        overflow: scroll,  // scroll, hidden, auto
        show_scrollbar: true,

        // Positioning
        absolute,          // absolute positioning
        top: 5,
        left: 10,
        z: 100            // z-index
    ) [
        // Children here
    ]
}
```

#### Text

```rust
node! {
    div [
        // Simple text
        text("Hello"),

        // Styled text
        text("Styled", color: red, bold, italic, underline),

        // Dynamic text
        text(format!("Count: {}", count)),

        // Text with wrapping
        text("Long text...", wrap: word)
    ]
}
```

#### Rich Text

```rust
node! {
    div [
        richtext [
            text("Normal "),
            text("Bold", bold),
            text(" and "),
            text("Colored", color: red)
        ],

        // With top-level styling
        richtext(wrap: word) [
            text("Line 1 "),
            text("Important", color: yellow, bold),
            text(" continues...")
        ]
    ]
}
```

#### Stacks

```rust
node! {
    div [
        // Vertical stack (default)
        vstack [
            text("Top"),
            text("Bottom")
        ],

        // Horizontal stack
        hstack(gap: 2) [
            text("Left"),
            text("Right")
        ]
    ]
}
```

#### Components

```rust
node! {
    div [
        // Embed other components
        node(MyComponent::new("config")),
        node(Counter)
    ]
}
```

#### Spacers

```rust
node! {
    div [
        text("Top"),
        spacer(2),  // 2 lines of space
        text("Bottom")
    ]
}
```

### Event Handlers

```rust
node! {
    div(focusable) [
        text("Interactive"),

        // Mouse events
        @click: ctx.handler(Msg::Clicked),

        // Keyboard events (requires focus)
        @char('a'): ctx.handler(Msg::KeyA),
        @key(Enter): ctx.handler(Msg::Enter),
        @key(Char('-')): ctx.handler(Msg::Minus),

        // Focus events
        @focus: ctx.handler(Msg::Focused),
        @blur: ctx.handler(Msg::Blurred),

        // Global events (work without focus)
        @char_global('q'): ctx.handler(Msg::Quit),
        @key_global(Esc): ctx.handler(Msg::Exit),

        // Any character handler
        @any_char: |ch| ctx.handler(Msg::Typed(ch))
    ]
}
```

### Optional Properties

Use `!` suffix for optional properties:

```rust
node! {
    div(
        // Only applied if Some
        bg: (optional_color)!,
        w: (optional_width)!,
        border: (if selected { Some(Color::Yellow) } else { None })!
    ) [
        text("Conditional styling")
    ]
}
```

## State Management

RxTUI provides automatic state management through the Context.

### Component State

```rust
#[derive(Debug, Clone, Default)]
struct MyState {
    counter: i32,
    text: String,
}

impl MyComponent {
    #[update]
    fn update(&self, ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
        // The #[update] macro automatically fetches state
        // and passes it as the last parameter

        state.counter += 1;
        Action::update(state)  // Save the new state
    }

    #[view]
    fn view(&self, ctx: &Context, state: MyState) -> Node {
        // The #[view] macro automatically fetches state
        node! {
            div [
                text(format!("Counter: {}", state.counter))
            ]
        }
    }
}
```

### Manual State Access

```rust
fn update(&self, ctx: &Context, msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
    // Manually get state (or initialize with Default)
    let mut state = ctx.get_state::<MyState>();

    // Modify state
    state.counter += 1;

    // Return updated state
    Action::update(state)
}
```

## Message Handling

Messages are how components respond to events.

### Basic Messages

```rust
#[derive(Debug, Clone)]
enum MyMsg {
    Click,
    KeyPress(char),
    Update(String),
}

impl MyComponent {
    #[update]
    fn update(&self, ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
        match msg {
            MyMsg::Click => {
                state.clicked = true;
                Action::update(state)
            }
            MyMsg::KeyPress(ch) => {
                state.text.push(ch);
                Action::update(state)
            }
            MyMsg::Update(text) => {
                state.text = text;
                Action::update(state)
            }
        }
    }
}
```

### Actions

Update methods return an Action:

```rust
pub enum Action {
    Update(Box<dyn State>),              // Update component state
    UpdateTopic(String, Box<dyn State>), // Update topic state
    None,                                // No action
    Exit,                                // Exit application
}
```

### Message with Value

```rust
// In view
node! {
    div [
        @any_char: ctx.handler_with_value(|ch| Box::new(MyMsg::Typed(ch)))
    ]
}
```

## Topic-Based Communication

Topics enable cross-component communication without direct references.

### Sending to Topics

```rust
impl Dashboard {
    #[update]
    fn update(&self, ctx: &Context, msg: DashboardMsg, state: DashboardState) -> Action {
        match msg {
            DashboardMsg::NotifyAll => {
                // Send message to topic
                ctx.send_to_topic("notifications", NotificationMsg::Alert);
                Action::none()
            }
        }
    }
}
```

### Receiving Topic Messages

```rust
impl NotificationBar {
    // Static topic
    #[update(msg = LocalMsg, topics = ["notifications" => NotificationMsg])]
    fn update(&self, ctx: &Context, messages: Messages, mut state: State) -> Action {
        match messages {
            Messages::LocalMsg(msg) => {
                // Handle local messages
            }
            Messages::NotificationMsg(msg) => {
                // Handle topic messages
                // Returning Action::update claims topic ownership
                state.notifications.push(msg);
                Action::update(state)
            }
        }
    }
}
```

### Dynamic Topics

```rust
struct Counter {
    topic_name: String,  // Topic determined at runtime
}

impl Counter {
    // Dynamic topic from field
    #[update(msg = CounterMsg, topics = [self.topic_name => ResetSignal])]
    fn update(&self, ctx: &Context, messages: Messages, mut state: CounterState) -> Action {
        match messages {
            Messages::CounterMsg(msg) => { /* ... */ }
            Messages::ResetSignal(_) => {
                // Reset when signal received
                Action::update(CounterState::default())
            }
        }
    }
}
```

### Topic State

```rust
// Write topic state (first writer becomes owner)
Action::UpdateTopic("app.settings".to_string(), Box::new(settings))

// Read topic state from any component
let settings: Option<Settings> = ctx.read_topic("app.settings");
```

## Layout System

RxTUI provides a flexible layout system with multiple sizing modes.

### Dimension Types

```rust
pub enum Dimension {
    Fixed(u16),       // Exact size in cells
    Percentage(f32),  // Percentage of parent (0.0 to 1.0)
    Auto,            // Share remaining space equally
    Content,         // Size based on children
}
```

### Layout Examples

```rust
node! {
    // Fixed layout
    div(w: 80, h: 24) [
        text("Fixed size")
    ],

    // Percentage-based
    div(w_pct: 0.5, h_pct: 0.8) [
        text("50% width, 80% height")
    ],

    // Auto sizing - share remaining space
    hstack [
        div(w: 20) [ text("Fixed") ],
        div(w_auto) [ text("Auto 1") ],  // Gets 50% of remaining
        div(w_auto) [ text("Auto 2") ]   // Gets 50% of remaining
    ],

    // Content-based sizing
    div(w_content, h_content) [
        text("Size fits content")
    ]
}
```

### Direction and Wrapping

```rust
node! {
    // Vertical layout (default)
    div(dir: vertical, gap: 2) [
        text("Line 1"),
        text("Line 2")
    ],

    // Horizontal layout
    div(dir: horizontal, gap: 1) [
        text("Col 1"),
        text("Col 2")
    ],

    // With wrapping
    div(dir: horizontal, wrap: wrap, w: 40) [
        // Children wrap to next line when width exceeded
        div(w: 15) [ text("Item 1") ],
        div(w: 15) [ text("Item 2") ],
        div(w: 15) [ text("Item 3") ]  // Wraps to next line
    ]
}
```

### Scrolling

```rust
node! {
    div(
        h: 10,              // Fixed container height
        overflow: scroll,   // Enable scrolling
        show_scrollbar: true,
        focusable          // Must be focusable for keyboard scrolling
    ) [
        // Content taller than container
        text("Line 1"),
        text("Line 2"),
        // ... many more lines
        text("Line 50")
    ]
}
```

Scrolling controls:
- **Arrow keys**: Scroll up/down by 1 line
- **Page Up/Down**: Scroll by container height
- **Home/End**: Jump to top/bottom
- **Mouse wheel**: Scroll up/down

Note: Only vertical scrolling is currently implemented.

## Styling

### Colors

RxTUI supports multiple color formats:

```rust
node! {
    div [
        // Named colors
        text("Red", color: red),
        text("Bright Blue", color: bright_blue),

        // Hex colors
        text("Hex", color: "#FF5733"),

        // RGB
        text("RGB", color: (Color::Rgb(255, 128, 0))),

        // Conditional
        text("Status", color: (if ok { green } else { red }))
    ]
}
```

Available named colors:
- Basic: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- Bright: `bright_black`, `bright_red`, `bright_green`, `bright_yellow`, `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`

### Borders

```rust
node! {
    div [
        // Simple border
        div(border: white) [ text("Single border") ],

        // Border styles
        div(border_style: (BorderStyle::Rounded, cyan)) [
            text("Rounded border")
        ],

        // Partial borders
        div(
            border: white,
            border_edges: BorderEdges::TOP | BorderEdges::BOTTOM
        ) [
            text("Top and bottom only")
        ]
    ]
}
```

Border styles:
- `Single` - Normal lines
- `Double` - Double lines
- `Rounded` - Rounded corners
- `Thick` - Thick lines

### Spacing

```rust
node! {
    div [
        // Padding
        div(pad: 2) [ text("All sides") ],
        div(pad_h: 2) [ text("Horizontal") ],
        div(pad_v: 1) [ text("Vertical") ],
        div(padding: (Spacing::new(1, 2, 3, 4))) [ text("Custom") ],

        // Gap between children
        div(gap: 2) [
            text("Item 1"),
            text("Item 2")  // 2 cells gap
        ]
    ]
}
```

### Focus Styles

```rust
node! {
    div(
        focusable,
        border: white,
        focus_style: ({
            Style::default()
                .background(Color::Blue)
                .border(Color::Yellow)
        })
    ) [
        text("Changes style when focused")
    ]
}
```

## Event Handling

### Focus-Based Events

Most events require the element to be focused:

```rust
node! {
    div(focusable) [
        text("Click or press keys"),

        // Mouse
        @click: ctx.handler(Msg::Clicked),

        // Keyboard
        @char('a'): ctx.handler(Msg::PressedA),
        @key(Enter): ctx.handler(Msg::Confirmed),
        @key(Backspace): ctx.handler(Msg::Delete),

        // Focus
        @focus: ctx.handler(Msg::GainedFocus),
        @blur: ctx.handler(Msg::LostFocus)
    ]
}
```

### Global Events

Global events work regardless of focus:

```rust
node! {
    div [
        // Application-wide shortcuts
        @char_global('q'): ctx.handler(Msg::Quit),
        @key_global(Esc): ctx.handler(Msg::Cancel),
        @char_global('/'): ctx.handler(Msg::Search)
    ]
}
```

### Focus Navigation

- **Tab**: Move to next focusable element
- **Shift+Tab**: Move to previous focusable element

## Built-in Components

### TextInput

A full-featured text input component:

```rust
use rxtui::components::TextInput;

node! {
    div [
        // Basic input
        input(placeholder: "Enter name...", focusable),

        // Custom styling
        input(
            placeholder: "Password...",
            password,              // Mask input
            border: yellow,
            w: 40,
            content_color: green,
            cursor_color: white
        ),

        // Or use the builder API
        node(
            TextInput::new()
                .placeholder("Email...")
                .width(50)
                .border(Color::Cyan)
                .focus_border(Color::Yellow)
        )
    ]
}
```

TextInput features:
- Full text editing (insert, delete, backspace)
- Cursor movement (arrows, Home/End)
- Word navigation (Alt+B/F or Ctrl+arrows)
- Word deletion (Ctrl+W, Alt+D)
- Line deletion (Ctrl+U/K)
- Password mode
- Placeholder text
- Customizable styling

## Effects (Async)

Effects enable async operations like timers, network requests, and file monitoring.

### Basic Effect

```rust
use rxtui::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct Timer;

#[component]  // Required to collect #[effect] methods
impl Timer {
    #[update]
    fn update(&self, ctx: &Context, msg: TimerMsg, mut state: TimerState) -> Action {
        match msg {
            TimerMsg::Tick => {
                state.seconds += 1;
                Action::update(state)
            }
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: TimerState) -> Node {
        node! {
            div [
                text(format!("Time: {}s", state.seconds))
            ]
        }
    }

    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send(TimerMsg::Tick);
        }
    }
}
```

### Multiple Effects

```rust
#[component]
impl MyComponent {
    #[effect]
    async fn monitor_file(&self, ctx: &Context) {
        // Watch for file changes
    }

    #[effect]
    async fn fetch_data(&self, ctx: &Context, state: MyState) {
        // Effects can access state
        if state.should_fetch {
            // Fetch from API
        }
    }
}
```

### Manual Effects

```rust
impl Component for MyComponent {
    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        vec![
            Box::pin(async move {
                // Async code
            })
        ]
    }
}
```

## Examples

### Complete Counter App

```rust
use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum Msg {
    Increment,
    Decrement,
    Reset,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct State {
    count: i32,
}

#[derive(Component)]
struct CounterApp;

impl CounterApp {
    #[update]
    fn update(&self, _ctx: &Context, msg: Msg, mut state: State) -> Action {
        match msg {
            Msg::Increment => {
                state.count += 1;
                Action::update(state)
            }
            Msg::Decrement => {
                state.count -= 1;
                Action::update(state)
            }
            Msg::Reset => Action::update(State::default()),
            Msg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: State) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Count: {}", state.count), color: white, bold),

                hstack(gap: 2) [
                    div(border: white, pad: 1, focusable) [
                        text("-"),
                        @click: ctx.handler(Msg::Decrement),
                        @key(Char('-')): ctx.handler(Msg::Decrement)
                    ],
                    div(border: white, pad: 1, focusable) [
                        text("+"),
                        @click: ctx.handler(Msg::Increment),
                        @key(Char('+')): ctx.handler(Msg::Increment)
                    ],
                    div(border: white, pad: 1, focusable) [
                        text("Reset"),
                        @click: ctx.handler(Msg::Reset),
                        @key(Char('r')): ctx.handler(Msg::Reset)
                    ]
                ],

                text("Press +/- to change, r to reset, q to quit", color: gray),

                @char_global('q'): ctx.handler(Msg::Exit),
                @key_global(Esc): ctx.handler(Msg::Exit)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(CounterApp)
}
```

### Running the Examples

The repository includes several examples:

```bash
# Simple interactive color demo
cargo run --example simple

# Multi-page feature showcase
cargo run --example demo

# Topic-based component communication
cargo run --example components

# Timer with async effects
cargo run --example timer --features effects
```

## Advanced Topics

### Custom Components

Create reusable component libraries:

```rust
pub struct Button {
    label: String,
    on_click: Box<dyn Fn() -> Box<dyn Message>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: Box::new(|| Box::new(())),
        }
    }

    pub fn on_click<F, M>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
        M: Message,
    {
        self.on_click = Box::new(move || Box::new(f()));
        self
    }
}

impl Component for Button {
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div(
                border: white,
                pad: 1,
                focusable,
                focus_style: (Style::default().background(Color::Blue))
            ) [
                text(&self.label),
                @click: (self.on_click)()
            ]
        }
    }
}
```

### Performance Tips

1. **Use keys for lists**: Helps with efficient diffing (not yet implemented)
2. **Minimize state updates**: Only update when necessary
3. **Use topics wisely**: Don't overuse for simple parent-child communication
4. **Profile rendering**: Use `RenderConfig` for debugging

### Debugging

```rust
let mut app = App::with_config(RenderConfig {
    use_double_buffer: false,  // Disable for debugging
    use_diffing: false,        // Show all updates
    poll_duration_ms: 100,     // Slow down for observation
})?;
```

## Architecture Overview

RxTUI uses a multi-layered architecture:

1. **Component Layer**: Your components with state and logic
2. **Virtual DOM**: Efficient tree diffing and patching
3. **Render Tree**: Layout calculation and positioning
4. **Terminal Buffer**: Double-buffered cell-level rendering
5. **Terminal Output**: Optimized escape sequence generation

The framework handles all the complexity, letting you focus on building your UI.
