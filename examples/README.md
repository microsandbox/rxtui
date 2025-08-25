# RxTUI Examples

This directory contains example applications demonstrating various features and patterns of the RxTUI framework.

## Examples

### [counter.rs](./counter.rs)
```bash
cargo run --example counter
```

<img width="500" alt="rxtui logo" src="https://github.com/user-attachments/assets/c2038f9f-5633-4617-bbfb-569a5a89697c" />
<img width="100%" alt="stopwatch demo" src="https://github.com/user-attachments/assets/8c627f96-b188-40ce-a022-384a6a1b7e66" />
<img width="100%" alt="progressbar demo" src="https://github.com/user-attachments/assets/df860dde-98f7-4ccd-ba32-c0187e0f214b" />
<img width="100%" alt="form demo" src="https://github.com/user-attachments/assets/c0c85d9f-9231-45b7-bcf5-081fab998e79" />
<img width="100%" alt="counter demo" src="https://github.com/user-attachments/assets/904cce59-48c4-4813-9a68-d1c0b8176c4b" />
<img width="100%" alt="align demo" src="https://github.com/user-attachments/assets/c3a05029-a4fd-4ed4-a8a6-a41c76114956" />

A minimal counter demonstrating:
- Basic component structure with `#[update]` and `#[view]` macros
- State management and message handling
- Keyboard event handlers (`↑`/`↓` keys)
- The absolute minimum code needed for an RxTUI app

<br />

### [form.rs](./form.rs)
```bash
cargo run --example form
```
Demonstrates form building capabilities:
- Text input fields with focus management
- Form validation and state management
- Submit/cancel actions
- Keyboard navigation between fields
- Error display and user feedback

<br />

### [stopwatch.rs](./stopwatch.rs)
```bash
cargo run --example stopwatch
```
Time-based UI updates:
- Effects system for side effects
- Timer implementation with start/stop/reset
- Formatting time display
- Combining user actions with background updates

<br />

### [components.rs](./components.rs)
```bash
cargo run --example components
```
Shows how to build complex UIs from reusable components:
- Multiple independent counter components with different colors
- Inter-component communication via topics
- Dynamic topic names in `#[update]` macro
- Nested component structure (Dashboard → Counter components)
- Both stateful (Counter) and stateless (Dashboard) components

## Feature Showcase

### [demo.rs](./demo.rs)
```bash
cargo run --example demo
```
Multi-page demo application showcasing:
- Tab-based navigation system
- 15 different pages each demonstrating specific features
- Component communication via topics
- Complex layouts and styling
- Everything RxTUI can do in one app

The demo includes specialized pages for:
1. **Overflow** - Text overflow and truncation handling
2. **Direction** - Vertical/horizontal layouts and flow
3. **Percentages** - Percentage-based sizing
4. **Borders** - Border styles and selective edges
5. **Absolute** - Absolute positioning and modals
6. **Text Styles** - Colors, bold, underline, etc.
7. **Auto Sizing** - Content-based sizing
8. **Text Wrap** - Word wrapping and text flow
9. **Element Wrap** - Flexbox-like element wrapping
10. **Unicode** - Unicode and emoji support
11. **Content Size** - Dynamic content sizing
12. **Focus** - Focus management and keyboard navigation
13. **Rich Text** - Mixed styles within text
14. **Text Input** - Interactive text input fields
15. **Scrollable** - Scrollable regions and overflow
