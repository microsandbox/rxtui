# RxTUI

> [!WARNING]
> This library is highly experimental and under active development. APIs may change significantly between versions. Use in production at your own risk.

A reactive terminal user interface framework for Rust that brings modern web development patterns to the terminal. Build sophisticated TUIs with the same mental model as React, but optimized for terminal constraints.

## Table of Contents

- [Why RxTUI?](#why-rxtui)
- [Quick Start](#quick-start)
- [Core Architecture](#core-architecture)
- [Building Your First Component](#building-your-first-component)
- [The Three Ways to Build UIs](#the-three-ways-to-build-uis)
- [Component Communication](#component-communication)
- [Styling and Layout](#styling-and-layout)
- [Event Handling](#event-handling)
- [Advanced Patterns](#advanced-patterns)
- [Performance Considerations](#performance-considerations)
- [Examples and Recipes](#examples-and-recipes)

## Why RxTUI?

Terminal UIs have traditionally been painful to build. You either work with low-level escape sequences (error-prone and tedious) or use immediate-mode libraries that require you to manage all state manually. RxTUI takes a different approach.

We bring the **retained-mode, component-based architecture** that revolutionized web development to the terminal. This means:

- **Your UI is a function of state** - Just describe what the UI should look like, not how to transition between states
- **Automatic optimization** - The framework handles diffing, dirty region tracking, and minimal redraws
- **Composable components** - Build complex UIs from simple, reusable pieces
- **Familiar patterns** - It is a mix of React component-style and elmish architecture

## Quick Start

Let's build a simple counter to see how it all fits together:

```rust
use rxtui::prelude::*;

// 1. Define your messages (events that can happen)
#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
    Exit,
}

// 2. Define your state (data that changes)
#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

// 3. Create your component
#[derive(Component, Clone, Default)]
struct Counter {}

impl Counter {
    #[update] // Handle messages and update state
    fn update(&self, ctx: &Context, msg: CounterMsg, mut state: CounterState) -> Action {
        match msg {
            CounterMsg::Increment => {
                state.count += 1;
                Action::update(state)
            }
            CounterMsg::Decrement => {
                state.count -= 1;
                Action::update(state)
            }
            CounterMsg::Exit => Action::exit(),
        }
    }

    #[view] // Render UI based on current state
    fn view(&self, ctx: &Context, state: CounterState) -> Node {
        node! {
            div(bg: blue, pad: 2) [
                text(format!("Count: {}", state.count), color: white, bold),
                spacer(1),
                hstack(gap: 2) [
                    div(bg: green, pad: 1, focusable) [
                        text("+ Increment"),
                        @click: ctx.handler(CounterMsg::Increment),
                        @key(Enter): ctx.handler(CounterMsg::Increment)
                    ],
                    div(bg: red, pad: 1, focusable) [
                        text("- Decrement"),
                        @click: ctx.handler(CounterMsg::Decrement),
                        @key(Enter): ctx.handler(CounterMsg::Decrement)
                    ]
                ],
                @key_global(Esc): ctx.handler(CounterMsg::Exit)
            ]
        }
    }
}

// 4. Run your app
fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(Counter::default())
}
```

This example demonstrates the complete flow: messages trigger updates, updates modify state, state changes trigger re-renders, and the UI reflects the new state. The framework handles everything else.

## Core Architecture

To understand how RxTUI works, let's follow a user interaction through the system:

### The Journey of a Keypress

When a user presses a key, here's what happens:

1. **Terminal Input** → The terminal sends raw input to our event system
2. **Event Routing** → The event is routed to the focused component (or globally if configured)
3. **Message Dispatch** → The component's event handler creates a Message
4. **State Update** → The component's `update()` method processes the message and returns a new state
5. **Virtual DOM Diff** → The new state triggers a re-render, creating a new virtual tree
6. **Patch Generation** → The diff engine compares old and new trees, generating minimal patches
7. **Buffer Update** → Patches are applied to our double buffer
8. **Terminal Output** → Optimized escape sequences are sent to the terminal

This pipeline ensures that every update is efficient and flicker-free. You just worry about steps 3-4 (your business logic), and the framework handles the rest.

### Why a Virtual DOM?

You might wonder why we need a virtual DOM for a terminal. After all, terminals are much simpler than web browsers, right?

Actually, terminals present unique challenges:

- **Every character is expensive** - Unlike pixels, each character requires escape sequences to position and style
- **No partial updates** - You can't update just part of a character cell
- **Flicker is obvious** - Clearing and redrawing is immediately visible
- **Limited bandwidth** - SSH connections and serial terminals have real bandwidth constraints

The virtual DOM solves these by:

- Calculating the minimal set of changes needed
- Batching updates intelligently
- Using a double buffer to eliminate flicker
- Optimizing escape sequences (combining movements, reusing styles)

## Building Your First Component

Components are the heart of RxTUI. Each component is a self-contained unit that manages its own state and rendering. Let's build a more realistic component - a task list:

```rust
// First, define what can happen (Messages)
#[derive(Debug, Clone)]
enum TodoMsg {
    AddTask(String),
    ToggleTask(usize),
    DeleteTask(usize),
}

// Then, define your data (State)
#[derive(Debug, Clone)]
struct TodoState {
    tasks: Vec<Task>,
    input_text: String,
}

#[derive(Debug, Clone)]
struct Task {
    text: String,
    completed: bool,
}

// Finally, create your component
#[derive(Component, Clone, Default)]
struct TodoList {}
```

The beauty of this pattern is that each component is independent. You can develop, test, and reason about them in isolation, then compose them into larger applications.

## The Two Ways to Build UIs

RxTUI provides two different APIs for building UIs, each suited to different use cases. Understanding when to use each one is key to productive development.

### 1. The node! Macro (Declarative)

Best for: **Most UI code, especially view methods**

The `node!` macro provides a JSX-like syntax that's concise and readable:

```rust
node! {
    div(bg: blue, pad: 2) [
        text("Hello", color: white),
        div(border: white) [
            text("Nested content")
        ]
    ]
}
```

**Why use it?**

- Minimal boilerplate
- Visual structure matches output
- Great for static layouts
- Easy to read and modify

**When to avoid it?**

- Highly dynamic structures (lots of conditionals)
- Procedurally generated UIs

### 2. The Builder API (Programmatic)

Best for: **Dynamic UIs, reusable components, procedural generation**

The builder API gives you full programmatic control:

```rust
let mut container = Div::new()
    .background(Color::Blue)
    .padding(Spacing::all(2));

// Dynamically add children based on data
for (i, item) in items.iter().enumerate() {
    let child = Text::new(&item.title)
        .color(if item.selected { Color::Yellow } else { Color::White });
    container = container.child(child.into());
}
```

**Why use it?**

- Full programmatic control
- Easy to use loops and conditionals
- Better for dynamic structures
- Type-safe and IDE-friendly

**When to avoid it?**

- Simple, static layouts (macro is cleaner)
- When visual structure is important for readability

### Choosing the Right API

Here's a simple decision tree:

1. **Writing a view method?** → Use `node!` macro
2. **Building UI from dynamic data?** → Use Builder API
3. **Creating a reusable component library?** → Use Builder API
4. **Prototyping quickly?** → Use `node!` macro

## Component Communication

As your application grows, components need to communicate. RxTUI provides several patterns for different scenarios.

### Direct Component Messages

Components send messages to themselves through event handlers:

```rust
impl MyComponent {
    #[view]
    fn view(&self, ctx: &Context) -> Node {
        node! {
            div [
                text("Click me"),
                @click: ctx.handler(MyMsg::Clicked)  // Sends to this component
            ]
        }
    }

    #[update]
    fn update(&self, ctx: &Context, msg: MyMsg) -> Action {
        // Handle messages sent to this component
        match msg {
            MyMsg::Clicked => {
                // Process message...
                Action::none()
            }
        }
    }
}
```

### Topic-Based Communication

Components can communicate across the tree using topics. Topics have ownership - the first component to handle a topic message becomes its owner and receives all future messages for that topic:

```rust
// Sender component - sends messages to a topic
impl Sender {
    #[update]
    fn update(&self, ctx: &Context, msg: SenderMsg) -> Action {
        match msg {
            SenderMsg::NotifyOthers => {
                // Send message to the topic - will be received by the owner (or first handler if unowned)
                ctx.send_to_topic("notifications", NotificationMsg::Alert);
                Action::none()
            }
        }
    }
}

// Receiver component - listens to a specific topic using the macro's topic support
impl Receiver {
    fn new(topic: String) -> Self {
        Self {
            topic_name: topic  // Store which topic to listen to
        }
    }

    #[update(msg = ReceiverMsg, topics = [self.topic_name => NotificationMsg])]
    fn update(&self, ctx: &Context, messages: Messages, mut state: ReceiverState) -> Action {
        match messages {
            Messages::ReceiverMsg(msg) => {
                // Handle regular messages
                Action::none()
            }
            Messages::NotificationMsg(_msg) => {
                // Handle the notification - returning Action::Update claims ownership
                state.notification_count += 1;
                Action::update(state)
                // This component now owns the topic and will receive all future messages
            }
        }
    }
}
```

### Topic State Ownership

Topics can also have associated state. The first component to update a topic becomes its owner:

```rust
impl Dashboard {
    #[update]
    fn update(&self, ctx: &Context, msg: DashboardMsg) -> Action {
        match msg {
            DashboardMsg::UpdateSharedState => {
                // First component to update becomes the owner
                Action::update_topic(
                    "shared.dashboard",
                    SharedDashboardState { /* ... */ }
                )
            }
        }
    }
}

// Other components can read topic state
impl Reader {
    #[view]
    fn view(&self, ctx: &Context) -> Node {
        // Read shared state from topic
        let shared = ctx.read_topic::<SharedDashboardState>("shared.dashboard");

        node! {
            div [
                text(format!("Shared value: {:?}", shared))
            ]
        }
    }
}
```

### Parent-Child Data Flow

Parents pass data down through component constructors:

```rust
impl Parent {
    #[view]
    fn view(&self, ctx: &Context, state: ParentState) -> Node {
        node! {
            div [
                // Pass data to child via constructor
                node(ChildComponent::new(state.child_config.clone())),
            ]
        }
    }
}
```

**When to use each pattern:**

- **Direct messages** - For component's internal state changes
- **Topics** - For cross-component communication and broadcasting
- **Topic state** - For shared state that multiple components need to read
- **Constructor data** - For configuration and initial state from parent to child

## Styling and Layout

RxTUI's styling system is designed to be familiar yet optimized for terminals.

### The Box Model

Just like CSS, every element has:

- **Content** - The actual text or child elements
- **Padding** - Space inside the border
- **Border** - Optional decorative border
- **Margin** - Space outside (via parent's gap or spacing)

```rust
div(
    pad: 2,        // Padding: 2 chars on all sides
    border: white, // Border with white color
    gap: 1,        // Gap between children (acts like margin)
)
```

### Layout Modes

#### Directional Layout (Default)

Elements stack in a direction (vertical by default):

```rust
// Vertical stacking (default)
div [
    text("First"),  // ↓
    text("Second"), // ↓
    text("Third")   // ↓
]

// Horizontal stacking
div(dir: horizontal) [
    text("Left"), /* → */ text("Center"), /* → */ text("Right")
]
```

#### Percentage-based Sizing

Make responsive layouts that adapt to terminal size:

```rust
div(w_pct: 0.5, h_pct: 0.8) [  // 50% width, 80% height
    text("Responsive content")
]
```

**Why percentages?** Terminal sizes vary wildly. Percentages ensure your UI looks good at any size.

#### Absolute Positioning

Break out of normal flow for overlays and popups:

```rust
div [
    text("Normal flow content"),

    // Overlay
    div(absolute, top: 5, right: 5, bg: red, z: 100) [
        text("Notification!")
    ]
]
```

### Styling Best Practices

1. **Use semantic colors** - Define color constants for your app's palette
2. **Prefer percentages for major layouts** - Fixed sizes for small elements only
3. **Test at different terminal sizes** - Users have varied setups
4. **Use borders sparingly** - They consume precious terminal space
5. **Consider monochrome** - Some users disable colors

## Event Handling

Events drive interactivity. RxTUI provides a rich event system that handles both keyboard and mouse input.

### Focus-based Events

Most events require focus. This creates predictable, accessible interfaces:

```rust
div(focusable) [
    text("Click or press Enter"),
    @click: handler,
    @key(Enter): handler,
    @char('a'): handler
]
```

**Why require focus?** It prevents accidental actions and makes keyboard navigation predictable.

### Global Events

Some events should work regardless of focus:

```rust
div [
    // App-wide shortcuts
    @key_global(Esc): ctx.handler(Msg::ShowMenu),
    @char_global('/'): ctx.handler(Msg::StartSearch),
    @char_global('?'): ctx.handler(Msg::ShowHelp)
]
```

**When to use global events?**

- Application-wide shortcuts (Esc, Ctrl+C)
- Modal triggers (/, ?, :)
- Emergency exits

### Event Propagation

Events bubble up from focused element to root, stopping at the first handler:

```rust
div [
    @key(Enter): parent_handler,  // Handles if child doesn't

    div(focusable) [
        @key(Enter): child_handler,  // Handles first when focused
    ]
]
```

This enables both specific and fallback behaviors.

## Advanced Patterns

### Composing Complex UIs

Build complex interfaces from simple, focused components:

```rust
impl App {
    #[view]
    fn view(&self, ctx: &Context, state: AppState) -> Node {
        node! {
            div(dir: vertical) [
                node(Header::new(&state.title)),

                div(dir: horizontal, h_pct: 0.8) [
                    node(Sidebar::new(&state.menu_items)),
                    node(Content::new(&state.current_page)),
                ],

                node(StatusBar::new(&state.status))
            ]
        }
    }
}
```

Each component handles its own concerns, making the overall structure clear and maintainable.

### Rich Text and Syntax Highlighting

For inline styling, use RichText:

```rust
node! {
    richtext [
        text("Error: ", color: red, bold),
        text("File "),
        text("config.json", color: cyan, underline),
        text(" not found")
    ]
}
```

This is perfect for:

- Syntax highlighting
- Log output with severity colors
- Inline emphasis
- Status indicators

### Interactive Forms

Combine multiple input components for forms:

```rust
struct FormState {
    username: String,
    password: String,
    remember: bool,
}

impl Form {
    #[view]
    fn view(&self, ctx: &Context, state: FormState) -> Node {
        node! {
            div(pad: 2) [
                text("Username:"),
                input(placeholder: "Enter username", focusable),

                spacer(1),

                text("Password:"),
                input(placeholder: "Enter password", focusable),

                spacer(1),

                hstack(gap: 2) [
                    div(bg: green, pad: 1, focusable) [
                        text("Login"),
                        @click: ctx.handler(FormMsg::Submit)
                    ],
                    div(bg: gray, pad: 1, focusable) [
                        text("Cancel"),
                        @click: ctx.handler(FormMsg::Cancel)
                    ]
                ]
            ]
        }
    }
}
```

### Scrollable Content

Handle content taller than its container:

```rust
// Container with fixed height of 10 lines, but contains 20+ lines of content
div(h: 10, overflow: hidden, show_scrollbar: true, focusable) [
    text("Line 1"),
    text("Line 2"),
    text("Line 3"),
    text("Line 4"),
    text("Line 5"),
    text("Line 6"),
    text("Line 7"),
    text("Line 8"),
    text("Line 9"),
    text("Line 10"),
    text("Line 11"),
    text("Line 12"),
    text("Line 13"),
    text("Line 14"),
    text("Line 15"),
    text("Line 16"),
    text("Line 17"),
    text("Line 18"),
    text("Line 19"),
    text("Line 20")
]
```

The framework automatically:

- Tracks vertical scroll position (horizontal scrolling is not yet supported)
- Shows vertical scrollbar indicator
- Handles keyboard scrolling when focused (Up/Down arrows, PageUp/PageDown, Home/End)
- Supports mouse wheel scrolling
- Optimizes rendering to only draw visible content

**Note:** Currently only vertical scrolling is implemented. Content that exceeds the horizontal viewport will be clipped.

## Examples

The `examples/` directory contains full applications:

- **simple.rs** - Interactive color picker showing event handling
- **demo.rs** - Multi-page showcase of all features
- **components.rs** - Topic-based component communication with multiple counters
- **spinner.rs** - Animated loading spinner with state management

Each example is thoroughly commented and demonstrates best practices.

## Getting Help

- **Examples** - Start with the examples, they cover most use cases
- **API Docs** - Run `cargo doc --open` for detailed API documentation
- **Source Code** - The source is well-commented and structured
- **Issues** - Check GitHub issues for common problems and solutions

Remember: RxTUI is about making terminal UI development enjoyable. If something feels hard, there's probably a better way. The framework handles the complexity so you can focus on your application logic.
