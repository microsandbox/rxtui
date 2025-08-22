# TextInput Deletion Operations Test Guide

## Keyboard Shortcuts for Deletion

### Word Deletion
- **Ctrl+W**: Delete word backward (to previous word boundary)
- **Alt+Backspace**: Delete word backward (alternative)
- **Alt+D**: Delete word forward (to next word boundary)

### Line Deletion
- **Ctrl+U**: Delete from cursor to beginning of line
- **Ctrl+K**: Delete from cursor to end of line

### Character Deletion (existing)
- **Backspace**: Delete character before cursor
- **Delete**: Delete character after cursor

## Test Scenarios

1. **Test Word Deletion Backward (Ctrl+W)**
   - Type: "hello world test"
   - Place cursor at end
   - Press Ctrl+W
   - Expected: "hello world " (deletes "test")

2. **Test Word Deletion Forward (Alt+D)**
   - Type: "hello world test"
   - Place cursor at beginning
   - Press Alt+D
   - Expected: " world test" (deletes "hello")

3. **Test Delete to Line Start (Ctrl+U)**
   - Type: "hello world test"
   - Place cursor after "world"
   - Press Ctrl+U
   - Expected: " test" (deletes "hello world")

4. **Test Delete to Line End (Ctrl+K)**
   - Type: "hello world test"
   - Place cursor after "hello"
   - Press Ctrl+K
   - Expected: "hello" (deletes " world test")

## Running the Test
```bash
cargo run --example text_input
```

Then test each keybinding with sample text to verify the deletion operations work correctly.
