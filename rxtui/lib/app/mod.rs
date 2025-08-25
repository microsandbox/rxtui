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
    fn test_border_uses_element_bg_when_available() {
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

        // Create a child div with border AND its own red background
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            background: Some(Color::Red), // Has its own background
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::ALL,
            }),
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

        // Check that border cells have the child's red background, not parent's blue
        // Top border
        for x in 1..6 {
            let cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Top border at position {x} should have red background from element, not blue from parent"
            );
        }

        // Left border
        for y in 1..4 {
            let cell = buffer.get_cell(1, y).unwrap();
            assert_eq!(
                cell.bg,
                Some(Color::Red),
                "Left border at position y={y} should have red background from element, not blue from parent"
            );
        }
    }

    #[test]
    fn test_selective_border_edges_background() {
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

        // Create a child with only horizontal borders (no corners)
        let mut child = RenderNode::element();
        child.x = 1;
        child.y = 1;
        child.width = 5;
        child.height = 3;
        child.style = Some(Style {
            background: Some(Color::Red),
            border: Some(Border {
                enabled: true,
                color: Color::White,
                style: BorderStyle::Single,
                edges: BorderEdges::TOP | BorderEdges::BOTTOM, // Only top and bottom, no corners
            }),
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

        // Check that ALL cells in the border row have red background
        // Including the corner positions (x=1 and x=5) even though they're empty
        for x in 1..6 {
            let top_cell = buffer.get_cell(x, 1).unwrap();
            assert_eq!(
                top_cell.bg,
                Some(Color::Red),
                "Top border row at x={x} should have red background, even empty corners"
            );

            let bottom_cell = buffer.get_cell(x, 3).unwrap();
            assert_eq!(
                bottom_cell.bg,
                Some(Color::Red),
                "Bottom border row at x={x} should have red background, even empty corners"
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

    #[test]
    fn test_text_center_alignment() {
        use crate::VNode;
        use crate::prelude::*;
        use crate::vdom::VDom;

        let node: VNode = Div::new()
            .width(10)
            .height(1)
            .child(Text::new("Hi").align(TextAlign::Center).into())
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 10);

        let mut buffer = ScreenBuffer::new(20, 10);
        let clip_rect = crate::Rect::new(0, 0, 20, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            render_node_to_buffer(&root.borrow(), &mut buffer, &clip_rect, None);
        }

        // "Hi" is 2 chars wide, container is 10 wide
        // Should be centered at position 4 (10 - 2) / 2 = 4

        let cell_h = buffer.get_cell(4, 0).unwrap();
        let cell_i = buffer.get_cell(5, 0).unwrap();
        assert_eq!(cell_h.char, 'H', "Expected 'H' at position 4");
        assert_eq!(cell_i.char, 'i', "Expected 'i' at position 5");
    }

    #[test]
    fn test_text_right_alignment() {
        use crate::VNode;
        use crate::prelude::*;
        use crate::vdom::VDom;

        let node: VNode = Div::new()
            .width(10)
            .height(1)
            .child(Text::new("End").align(TextAlign::Right).into())
            .into();

        let mut vdom = VDom::new();
        vdom.render(node);
        vdom.layout(20, 10);

        let mut buffer = ScreenBuffer::new(20, 10);
        let clip_rect = crate::Rect::new(0, 0, 20, 10);

        if let Some(root) = &vdom.get_render_tree().root {
            render_node_to_buffer(&root.borrow(), &mut buffer, &clip_rect, None);
        }

        // "End" is 3 chars wide, container is 10 wide
        // Should be right-aligned at position 7 (10 - 3 = 7)
        let cell_e = buffer.get_cell(7, 0).unwrap();
        let cell_n = buffer.get_cell(8, 0).unwrap();
        let cell_d = buffer.get_cell(9, 0).unwrap();
        assert_eq!(cell_e.char, 'E');
        assert_eq!(cell_n.char, 'n');
        assert_eq!(cell_d.char, 'd');
    }
}
