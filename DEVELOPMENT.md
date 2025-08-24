# Development Guide

This guide will help you set up your development environment for working on RxTUI.

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git
- A terminal emulator with Unicode support

## Setting Up Your Development Environment

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/rxtui.git
cd rxtui
```

### 2. Install Dependencies

```bash
cargo build
```

This will download and compile all dependencies.

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### 4. Build Examples

```bash
# Build all examples
cargo build --examples

# Run specific example
cargo run --example demo
```

## Project Structure

```
rxtui/
├── rxtui/               # Main library crate
│   ├── lib/            # Library source code
│   │   ├── app/        # Application core
│   │   ├── components/ # Built-in components
│   │   ├── macros/     # Macro implementations
│   │   └── ...
│   ├── examples/       # Example applications
│   └── tests/          # Integration tests
├── rxtui-macros/       # Proc macro crate
├── docs/               # Documentation
└── examples/           # Standalone examples
```

## Development Workflow

### Running in Development Mode

For faster iteration during development:

```bash
# Watch for changes and rebuild
cargo watch -x build

# Run tests on file change
cargo watch -x test

# Run specific example on change
cargo watch -x "run --example demo"
```

### Debugging

Enable debug output:

```rust
// In your app configuration
let app = App::new()?
    .render_config(RenderConfig {
        use_double_buffer: false,  // Disable for debugging
        use_diffing: false,        // See all renders
        poll_duration_ms: 100,     // Slower polling
    });
```

### Performance Profiling

```bash
# Build with release optimizations but keep debug symbols
cargo build --release --features debug

# Profile with your favorite tool
# Example with perf on Linux:
perf record --call-graph=dwarf cargo run --release --example demo
perf report
```

## Common Development Tasks

### Adding a New Component

1. Create component file in `rxtui/lib/components/`
2. Implement the Component trait
3. Add to `mod.rs` exports
4. Write tests in component file
5. Add example usage

### Modifying the node! Macro

1. Edit `rxtui/lib/macros/node.rs`
2. Test with `cargo test macro_tests`
3. Update documentation if syntax changes
4. Add examples of new syntax

### Adding Event Handlers

1. Define event in `rxtui/lib/app/events.rs`
2. Add handler parsing in macro
3. Implement event dispatch
4. Write tests for new events

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

Place in `rxtui/tests/` directory:

```rust
use rxtui::prelude::*;

#[test]
fn test_complete_flow() {
    // Test complete user flow
}
```

### Visual Tests

For testing rendered output:

```rust
#[test]
fn test_rendering() {
    let buffer = TestBuffer::new(80, 24);
    // Render and assert buffer contents
}
```

## Code Quality

### Before Committing

Run these checks:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Run tests
cargo test

# Check documentation
cargo doc --no-deps --open
```

### Continuous Integration

Our CI runs:
- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo doc`

## Troubleshooting

### Common Issues

**Terminal doesn't display correctly**
- Ensure your terminal supports Unicode
- Check TERM environment variable
- Try different terminal emulator

**Tests fail with display issues**
- Tests should use headless mode
- Mock terminal for testing

**Performance issues**
- Enable optimizations: `cargo build --release`
- Profile to find bottlenecks
- Check render configuration

## Getting Help

- Open an issue on GitHub
- Join our community discussions
- Check existing issues for solutions

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Run full test suite
4. Create git tag
5. Push to trigger CI release

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Terminal Escape Sequences](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [crossterm Documentation](https://docs.rs/crossterm/)
