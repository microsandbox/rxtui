pub mod config;
pub mod context;
pub mod core;
pub mod events;
pub mod renderer;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use config::RenderConfig;
pub use context::{Context, Dispatcher, StateMap};
pub use core::App;
pub use renderer::render_node_to_buffer;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        buffer::ScreenBuffer,
        render_tree::RenderNode,
        style::{Color, Style},
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_text_inherits_parent_background() {
        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 0;
        text_node.y = 0;
        text_node.width = 5;
        text_node.height = 1;
        // No style set - should inherit parent's background

        // Add text as child of parent
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));
        parent_rc.borrow_mut().children.push(text_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 3);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 3);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the parent's blue background
        for x in 0..5 {
            let cell = buffer.get_cell(x, 0).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Text at position {x} should have blue background"
            );
        }
    }

    #[test]
    fn test_text_own_background_takes_precedence() {
        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a text node with its own red background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 0;
        text_node.y = 0;
        text_node.width = 5;
        text_node.height = 1;
        text_node.style = Some(Style {
            background: Some(Color::Red),
            ..Default::default()
        });

        // Add text as child of parent
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));
        parent_rc.borrow_mut().children.push(text_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 3);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 3);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have their own red background, not parent's blue
        for x in 0..5 {
            let cell = buffer.get_cell(x, 0).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Text at position {x} should have red background"
            );
        }
    }

    #[test]
    fn test_multi_level_background_inheritance() {
        // Create a grandparent div with blue background
        let mut grandparent = RenderNode::element();
        grandparent.x = 0;
        grandparent.y = 0;
        grandparent.width = 15;
        grandparent.height = 5;
        grandparent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a parent div WITHOUT background
        let mut parent = RenderNode::element();
        parent.x = 1;
        parent.y = 1;
        parent.width = 10;
        parent.height = 3;
        // No background style - should inherit from grandparent

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 1;
        text_node.y = 1;
        text_node.width = 5;
        text_node.height = 1;
        // No style set - should inherit through parent from grandparent

        // Build the tree
        let grandparent_rc = Rc::new(RefCell::new(grandparent));
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));

        parent_rc.borrow_mut().children.push(text_rc);
        grandparent_rc.borrow_mut().children.push(parent_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(15, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 15, 5);
        render_node_to_buffer(&grandparent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the grandparent's blue background
        // Text is at absolute position (2, 2) due to nested positioning
        for x in 2..7 {
            let cell = buffer.get_cell(x, 2).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Text at position {x} should inherit blue background from grandparent"
            );
        }
    }

    #[test]
    fn test_border_background_inheritance() {
        use crate::style::{Border, BorderEdges, BorderStyle};

        // Create a parent div with blue background
        let mut parent = RenderNode::element();
        parent.x = 0;
        parent.y = 0;
        parent.width = 10;
        parent.height = 5;
        parent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a child div with border but no background
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::ALL,
            }),
            // No background - border should inherit parent's blue
            ..Default::default()
        });

        // Build the tree
        let parent_rc = Rc::new(RefCell::new(parent));
        let child_rc = Rc::new(RefCell::new(child));
        parent_rc.borrow_mut().children.push(child_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(10, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 10, 5);
        render_node_to_buffer(&parent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that border cells have the parent's blue background
        // Top border
        for x in 1..6 {
            let cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Top border at position {x} should have blue background"
            );
        }

        // Left border
        for y in 1..4 {
            let cell = buffer.get_cell(1, y).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Blue),
                "Left border at position y={y} should have blue background"
            );
        }
    }

    #[test]
    fn test_element_with_own_bg_overrides_inheritance() {
        // Create a grandparent div with blue background
        let mut grandparent = RenderNode::element();
        grandparent.x = 0;
        grandparent.y = 0;
        grandparent.width = 15;
        grandparent.height = 5;
        grandparent.style = Some(Style {
            background: Some(Color::Blue),
            ..Default::default()
        });

        // Create a parent div with red background (overrides blue)
        let mut parent = RenderNode::element();
        parent.x = 1;
        parent.y = 1;
        parent.width = 10;
        parent.height = 3;
        parent.style = Some(Style {
            background: Some(Color::Red),
            ..Default::default()
        });

        // Create a text node without background
        let mut text_node = RenderNode::text("Hello");
        text_node.x = 1;
        text_node.y = 1;
        text_node.width = 5;
        text_node.height = 1;
        // Should inherit red from parent, not blue from grandparent

        // Build the tree
        let grandparent_rc = Rc::new(RefCell::new(grandparent));
        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text_node));

        parent_rc.borrow_mut().children.push(text_rc);
        grandparent_rc.borrow_mut().children.push(parent_rc);

        // Create a buffer and render
        let mut buffer = ScreenBuffer::new(15, 5);
        let clip_rect = crate::bounds::Rect::new(0, 0, 15, 5);
        render_node_to_buffer(&grandparent_rc.borrow(), &mut buffer, &clip_rect, None);

        // Check that text cells have the parent's red background (not grandparent's blue)
        for x in 2..7 {
            let cell = buffer.get_cell(x, 2).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Text at position {x} should have red background from parent, not blue from grandparent"
            );
        }
    }
}
