# RxTUI - Implementation Details

## Overview

RxTUI is a reactive terminal user interface framework inspired by React's component model. It provides a declarative, component-based API for building interactive terminal applications with efficient rendering through virtual DOM diffing and advanced cross-component communication via topic-based messaging.

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                     Component System                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │  Components  │  │   Messages   │  │    Topics    │   │
│  │  - update()  │  │  - Direct    │  │  - Ownership │   │
│  │  - view()    │  │  - Topic     │  │  - Broadcast │   │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘   │
│         │                 │                 │           │
│  ┌──────▼─────────────────▼─────────────────▼────────┐  │
│  │                     Context                       │  │
│  │  - StateMap: Component state storage              │  │
│  │  - Dispatcher: Message routing                    │  │
│  │  - TopicStore: Topic ownership & state            │  │
│  └──────────────────────┬────────────────────────────┘  │
└─────────────────────────┼───────────────────────────────┘
                          │
┌─────────────────────────▼──────────────────────────────┐
│                    Rendering Pipeline                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │     Node     │──│     VNode    │──│  RenderNode  │  │
│  │  (Component) │  │  (Virtual)   │  │ (Positioned) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                 │          │
│  ┌──────▼─────────────────▼─────────────────▼───────┐  │
│  │                   Virtual DOM (VDom)             │  │
│  │  - Diff: Compare old and new trees               │  │
│  │  - Patch: Generate minimal updates               │  │
│  │  - Layout: Calculate positions and sizes         │  │
│  └──────────────────────┬───────────────────────────┘  │
└─────────────────────────┼──────────────────────────────┘
                          │
┌─────────────────────────▼──────────────────────────────┐
│                     Terminal Output                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │Double Buffer │  │Cell Diffing  │  │  Optimized   │  │
│  │  Front/Back  │  │   Updates    │  │   Renderer   │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Component System (`lib/component.rs`)

The component system is the heart of the framework, providing a React-like component model with state management and message passing.

#### Component Trait

```rust
pub trait Component: 'static {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action;
    fn view(&self, ctx: &Context) -> Node;
    fn get_id(&self) -> Option<ComponentId>;
    fn set_id(&mut self, id: ComponentId);
}
```

**Key Design Decisions:**
- Components are stateless - all state is managed by Context
- Update method receives optional topic for cross-component messaging
- Components can be derived using `#[derive(Component)]` macro

#### Message and State Traits

Both Message and State traits are auto-implemented for any type that is `Clone + Send + 'static`:

```rust
pub trait Message: Any + Send + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Message>;
}

pub trait State: Any + Send + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn State>;
}
```

**Extension Traits for Downcasting:**
- `MessageExt` provides `downcast<T>()` for message type checking
- `StateExt` provides `downcast<T>()` for state type checking

#### Actions

Components return actions from their update method:

```rust
pub enum Action {
    Update(Box<dyn State>),              // Update component's local state
    UpdateTopic(String, Box<dyn State>), // Update topic state (first writer owns)
    None,                                // No action needed
    Exit,                                // Exit the application
}
```

### 2. Context System (`lib/app/context.rs`)

The Context provides components with everything they need to function:

#### Core Components

**Context Structure:**
- `current_component_id`: Component being processed
- `dispatch`: Message dispatcher
- `states`: Component state storage (StateMap)
- `topics`: Topic-based messaging (TopicStore)
- `message_queues`: Regular message queues
- `topic_message_queues`: Topic message queues

**StateMap:**
- Stores component states with interior mutability
- `get_or_init<T>()`: Get state or initialize with Default
- Type-safe state retrieval with automatic downcasting

**Dispatcher:**
- Routes messages to components or topics
- `send(component_id, message)`: Direct component messaging
- `send_to_topic(topic, message)`: Topic-based messaging

**TopicStore:**
- Manages topic ownership (first writer becomes owner)
- Stores topic states separately from component states
- Tracks which component owns which topic

#### Message Flow

1. **Direct Messages**: Sent to specific component via `ctx.handler(msg)`
2. **Topic Messages**: Sent via `ctx.send_to_topic(topic, msg)`
   - If topic has owner → delivered only to owner
   - If no owner → cloned to all components until one claims it

### 3. Topic-Based Messaging System

A unique feature for cross-component communication without direct references:

#### Concepts

- **Topics**: Named channels for messages (e.g., "counter_a", "global_state")
- **Ownership**: First component to write to a topic becomes its owner
- **Unassigned Messages**: Messages to unclaimed topics are cloned to all components

#### How It Works

1. **Sending Messages:**
   ```rust
   ctx.send_to_topic("my-topic", Box::new(MyMessage));
   ```

2. **Claiming Ownership:**
   ```rust
   // First component to return this action owns the topic
   Action::UpdateTopic("my-topic".into(), Box::new(MyState))
   ```

3. **Handling Topic Messages (Using Macros):**
   ```rust
   // With the new #[update] macro:
   #[update(
       msg = MyMsg,
       topics = ["my-topic" => TopicMsg]
   )]
   fn update(&self, ctx: &Context, messages: Messages) -> Action {
       match messages {
           Messages::MyMsg(msg) => { /* handle regular message */ }
           Messages::TopicMsg(msg) => { /* handle topic message */ }
       }
   }

   // Or manually without macro:
   fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
       if let Some(topic) = topic {
           if topic == "my-topic" {
               // Handle message for this topic
           }
       }
   }
   ```

4. **Reading Topic State:**
   ```rust
   let state: MyState = ctx.read_topic("my-topic")?;
   ```

**Design Rationale:**
- Enables decoupled component communication
- Supports both single-writer/multiple-reader and multiple-writer/single-reader patterns
- Automatic ownership management prevents conflicts

### 4. Application Core (`lib/app/core.rs`)

The App struct manages the entire application lifecycle:

#### Initialization
```rust
App::new()
```
- Enables terminal raw mode and alternate screen
- Hides cursor and enables mouse capture
- Initializes double buffer for flicker-free rendering
- Sets up event handling

#### Event Loop

The main loop (`run_loop`) follows this sequence:

1. **Message Processing**:
   - Components drain all pending messages (regular + topic)
   - Messages trigger state updates via component's `update` method

2. **Tree Expansion**:
   - Root component's `view` method generates Node tree
   - Child components are recursively expanded
   - Component IDs are assigned based on tree position

3. **Virtual DOM Update**:
   - VDom diffs new tree against current
   - Generates patches for changes
   - Updates render tree

4. **Layout & Rendering**:
   - Calculate positions and sizes
   - Render to back buffer
   - Diff buffers and apply changes to terminal

5. **Event Handling**:
   - Process keyboard/mouse events
   - Events trigger new messages
   - Loop continues

#### Component Tree Expansion

The `expand_component_tree` method is crucial:

1. Drains all messages for the component
2. Processes each message:
   - Regular messages → component's update
   - Topic messages → check if component handles topic
3. Handles actions:
   - `Update` → update component state
   - `UpdateTopic` → update topic state, claim ownership
   - `Exit` → propagate exit signal
4. Calls component's `view` to get UI tree
5. Recursively expands child components

### 5. Node Types

Three levels of node representation:

#### Node (`lib/node.rs`)
High-level component tree:
```rust
pub enum Node {
    Component(Box<dyn Component>), // Component instance
    Div(Div),                      // Div with children
    Text(Text),                    // Text content
    RichText(RichText),           // Styled text with multiple spans
}
```

#### VNode (`lib/vnode.rs`)
Virtual DOM nodes after component expansion:
```rust
pub enum VNode {
    Div(Div),              // Expanded div
    Text(Text),           // Text node
    RichText(RichText),   // Rich text node
}
```

#### RenderNode (`lib/render_tree/node.rs`)
Positioned nodes ready for drawing:
```rust
pub struct RenderNode {
    pub node_type: RenderNodeType,
    pub x: u16, pub y: u16,           // Position
    pub width: u16, pub height: u16,   // Size
    pub style: Option<Style>,          // Visual style
    pub children: Vec<Rc<RefCell<RenderNode>>>,
    pub parent: Option<Weak<RefCell<RenderNode>>>,
    pub focusable: bool,
    pub focused: bool,
    pub dirty: bool,
    // ... event handlers, z-index, etc.
}
```

### 6. Div System (`lib/div.rs`)

Divs are the building blocks of the UI:

#### Properties
- **Layout**: Direction, gap, wrap mode
- **Sizing**: Width/height (Fixed, Percentage, Auto, Content)
- **Styling**: Background, padding, borders, overflow
- **Focus**: Focusable flag, focus styles
- **Events**: Click/key handlers (global and local)

#### Builder Pattern
```rust
Div::new()
    .background(Color::Blue)
    .padding(Spacing::all(2))
    .direction(Direction::Horizontal)
    .width(20)
    .height_percent(0.5)
    .focusable(true)
    .focus_style(Style::default().background(Color::White))
    .on_click(ctx.handler(MyMsg::Click))
    .on_key(Key::Enter, ctx.handler(MyMsg::Enter))
    .on_char_global('q', ctx.handler(MyMsg::Quit))
    .children(vec![...])
```

### 7. Virtual DOM (`lib/vdom.rs`)

Manages UI state and efficient updates:

#### Core Operations

1. **Render**: Accept new VNode tree
2. **Diff**: Compare with current tree
3. **Patch**: Apply changes to render tree
4. **Layout**: Calculate positions
5. **Draw**: Output to terminal

#### Diffing Algorithm (`lib/diff.rs`)

Generates minimal patches:
```rust
pub enum Patch {
    Create(VNode, Vec<usize>),           // Add node at path
    Delete(Vec<usize>),                  // Remove node
    Replace(VNode, Vec<usize>),          // Replace subtree
    UpdateDiv(Div, Vec<usize>),      // Update properties
    UpdateText(Text, Vec<usize>),        // Update text
    Move(Vec<usize>, Vec<usize>),        // Reorder children
}
```

### 8. Layout System (`lib/render_tree/`)

Sophisticated layout engine supporting multiple sizing modes:

#### Dimension Types
```rust
pub enum Dimension {
    Fixed(u16),       // Exact size in cells
    Percentage(f32),  // Percentage of parent (0.0-1.0)
    Auto,            // Share remaining space equally
    Content,         // Size based on children
}
```

#### Layout Algorithm

1. **Fixed**: Use exact size
2. **Percentage**: Calculate from parent size
3. **Content**:
   - Horizontal: width = sum of children, height = max child
   - Vertical: width = max child, height = sum of children
4. **Auto**: Divide remaining space equally among auto-sized elements

#### Text Wrapping
Multiple wrapping modes supported:
- `None`: No wrapping
- `Character`: Break at any character
- `Word`: Break at word boundaries
- `WordBreak`: Try words, break if necessary

### 9. Rendering Pipeline (`lib/app/renderer.rs`)

Converts render tree to terminal output:

#### Rendering Steps

1. **Clear Background**: Fill with parent background color
2. **Draw Borders**: Render border characters if present
3. **Apply Padding**: Adjust content area
4. **Render Content**:
   - For containers: Recurse into children
   - For text: Draw text with wrapping
5. **Apply Clipping**: Ensure content stays within bounds

#### Style Inheritance
- Text nodes inherit parent's background if not specified
- Focus styles override normal styles
- Children can override parent styles

### 10. Terminal Output System

#### Double Buffering (`lib/buffer.rs`)

Eliminates flicker completely:

```rust
pub struct DoubleBuffer {
    front_buffer: ScreenBuffer,  // Currently displayed
    back_buffer: ScreenBuffer,   // Next frame
    width: u16,
    height: u16,
}
```

**Cell Structure:**
```rust
pub struct Cell {
    pub char: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}
```

**Diff Process:**
1. Render to back buffer
2. Compare with front buffer cell-by-cell
3. Generate list of changed cells
4. Apply updates to terminal
5. Swap buffers

#### Terminal Renderer (`lib/terminal.rs`)

Optimized output with multiple strategies:

1. **Batch Updates**: Group cells with same colors
2. **Skip Unchanged**: Only update modified cells
3. **Optimize Movements**: Minimize cursor jumps
4. **Run-Length Encoding**: Compress repeated characters

### 11. Event System (`lib/app/events.rs`)

Comprehensive input handling:

#### Keyboard Events

**Focus Navigation:**
- Tab: Next focusable element
- Shift+Tab: Previous focusable element

**Event Routing:**
1. Global handlers always receive events
2. Focused element receives local events
3. Character and key handlers triggered

#### Mouse Events

**Click Handling:**
1. Find node at click position
2. Set focus if focusable
3. Trigger click handler

### 12. RichText System (`lib/node/rich_text.rs`)

Provides inline text styling with multiple spans:

#### Core Structure
```rust
pub struct RichText {
    pub spans: Vec<TextSpan>,
    pub style: Option<TextStyle>,  // Top-level style for wrapping, etc.
}

pub struct TextSpan {
    pub content: String,
    pub style: Option<TextStyle>,
}
```

#### Builder API
```rust
RichText::new()
    .text("Normal text ")
    .colored("red text", Color::Red)
    .bold("bold text")
    .italic("italic text")
    .styled("custom", TextStyle { ... })
    .wrap(TextWrap::Word)
```

#### Features
- **Multiple Spans**: Each span can have different styling
- **Top-Level Styling**: Apply wrapping or common styles to all spans
- **Helper Methods**: `bold_all()`, `color()` for all spans
- **Text Wrapping**: Preserves span styles across wrapped lines
- **Internal Cursor Support**: Used by TextInput component

### 13. Style System (`lib/style.rs`)

Rich styling capabilities:

#### Colors
- 16 standard terminal colors
- Bright variants
- RGB support (24-bit color)

#### Text Styles
```rust
bitflags! {
    pub struct TextStyle: u8 {
        const BOLD = 0b00000001;
        const ITALIC = 0b00000010;
        const UNDERLINE = 0b00000100;
        const STRIKETHROUGH = 0b00001000;
        const DIM = 0b00010000;
        const BLINK_SLOW = 0b00100000;
        const BLINK_RAPID = 0b01000000;
        const REVERSE = 0b10000000;
    }
}
```

#### Borders
Multiple styles supported:
- Single, Double, Rounded, Thick
- Configurable edges (top, right, bottom, left)

#### Focus System
```rust
pub struct Style {
    pub background: Option<Color>,
    pub padding: Option<Spacing>,
    pub border: Option<BorderStyle>,
    pub border_color: Option<Color>,
    pub border_edges: BorderEdges,
    pub overflow: Overflow,
    pub opacity: f32,
}
```

## Performance Optimizations

### 1. Virtual DOM
- Minimal patch generation
- Short-circuit unchanged subtrees
- Efficient path-based updates

### 2. Double Buffering
- Zero flicker guaranteed
- Cell-level diffing
- Only changed cells updated

### 3. Terminal Renderer
- Batch color changes
- Optimize cursor movements
- Run-length encoding
- Skip unchanged regions

### 4. Message System
- Zero-copy message routing
- Lazy state cloning
- Efficient topic distribution

### 5. Memory Management
- Rc/RefCell for shared ownership
- Weak references prevent cycles
- Minimal allocations during render

## Example: Complete Application

```rust
use rxtui::prelude::*;

// Messages
#[derive(Debug, Clone)]
enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Debug, Clone)]
struct ResetSignal;

// State
#[derive(Debug, Clone, Default)]
struct CounterState {
    count: i32,
}

// Component
#[derive(Component, Clone)]
struct Counter {
    topic_name: String,
    label: String,
}

impl Counter {
    fn new(topic: &str, label: &str) -> Self {
        Self {
            topic_name: topic.into(),
            label: label.into(),
        }
    }

    // Using the new #[update] macro with dynamic topic support
    #[update(
        msg = CounterMsg,
        topics = [self.topic_name => ResetSignal]
    )]
    fn update(&self, ctx: &Context, messages: Messages, mut state: CounterState) -> Action {
        match messages {
            Messages::CounterMsg(msg) => {
                match msg {
                    CounterMsg::Increment => state.count += 1,
                    CounterMsg::Decrement => state.count -= 1,
                }
                Action::Update(Box::new(state))
            }
            Messages::ResetSignal(_) => {
                Action::Update(Box::new(CounterState::default()))
            }
        }
    }

    // Using the new #[view] macro
    #[view]
    fn view(&self, ctx: &Context, state: CounterState) -> Node {

        Div::new()
            .background(Color::Blue)
            .padding(Spacing::all(1))
            .direction(Direction::Vertical)
            .focusable(true)
            .focus_style(Style::default().background(Color::Cyan))
            .on_key(Key::Up, ctx.handler(CounterMsg::Increment))
            .on_key(Key::Down, ctx.handler(CounterMsg::Decrement))
            .children(vec![
                Text::new(&self.label).color(Color::White).into(),
                Text::new(format!("Count: {}", state.count))
                    .color(Color::BrightWhite)
                    .into(),
                Div::new()
                    .direction(Direction::Horizontal)
                    .gap(2)
                    .children(vec![
                        Div::new()
                            .background(Color::Green)
                            .padding(Spacing::horizontal(1))
                            .on_click(ctx.handler(CounterMsg::Increment))
                            .children(vec![Text::new("+").into()])
                            .into(),
                        Div::new()
                            .background(Color::Red)
                            .padding(Spacing::horizontal(1))
                            .on_click(ctx.handler(CounterMsg::Decrement))
                            .children(vec![Text::new("-").into()])
                            .into(),
                    ])
                    .into(),
            ])
            .into()
    }
}

// Dashboard with reset button
#[derive(Component, Clone, Default)]
struct Dashboard {}

impl Dashboard {
    #[update]
    fn update(&self, ctx: &Context, msg: ResetSignal) -> Action {
        // Send reset to all counters via topics
        ctx.send_to_topic("counter_a", Box::new(ResetSignal));
        ctx.send_to_topic("counter_b", Box::new(ResetSignal));
        Action::None
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        Div::new()
            .padding(Spacing::all(2))
            .direction(Direction::Vertical)
            .on_char_global('q', ctx.handler(ExitSignal))
            .on_char_global('r', ctx.handler(ResetSignal))
            .children(vec![
                Text::new("Dashboard - Press 'r' to reset, 'q' to quit")
                    .color(Color::Yellow)
                    .into(),
                Div::new()
                    .direction(Direction::Horizontal)
                    .gap(2)
                    .children(vec![
                        Node::Component(Box::new(Counter::new("counter_a", "Counter A"))),
                        Node::Component(Box::new(Counter::new("counter_b", "Counter B"))),
                    ])
                    .into(),
            ])
            .into()
    }
}

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(Dashboard::default())
}
```

## Testing Strategy

### Unit Tests
- Component update logic
- State management
- Message routing
- Layout calculations

### Integration Tests
- Full rendering pipeline
- Event handling
- Topic messaging
- Focus navigation

### Visual Tests
- Manual testing with examples
- Screenshot comparisons
- Terminal compatibility

## Future Roadmap

### Short Term
- [ ] Input components (text fields, checkboxes)
- [ ] List virtualization for large datasets
- [ ] Scrollable containers
- [ ] More border styles

### Medium Term
- [ ] Animation system
- [ ] Flexbox-like layout
- [ ] Grid layout
- [ ] Theme system
- [ ] Hot reload for development

### Long Term
- [ ] Accessibility (screen reader support)
- [ ] Plugin system
- [ ] Visual designer
- [ ] Cross-platform GUI support
- [ ] Web assembly target

## Design Principles

1. **Type Safety**: Leverage Rust's type system for compile-time guarantees
2. **Performance**: Zero-cost abstractions, minimal allocations
3. **Ergonomics**: Intuitive builder APIs, derive macros
4. **Flexibility**: Composable components, extensible architecture
5. **Reliability**: No panics in production code, graceful degradation

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Code style and organization
- Testing requirements
- Documentation standards
- Pull request process
