# RxTUI - Implementation Details

## Overview

RxTUI is a reactive terminal user interface framework inspired by React's component model. It provides a declarative, component-based API for building interactive terminal applications with efficient rendering through virtual DOM diffing and advanced cross-component communication via topic-based messaging.

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                     Component System                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Components  │  │   Messages   │  │    Topics    │  │
│  │  - update()  │  │  - Direct    │  │  - Ownership │  │
│  │  - view()    │  │  - Topic     │  │  - Broadcast │  │
│  │  - effects() │  │  - Async     │  │  - State     │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
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
    fn effects(&self, ctx: &Context) -> Vec<Effect>;  // Optional, requires "effects" feature
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn Component>;
}
```

**Key Design Decisions:**
- Components are stateless - all state is managed by Context
- Update method receives optional topic for cross-component messaging
- Components can be derived using `#[derive(Component)]` macro
- Components must implement Clone for tree manipulation
- Effects support async background tasks (with feature flag)

#### Message and State Traits

Both Message and State traits are auto-implemented for any type that is `Clone + Send + Sync + 'static`:

```rust
pub trait Message: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Message>;
}

pub trait State: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
- Stores component states with interior mutability using `Arc<RwLock<HashMap>>`
- `get_or_init<T>()`: Get state or initialize with Default
- Type-safe state retrieval with automatic downcasting

**Dispatcher:**
- Routes messages to components or topics
- `send_to_id(component_id, message)`: Direct component messaging
- `send_to_topic(topic, message)`: Topic-based messaging

**TopicStore:**
- Manages topic ownership (first writer becomes owner)
- Stores topic states separately from component states
- Tracks which component owns which topic
- Thread-safe with RwLock protection

#### Message Flow

1. **Direct Messages**: Sent to specific component via `ctx.handler(msg)`
2. **Topic Messages**: Sent via `ctx.send_to_topic(topic, msg)`
   - If topic has owner → delivered only to owner
   - If no owner → broadcast to all components until one claims it

### 3. Topic-Based Messaging System

A unique feature for cross-component communication without direct references:

#### Concepts

- **Topics**: Named channels for messages (e.g., "counter_a", "global_state")
- **Ownership**: First component to write to a topic becomes its owner
- **Unassigned Messages**: Messages to unclaimed topics are broadcast to all components

#### How It Works

1. **Sending Messages:**
   ```rust
   ctx.send_to_topic("my-topic", MyMessage);
   ```

2. **Claiming Ownership:**
   ```rust
   // First component to return this action owns the topic
   Action::UpdateTopic("my-topic", MyState)
   ```

3. **Handling Topic Messages (Using Macros):**
   ```rust
   // With the new #[update] macro:
   #[update(msg = MyMsg, topics = ["my-topic" => TopicMsg])]
   fn update(&self, ctx: &Context, messages: Messages) -> Action {
       match messages {
           Messages::MyMsg(msg) => { /* handle regular message */ }
           Messages::TopicMsg(msg) => { /* handle topic message */ }
       }
   }
   ```

4. **Reading Topic State:**
   ```rust
   let state: Option<MyState> = ctx.read_topic("my-topic");
   ```

**Design Rationale:**
- Enables decoupled component communication
- Supports both single-writer/multiple-reader and broadcast patterns
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
- Creates effect runtime (if feature enabled)

#### Event Loop

The main loop (`run_loop`) follows this sequence:

1. **Component Tree Expansion**:
   - Start with root component
   - Recursively expand components to VNodes
   - Assign component IDs based on tree position

2. **Message Processing**:
   - Components drain all pending messages (regular + topic)
   - Messages trigger state updates via component's `update` method
   - Handle actions (Update, UpdateTopic, Exit, None)

3. **Virtual DOM Update**:
   - VDom diffs new tree against current
   - Generates patches for changes
   - Updates render tree

4. **Layout & Rendering**:
   - Calculate positions and sizes
   - Render to back buffer
   - Diff buffers and apply changes to terminal

5. **Event Handling**:
   - Process keyboard/mouse events (poll with 16ms timeout by default)
   - Events trigger new messages via event handlers
   - Handle terminal resize events

6. **Effect Management** (if feature enabled):
   - Spawn effects for newly mounted components
   - Cleanup effects for unmounted components
   - Effects run in Tokio runtime

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

#### Node (`lib/node/`)
High-level component tree:
```rust
pub enum Node {
    Component(Box<dyn Component>), // Component instance
    Div(Div),                      // Container with children
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
    pub content_width: u16,            // Actual content size
    pub content_height: u16,
    pub scroll_y: u16,                 // Vertical scroll offset
    pub scrollable: bool,              // Has overflow:scroll/auto
    pub style: Option<Style>,          // Visual style
    pub children: Vec<Rc<RefCell<RenderNode>>>,
    pub parent: Option<Weak<RefCell<RenderNode>>>,
    pub focusable: bool,
    pub focused: bool,
    pub dirty: bool,
    // ... event handlers, z-index, etc.
}
```

### 6. Div System (`lib/node/div.rs`)

Divs are the building blocks of the UI:

#### Properties
- **Layout**: Direction, gap, wrap mode
- **Sizing**: Width/height (Fixed, Percentage, Auto, Content)
- **Styling**: Background, padding, borders, overflow
- **Focus**: Focusable flag, focus styles
- **Events**: Click/key handlers (global and local)
- **Scrolling**: overflow, show_scrollbar flags

#### Builder Pattern
```rust
Div::new()
    .background(Color::Blue)
    .padding(Spacing::all(2))
    .direction(Direction::Horizontal)
    .width(20)
    .height_percent(0.5)
    .focusable(true)
    .overflow(Overflow::Scroll)
    .show_scrollbar(true)
    .on_click(handler)
    .on_key(Key::Enter, handler)
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
    Create(VNode, Vec<usize>),        // Add node at path
    Delete(Vec<usize>),               // Remove node
    Replace(VNode, Vec<usize>),       // Replace subtree
    UpdateDiv(Div, Vec<usize>),       // Update properties
    UpdateText(Text, Vec<usize>),     // Update text
    UpdateRichText(RichText, Vec<usize>), // Update rich text
    Move(Vec<usize>, Vec<usize>),     // Reorder children
}
```

### 8. Layout System (`lib/render_tree/tree.rs`)

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

#### Scrolling Support
- **Vertical scrolling**: Implemented with scroll_y offset
- **Scrollbar rendering**: Optional visual indicator
- **Keyboard navigation**: Up/Down arrows, PageUp/PageDown, Home/End
- **Mouse wheel**: ScrollUp/ScrollDown events
- **Content tracking**: content_height vs container height
- **Note**: Horizontal scrolling not yet implemented

### 9. Rendering Pipeline (`lib/app/renderer.rs`)

Converts render tree to terminal output:

#### Rendering Steps

1. **Clear Background**: Fill with parent background color
2. **Draw Borders**: Render border characters if present
3. **Apply Padding**: Adjust content area
4. **Handle Scrolling**: Apply scroll_y offset for scrollable containers
5. **Render Content**:
   - For containers: Recurse into children
   - For text: Draw text with wrapping
   - For rich text: Draw styled segments
6. **Apply Clipping**: Ensure content stays within bounds
7. **Draw Scrollbar**: Show position indicator if enabled

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
    pub style: TextStyle,  // Bitflags for bold, italic, etc.
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
4. **Style Batching**: Combine style changes

### 11. Event System (`lib/app/events.rs`)

Comprehensive input handling:

#### Keyboard Events

**Focus Navigation:**
- Tab: Next focusable element
- Shift+Tab: Previous focusable element

**Scrolling (for focused scrollable elements):**
- Up/Down arrows: Scroll by 1 line
- PageUp/PageDown: Scroll by container height
- Home/End: Jump to top/bottom

**Event Routing:**
1. Global handlers always receive events
2. Focused element receives local events
3. Character and key handlers triggered

#### Mouse Events

**Click Handling:**
1. Find node at click position
2. Set focus if focusable
3. Trigger click handler

**Scroll Handling:**
1. Find scrollable node under cursor
2. Apply scroll delta
3. Clamp to content bounds

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

### 13. Style System (`lib/style.rs`)

Rich styling capabilities:

#### Colors
- 16 standard terminal colors (Black, Red, Green, Yellow, Blue, Magenta, Cyan, White)
- Bright variants (BrightBlack through BrightWhite)
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

#### Overflow
```rust
pub enum Overflow {
    None,   // Content not clipped
    Hidden, // Content clipped at boundaries
    Scroll, // Content clipped but scrollable
    Auto,   // Auto show scrollbars
}
```

### 14. Macro System (`lib/macros/`)

Provides ergonomic APIs for building UIs:

#### node! Macro
JSX-like syntax for building UI trees:
```rust
node! {
    div(bg: blue, pad: 2) [
        text("Hello", color: white),
        div(border: white) [
            text("Nested")
        ]
    ]
}
```

#### Attribute Macros

**#[derive(Component)]**: Auto-implements Component trait boilerplate

**#[component]**: Collects #[effect] methods for async support

**#[update]**: Handles message downcasting and state management

**#[view]**: Automatically fetches component state

**#[effect]**: Marks async methods as effects

### 15. Effects System (`lib/effect/`, requires feature flag)

Supports async background tasks:

#### Effect Runtime
- Spawns Tokio runtime for async execution
- Manages effect lifecycle per component
- Automatic cleanup on component unmount

#### Effect Definition
```rust
#[component]
impl MyComponent {
    #[effect]
    async fn background_task(&self, ctx: &Context) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            ctx.send(MyMsg::Tick);
        }
    }
}
```

#### Common Use Cases
- Timers and periodic updates
- Network requests
- File system monitoring
- WebSocket connections
- Background computations

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
- Skip unchanged regions

### 4. Message System
- Zero-copy message routing where possible
- Lazy state cloning
- Efficient topic distribution

### 5. Memory Management
- Rc/RefCell for shared ownership
- Weak references prevent cycles
- Minimal allocations during render

## Configuration

### RenderConfig
Controls rendering behavior for debugging:
- `poll_duration_ms`: Event poll timeout (default 16ms)
- `use_double_buffer`: Enable/disable double buffering
- `use_diffing`: Enable/disable cell diffing
- `use_alternate_screen`: Use alternate screen buffer

## Future Enhancements

### Planned Features
- Horizontal scrolling support
- More built-in components (Button, Select, Table)
- Animation system
- Layout constraints
- Accessibility features

### Known Limitations
- Horizontal scrolling not implemented
- No built-in form validation
- Limited animation support
- No tree-shaking for unused features
