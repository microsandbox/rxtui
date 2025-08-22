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
}
