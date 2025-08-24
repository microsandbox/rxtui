# RxTUI Quick Reference

## Component Template

```rust
use rxtui::prelude::*;

#[derive(Debug, Clone)]
enum MyMsg {
    Click,
    Exit,
}

#[derive(Debug, Clone, Default)]
struct MyState {
    counter: i32,
}

#[derive(Component)]
struct MyComponent;

impl MyComponent {
    #[update]
    fn update(&self, _ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
        match msg {
            MyMsg::Click => {
                state.counter += 1;
                Action::update(state)
            }
            MyMsg::Exit => Action::exit(),
        }
    }

    #[view]
    fn view(&self, ctx: &Context, state: MyState) -> Node {
        node! {
            div(bg: black, pad: 2) [
                text(format!("Count: {}", state.counter)),
                @click: ctx.handler(MyMsg::Click),
                @key_global(Esc): ctx.handler(MyMsg::Exit)
            ]
        }
    }
}

fn main() -> std::io::Result<()> {
    App::new()?.run(MyComponent)
}
```

## node! Macro Syntax

Declarative UI building with a Rust-native DSL:

### Elements

```rust
node! {
    // Containers
    div(...) [...],
    vstack(...) [...],    // Vertical stack
    hstack(...) [...],    // Horizontal stack

    // Text
    text("content", ...),
    richtext(...) [
        text("span1"),
        text("span2", color: red)
    ],

    // Components
    node(MyComponent::new()),

    // Input
    input(placeholder: "...", focusable),

    // Spacer
    spacer(2),
}
```

### Div Properties

```rust
div(
    // Layout
    dir: vertical,        // horizontal, v, h
    gap: 2,              // space between children
    wrap: wrap,          // wrap, nowrap

    // Sizing
    w: 50,               // fixed width
    h: 20,               // fixed height
    w_pct: 0.5,          // 50% width
    h_pct: 0.8,          // 80% height
    w_auto,              // auto width
    h_auto,              // auto height
    w_content,           // fit content width
    h_content,           // fit content height

    // Styling
    bg: blue,            // background
    pad: 2,              // padding all
    pad_h: 1,            // padding horizontal
    pad_v: 1,            // padding vertical

    // Border
    border: white,       // border color
    border_style: (BorderStyle::Rounded, cyan),
    border_edges: BorderEdges::TOP | BorderEdges::BOTTOM,

    // Scrolling
    overflow: scroll,    // hidden, auto
    show_scrollbar: true,

    // Focus
    focusable,           // can receive focus
    focus_style: (Style::default().background(Color::Blue)),

    // Position
    absolute,            // absolute positioning
    pos: absolute,       // same as above
    top: 5,
    left: 10,
    bottom: 5,
    right: 10,
    z: 100,             // z-index
)
```

### Text Properties

```rust
text(
    "content",

    // Colors
    color: red,          // text color
    bg: blue,           // background

    // Styles
    bold,
    italic,
    underline,
    strikethrough,

    // Wrapping
    wrap: word,         // none, character, word, word_break
)
```

### Event Handlers

```rust
div(focusable) [
    // Mouse
    @click: handler,

    // Keyboard (requires focus)
    @char('a'): handler,
    @key(Enter): handler,
    @key(Backspace): handler,
    @char('-'): handler,  // For character keys, use @char

    // Global (no focus needed)
    @char_global('q'): handler,
    @key_global(Esc): handler,

    // Focus
    @focus: handler,
    @blur: handler,

    // Any character
    @any_char: |ch| handler(ch),
]
```

### Optional Properties

```rust
div(
    // Use ! suffix for Option<T> values
    bg: (optional_color)!,
    w: (optional_width)!,
    border: (if selected { Some(yellow) } else { None })!,
)
```

## Colors

### Named Colors

Basic: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`

Bright: `bright_black`, `bright_red`, `bright_green`, `bright_yellow`, `bright_blue`, `bright_magenta`, `bright_cyan`, `bright_white`

### Color Formats

```rust
// Named (no prefix)
color: red
color: bright_blue

// Hex string
color: "#FF5733"
color: "#F50"

// RGB expression
color: (Color::Rgb(255, 128, 0))

// Variable
color: (my_color)

// Conditional
color: (if ok { green } else { red })
```

## Common Patterns

### Loading State

Use the builder pattern or pre-compute nodes when you need dynamic content:

```rust
// Option 1: Pre-compute the node
let status_node = match state.status {
    Loading => node! { text("Loading...", color: yellow) },
    Error(e) => node! { text(format!("Error: {}", e), color: red) },
    Success(data) => node! { text(format!("Data: {}", data)) },
};

node! {
    div [
        node(status_node)
    ]
}

// Option 2: Use builder pattern
let content = match state.status {
    Loading => Text::new("Loading...").color(Color::Yellow).into(),
    Error(e) => Text::new(format!("Error: {}", e)).color(Color::Red).into(),
    Success(data) => Text::new(format!("Data: {}", data)).into(),
};

Div::default()
    .children(vec![content])
    .into()
```

### List Rendering

Build lists outside the macro or use the builder pattern:

```rust
// Pre-build the list of nodes
let item_nodes: Vec<Node> = state.items.iter()
    .map(|item| node! {
        div [
            text(&item.name)
        ]
    })
    .collect();

// Then use builder pattern to combine
Div::default()
    .children(item_nodes)
    .into()
```

### Conditional Rendering

Handle conditions outside the macro:

```rust
// Build conditional elements
let mut children = vec![];

if state.show_header {
    children.push(node! { text("Header", bold) });
}

children.push(node! { text("Always visible") });

if let Some(message) = &state.message {
    children.push(node! { text(message, color: yellow) });
}

// Use builder pattern
Div::default()
    .children(children)
    .into()
```

### Scrollable Container

```rust
div(
    h: 10,               // fixed height
    overflow: scroll,
    show_scrollbar: true,
    focusable           // for keyboard scrolling
) [
    // content taller than container
]
```

### Modal Overlay

```rust
div [
    // Main content
    div [ /* ... */ ],

    // Modal
    if state.show_modal {
        div(absolute, top: 0, left: 0, w_pct: 1.0, h_pct: 1.0, bg: black, z: 1000) [
            div(w: 40, h: 10, bg: white, border: black, pad: 2) [
                text("Modal Content", color: black)
            ]
        ]
    }
]
```

## Update Patterns

### Basic Update

```rust
#[update]
fn update(&self, ctx: &Context, msg: MyMsg, mut state: MyState) -> Action {
    match msg {
        MyMsg::Increment => {
            state.count += 1;
            Action::update(state)
        }
    }
}
```

### Topic Messaging

```rust
// Send to topic
ctx.send_to_topic("my.topic", MyMessage);

// Receive from topic
#[update(msg = LocalMsg, topics = ["my.topic" => TopicMsg])]
fn update(&self, ctx: &Context, messages: Messages, mut state: State) -> Action {
    match messages {
        Messages::LocalMsg(msg) => { /* local */ }
        Messages::TopicMsg(msg) => { /* from topic */ }
    }
}
```

### Dynamic Topics

```rust
struct Component {
    topic: String,
}

#[update(msg = Msg, topics = [self.topic => TopicMsg])]
fn update(&self, ctx: &Context, messages: Messages, state: State) -> Action {
    // Topic name from field
}
```

## Effects (Async)

Effects are enabled by default. Just add tokio:
```toml
[dependencies]
rxtui = "0.1"
tokio = { version = "1", features = ["full"] }
```

### Timer Effect

```rust
#[component]
impl Timer {
    #[effect]
    async fn tick(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send(TimerMsg::Tick);
        }
    }
}
```

### Effect with State

```rust
#[effect]
async fn monitor(&self, ctx: &Context, state: MyState) {
    if state.should_monitor {
        // Do async work
    }
}
```

## TextInput

### Basic Usage

```rust
node! {
    div [
        input(placeholder: "Enter text...", focusable)
    ]
}
```

### Customized

```rust
node! {
    div [
        input(
            placeholder: "Password",
            password,           // mask input
            border: yellow,
            w: 40,
            content_color: green,
            cursor_color: white,
            focusable
        )
    ]
}
```

### Builder API

```rust
node(
    TextInput::new()
        .placeholder("Email...")
        .width(50)
        .border(Color::Cyan)
        .focus_border(Color::Yellow)
)
```

## Layout Tips

### Responsive Layout

```rust
div(w_pct: 1.0, h_pct: 1.0) [  // Full screen
    div(w_pct: 0.3) [ /* 30% sidebar */ ],
    div(w_pct: 0.7) [ /* 70% main */ ]
]
```

### Auto Sizing

```rust
hstack [
    div(w: 20) [ /* fixed */ ],
    div(w_auto) [ /* expands */ ],
    div(w: 20) [ /* fixed */ ]
]
```

### Content Sizing

```rust
div(w_content, h_content) [
    // Size fits children
]
```

## Keyboard Shortcuts

### Focus Navigation
- `Tab` - Next focusable
- `Shift+Tab` - Previous focusable

### Scrolling (when focused)
- `↑/↓` - Scroll up/down
- `Page Up/Down` - Page scroll
- `Home/End` - Jump to top/bottom

### TextInput
- `←/→` - Move cursor
- `Home/End` - Line start/end
- `Alt+B/F` - Word left/right
- `Ctrl+W` - Delete word backward
- `Alt+D` - Delete word forward
- `Ctrl+U` - Delete to line start
- `Ctrl+K` - Delete to line end

## Actions

```rust
Action::update(state)        // Update component state
Action::update_topic(topic, state)  // Update topic state
Action::none()              // No action
Action::exit()              // Exit app
```

## App Configuration

```rust
let mut app = App::new()?
    .render_config(RenderConfig {
        poll_duration_ms: 16,      // Event poll timeout
        use_double_buffer: true,   // Flicker-free rendering
        use_diffing: true,         // Optimize updates
        use_alternate_screen: true, // Separate screen
    });
app.run(MyComponent)?;
```

## Debugging

```rust
// Disable optimizations for debugging
let mut app = App::new()?
    .render_config(RenderConfig {
        use_double_buffer: false,
        use_diffing: false,
        poll_duration_ms: 100,
    });
app.run(MyComponent)?;
```

## Performance Tips

1. Minimize state updates
2. Use topics only when needed
3. Avoid recreating large trees
4. Use `w_content`/`h_content` sparingly
5. Profile with `RenderConfig`

## Common Gotchas

1. **Focus required**: Most events need `focusable`
2. **State cloning**: State is cloned on update
3. **Topic ownership**: First updater owns topic
4. **Scrolling**: Container must be `focusable`

## Import Everything

```rust
use rxtui::prelude::*;
```

Includes:
- Core: `App`, `Context`, `Component`, `Node`, `Action`
- State: `State`, `Message`
- Style: `Color`, `Style`, `Direction`, `Spacing`, `Border`, `BorderStyle`
- Macros: `node!`, `#[component]`, `#[update]`, `#[view]`, `#[effect]`
- Components: `TextInput`
- Keys: `Key`, `KeyWithModifiers`
