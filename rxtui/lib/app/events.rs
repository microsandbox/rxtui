use crate::key::{Key, KeyWithModifiers};
use crate::render_tree::RenderNode;
use crate::vdom::VDom;
use crossterm::event::{KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::cell::RefCell;
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Processes keyboard input events.
///
/// Handles Tab/Shift+Tab for focus navigation, Enter to activate focused elements,
/// broadcasts to global handlers,
/// then routes other keys to the focused element.
pub fn handle_key_event(vdom: &VDom, key_event: KeyEvent) {
    // Try to create both simple key and key with modifiers
    if let Some(key) = Key::from_key_code(key_event.code) {
        let render_tree = vdom.get_render_tree();

        // Handle Tab/BackTab navigation for focus switching
        if key == Key::Tab {
            render_tree.focus_next();
            return;
        }
        if key == Key::BackTab {
            render_tree.focus_prev();
            return;
        }

        // Handle Enter to activate focused element
        if key == Key::Enter
            && let Some(focused) = render_tree.get_focused_node()
        {
            // Only simulate click if the element actually has a click handler
            // This allows elements like TextInput to handle Enter as a regular key
            if focused.borrow().events.on_click.is_some() {
                focused.borrow().handle_click();
                // Return immediately to prevent Enter from being handled again
                // The click simulation takes precedence
                return;
            }
            // If no click handler, let Enter continue to be processed as a normal key
        }

        // Create KeyWithModifiers for handlers that need it
        if let Some(key_with_modifiers) = KeyWithModifiers::from_key_event(key_event) {
            // Phase 1: Always broadcast to global handlers
            if let Some(root) = &render_tree.root {
                // Check modifier handlers FIRST (more specific)
                broadcast_global_key_with_modifiers(root, key_with_modifiers);
                // Then simple key handlers (less specific)
                broadcast_global_key(root, key);
            }

            // Phase 2: Route to focused element for non-global handlers
            if let Some(focused) = render_tree.get_focused_node() {
                // Handle scroll navigation for scrollable focused elements
                let mut handled = false;
                if focused.borrow().scrollable && focused.borrow().focused {
                    handled = handle_scroll_key(&focused, key);
                }

                if !handled {
                    // Check modifier handlers FIRST (more specific)
                    focused
                        .borrow()
                        .handle_key_with_modifiers(key_with_modifiers);
                    // Only handle simple key if modifiers weren't pressed
                    // This prevents Ctrl+A from also triggering 'a' handler
                    if !key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && !key_event.modifiers.contains(KeyModifiers::ALT)
                        && !key_event.modifiers.contains(KeyModifiers::META)
                    {
                        focused.borrow().handle_key(key);
                    }
                }
            } else {
                // No focused element, broadcast to all for non-global handlers
                if let Some(root) = &render_tree.root {
                    broadcast_key_with_modifiers(root, key_with_modifiers);
                    if !key_event.modifiers.contains(KeyModifiers::CONTROL)
                        && !key_event.modifiers.contains(KeyModifiers::ALT)
                        && !key_event.modifiers.contains(KeyModifiers::META)
                    {
                        broadcast_key(root, key);
                    }
                }
            }
        } else {
            // Fallback to simple key handling if modifier extraction fails
            // Phase 1: Always broadcast to global handlers
            if let Some(root) = &render_tree.root {
                broadcast_global_key(root, key);
            }

            // Phase 2: Route to focused element for non-global handlers
            if let Some(focused) = render_tree.get_focused_node() {
                focused.borrow().handle_key(key);
            } else {
                // No focused element, broadcast to all for non-global handlers
                if let Some(root) = &render_tree.root {
                    broadcast_key(root, key);
                }
            }
        }
    }
}

/// Recursively broadcasts a key press to all nodes in the subtree.
///
/// Each node's non-global key handler is called.
pub fn broadcast_key(node: &Rc<RefCell<RenderNode>>, key: Key) {
    let node_ref = node.borrow();
    node_ref.handle_key(key);
    for child in &node_ref.children {
        broadcast_key(child, key);
    }
}

/// Recursively broadcasts a key press to global handlers in all nodes.
///
/// Global handlers work regardless of focus state.
pub fn broadcast_global_key(node: &Rc<RefCell<RenderNode>>, key: Key) {
    let node_ref = node.borrow();
    node_ref.handle_global_key(key);
    let children = node_ref.children.clone();
    drop(node_ref); // Release borrow before recursing
    for child in &children {
        broadcast_global_key(child, key);
    }
}

/// Recursively broadcasts a key press with modifiers to all nodes in the subtree.
///
/// Each node's non-global key with modifiers handler is called.
pub fn broadcast_key_with_modifiers(
    node: &Rc<RefCell<RenderNode>>,
    key_with_modifiers: KeyWithModifiers,
) {
    let node_ref = node.borrow();
    node_ref.handle_key_with_modifiers(key_with_modifiers);
    for child in &node_ref.children {
        broadcast_key_with_modifiers(child, key_with_modifiers);
    }
}

/// Recursively broadcasts a key press with modifiers to global handlers in all nodes.
///
/// Global handlers work regardless of focus state.
pub fn broadcast_global_key_with_modifiers(
    node: &Rc<RefCell<RenderNode>>,
    key_with_modifiers: KeyWithModifiers,
) {
    let node_ref = node.borrow();
    node_ref.handle_global_key_with_modifiers(key_with_modifiers);
    let children = node_ref.children.clone();
    drop(node_ref); // Release borrow before recursing
    for child in &children {
        broadcast_global_key_with_modifiers(child, key_with_modifiers);
    }
}

/// Processes mouse input events.
///
/// Handles:
/// - Mouse down events by finding the node at the click position
/// - Sets focus to the clicked node if it's focusable
/// - Triggers the node's click handler
/// - Mouse wheel events for scrolling
pub fn handle_mouse_event(vdom: &VDom, mouse_event: MouseEvent) {
    let render_tree = vdom.get_render_tree();

    match mouse_event.kind {
        MouseEventKind::Down(_) => {
            if let Some(node) = render_tree.find_node_at(mouse_event.column, mouse_event.row) {
                render_tree.set_hovered_node(Some(node.clone()));
                // Set focus if the node is focusable
                {
                    let node_ref = node.borrow();
                    if node_ref.focusable {
                        drop(node_ref); // Release borrow before setting focus
                        render_tree.set_focused_node(Some(node.clone()));
                    }
                }

                // Handle the click
                node.borrow().handle_click();
            } else {
                render_tree.set_hovered_node(None);
            }
        }
        MouseEventKind::ScrollUp => {
            // Find the scrollable node at the mouse position
            if let Some(node) = render_tree.find_node_at(mouse_event.column, mouse_event.row) {
                render_tree.set_hovered_node(Some(node.clone()));
                // Find the nearest scrollable ancestor (including self)
                if let Some(scrollable_node) = find_scrollable_ancestor(&node) {
                    let mut node_ref = scrollable_node.borrow_mut();
                    if node_ref.update_scroll(-3) {
                        // Mark dirty if scroll position changed
                        node_ref.mark_dirty();
                    }
                }
            } else {
                render_tree.set_hovered_node(None);
            }
        }
        MouseEventKind::ScrollDown => {
            // Find the scrollable node at the mouse position
            if let Some(node) = render_tree.find_node_at(mouse_event.column, mouse_event.row) {
                render_tree.set_hovered_node(Some(node.clone()));
                // Find the nearest scrollable ancestor (including self)
                if let Some(scrollable_node) = find_scrollable_ancestor(&node) {
                    let mut node_ref = scrollable_node.borrow_mut();
                    if node_ref.update_scroll(3) {
                        // Mark dirty if scroll position changed
                        node_ref.mark_dirty();
                    }
                }
            } else {
                render_tree.set_hovered_node(None);
            }
        }
        MouseEventKind::Moved | MouseEventKind::Drag(_) => {
            let hovered = render_tree.find_node_at(mouse_event.column, mouse_event.row);
            render_tree.set_hovered_node(hovered);
        }
        MouseEventKind::Up(_) => {
            let hovered = render_tree.find_node_at(mouse_event.column, mouse_event.row);
            render_tree.set_hovered_node(hovered);
        }
        _ => {}
    }
}

/// Finds the nearest scrollable ancestor of a node (including the node itself).
fn find_scrollable_ancestor(node: &Rc<RefCell<RenderNode>>) -> Option<Rc<RefCell<RenderNode>>> {
    // Check if this node is scrollable
    if node.borrow().scrollable {
        return Some(node.clone());
    }

    // Check parent nodes
    let parent_weak = node.borrow().parent.clone();
    if let Some(parent_weak) = parent_weak
        && let Some(parent) = parent_weak.upgrade()
    {
        return find_scrollable_ancestor(&parent);
    }

    None
}

/// Handles keyboard scrolling for a scrollable node.
///
/// Returns true if the key was handled for scrolling.
fn handle_scroll_key(node: &Rc<RefCell<RenderNode>>, key: Key) -> bool {
    let mut node_ref = node.borrow_mut();
    if !node_ref.scrollable {
        return false;
    }

    match key {
        Key::Up => {
            if node_ref.update_scroll(-1) {
                node_ref.mark_dirty();
                return true;
            }
        }
        Key::Down => {
            if node_ref.update_scroll(1) {
                node_ref.mark_dirty();
                return true;
            }
        }
        Key::PageUp => {
            // Scroll up by half the viewport height
            let scroll_amount = (node_ref.height / 2).max(1) as i16;
            if node_ref.update_scroll(-scroll_amount) {
                node_ref.mark_dirty();
                return true;
            }
        }
        Key::PageDown => {
            // Scroll down by half the viewport height
            let scroll_amount = (node_ref.height / 2).max(1) as i16;
            if node_ref.update_scroll(scroll_amount) {
                node_ref.mark_dirty();
                return true;
            }
        }
        Key::Home => {
            // Scroll to top
            node_ref.set_scroll_y(0);
            node_ref.mark_dirty();
            return true;
        }
        Key::End => {
            // Scroll to bottom
            let max_y = node_ref.get_max_scroll_y();
            node_ref.set_scroll_y(max_y);
            node_ref.mark_dirty();
            return true;
        }
        _ => {}
    }

    false
}
