use crate::render_tree::{RenderNode, RenderNodeType};
use crate::style::{Dimension, Direction, Style, TextStyle, TextWrap, WrapMode};
use crate::utils::display_width;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_content_sizing_with_wrapped_text() {
    // Create a text node with wrapping enabled
    let mut text_node = RenderNode::text("This is a long text that should wrap");
    text_node.text_style = Some(TextStyle {
        wrap: Some(TextWrap::Word),
        ..Default::default()
    });
    text_node.style = Some(Style {
        width: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Calculate intrinsic size
    let (width, height) = text_node.calculate_intrinsic_size();

    // Text should wrap to fit within 10 columns
    // With wrapping "This is a long text that should wrap" at width 10:
    // Lines now preserve trailing spaces, so width is 10
    assert_eq!(width, 10, "Width should be longest wrapped line");
    assert!(height > 1, "Height should be more than 1 due to wrapping");
}

#[test]
fn test_horizontal_layout_with_wrapped_text() {
    // Create a parent with horizontal layout
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(30)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Create a text node with long content that will wrap
    let mut text1 = RenderNode::text("This is some long text that should wrap");
    text1.text_style = Some(TextStyle {
        wrap: Some(TextWrap::Word),
        ..Default::default()
    });
    text1.style = Some(Style {
        width: Some(Dimension::Fixed(15)),
        ..Default::default()
    });

    // Create a second text node
    let mut text2 = RenderNode::text("Short");
    text2.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    let parent_rc = Rc::new(RefCell::new(parent));
    let text1_rc = Rc::new(RefCell::new(text1));
    let text2_rc = Rc::new(RefCell::new(text2));

    RenderNode::add_child_with_parent(&parent_rc, text1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, text2_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check text wrapping
    let text1_ref = text1_rc.borrow();
    match &text1_ref.node_type {
        RenderNodeType::TextWrapped(lines) => {
            assert!(
                lines.len() > 1,
                "Text should be wrapped into multiple lines"
            );
        }
        _ => panic!("Text should be wrapped"),
    }

    assert_eq!(text1_ref.width, 15, "Text1 width should be fixed at 15");

    // Text2 should get remaining space or its content width
    let text2_ref = text2_rc.borrow();
    // With Auto width, text2 gets its content width ("Short" = 5 chars)
    // not the remaining space (15)
    assert_eq!(text2_ref.width, 5, "Text2 should get its content width");
}

#[test]
fn test_element_wrap_with_fixed_width() {
    // Create a parent with horizontal layout and wrapping enabled
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(20)),
        height: Some(Dimension::Content),
        wrap: Some(WrapMode::Wrap),
        gap: Some(1),
        ..Default::default()
    });

    // Create several children that won't fit in one row
    let children: Vec<_> = (0..5)
        .map(|_| {
            let mut child = RenderNode::element();
            child.style = Some(Style {
                width: Some(Dimension::Fixed(8)),
                height: Some(Dimension::Fixed(2)),
                ..Default::default()
            });
            Rc::new(RefCell::new(child))
        })
        .collect();

    let parent_rc = Rc::new(RefCell::new(parent));
    for child in &children {
        RenderNode::add_child_with_parent(&parent_rc, child.clone());
    }

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check that children are wrapped to multiple rows
    // Row 1: 8 + 1 + 8 = 17 (fits)
    // Row 2: 8 + 1 + 8 = 17 (fits)
    // Row 3: 8 (last child)

    assert_eq!(children[0].borrow().y, 0, "Child 0 should be on row 1");
    assert_eq!(children[1].borrow().y, 0, "Child 1 should be on row 1");
    assert_eq!(children[2].borrow().y, 3, "Child 2 should be on row 2");
    assert_eq!(children[3].borrow().y, 3, "Child 3 should be on row 2");
    assert_eq!(children[4].borrow().y, 6, "Child 4 should be on row 3");

    // Parent height should expand to fit all rows
    let parent_ref = parent_rc.borrow();
    assert_eq!(parent_ref.height, 8, "Parent should expand to fit 3 rows");
}

#[test]
fn test_element_wrap_with_percentage_children() {
    // Create a parent with wrapping and fixed width
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(30)),
        height: Some(Dimension::Content),
        wrap: Some(WrapMode::Wrap),
        ..Default::default()
    });

    // Create children with percentage widths
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Percentage(0.4)), // 40% = 12
        height: Some(Dimension::Fixed(2)),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Percentage(0.4)), // 40% = 12
        height: Some(Dimension::Fixed(2)),
        ..Default::default()
    });

    let mut child3 = RenderNode::element();
    child3.style = Some(Style {
        width: Some(Dimension::Percentage(0.3)), // 30% = 9
        height: Some(Dimension::Fixed(2)),
        ..Default::default()
    });

    let parent_rc = Rc::new(RefCell::new(parent));
    let child1_rc = Rc::new(RefCell::new(child1));
    let child2_rc = Rc::new(RefCell::new(child2));
    let child3_rc = Rc::new(RefCell::new(child3));

    RenderNode::add_child_with_parent(&parent_rc, child1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child2_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child3_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check wrapping behavior
    // Row 1: 12 + 12 = 24 (fits)
    // Row 2: 9 (third child)

    let child1_ref = child1_rc.borrow();
    let child2_ref = child2_rc.borrow();
    let child3_ref = child3_rc.borrow();

    assert_eq!(child1_ref.y, 0, "Child 1 should be on row 1");
    assert_eq!(child2_ref.y, 0, "Child 2 should be on row 1");
    assert_eq!(child3_ref.y, 2, "Child 3 should be on row 2");
}

#[test]
fn test_nested_wrapping_containers() {
    // Create a parent with wrapping
    let mut outer = RenderNode::element();
    outer.x = 0;
    outer.y = 0;
    outer.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(40)),
        height: Some(Dimension::Content),
        wrap: Some(WrapMode::Wrap),
        ..Default::default()
    });

    // Create an inner container that also wraps
    let mut inner = RenderNode::element();
    inner.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(18)),
        height: Some(Dimension::Content),
        wrap: Some(WrapMode::Wrap),
        ..Default::default()
    });

    // Add children to inner
    let inner_children: Vec<_> = (0..3)
        .map(|_| {
            let mut child = RenderNode::element();
            child.style = Some(Style {
                width: Some(Dimension::Fixed(7)),
                height: Some(Dimension::Fixed(1)),
                ..Default::default()
            });
            Rc::new(RefCell::new(child))
        })
        .collect();

    let outer_rc = Rc::new(RefCell::new(outer));
    let inner_rc = Rc::new(RefCell::new(inner));

    for child in &inner_children {
        RenderNode::add_child_with_parent(&inner_rc, child.clone());
    }

    // Add inner and another element to outer
    let mut sibling = RenderNode::element();
    sibling.style = Some(Style {
        width: Some(Dimension::Fixed(20)),
        height: Some(Dimension::Fixed(3)),
        ..Default::default()
    });
    let sibling_rc = Rc::new(RefCell::new(sibling));

    RenderNode::add_child_with_parent(&outer_rc, inner_rc.clone());
    RenderNode::add_child_with_parent(&outer_rc, sibling_rc.clone());

    // Layout the outer
    outer_rc.borrow_mut().layout_with_parent(100, 50);

    // Inner should wrap its children
    // Sibling should wrap to next row in outer
    let inner_ref = inner_rc.borrow();
    let sibling_ref = sibling_rc.borrow();

    assert_eq!(inner_ref.y, 0, "Inner should be on first row");
    assert_eq!(sibling_ref.y, 0, "Sibling should be on first row");

    // Check that inner children wrapped correctly
    assert_eq!(inner_children[0].borrow().y, 0, "Inner child 0 on row 1");
    assert_eq!(inner_children[1].borrow().y, 0, "Inner child 1 on row 1");
    assert_eq!(inner_children[2].borrow().y, 1, "Inner child 2 on row 2");
}

#[test]
fn test_text_wrapping_with_parent_fixed_width() {
    // Create a parent with fixed width
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.style = Some(Style {
        width: Some(Dimension::Fixed(15)),
        height: Some(Dimension::Content),
        ..Default::default()
    });

    // Create a text node with wrapping enabled but no fixed width
    let mut text = RenderNode::text("This is a long text that needs wrapping");
    text.text_style = Some(TextStyle {
        wrap: Some(TextWrap::Word),
        ..Default::default()
    });

    let parent_rc = Rc::new(RefCell::new(parent));
    let text_rc = Rc::new(RefCell::new(text));

    RenderNode::add_child_with_parent(&parent_rc, text_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Text should wrap to parent's width
    let text_ref = text_rc.borrow();
    match &text_ref.node_type {
        RenderNodeType::TextWrapped(lines) => {
            assert!(lines.len() > 1, "Text should wrap to multiple lines");
            for line in lines {
                assert!(
                    display_width(line) <= 15,
                    "Each line should fit within parent width"
                );
            }
        }
        _ => panic!("Text should be wrapped"),
    }

    assert_eq!(text_ref.width, 15, "Text width should match parent");
}

#[test]
fn test_multiple_wrapped_texts_horizontal() {
    // Create a parent with horizontal layout
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(40)),
        height: Some(Dimension::Content),
        ..Default::default()
    });

    // Create two text nodes with wrapping
    let mut text1 = RenderNode::text("First text that will wrap nicely");
    text1.text_style = Some(TextStyle {
        wrap: Some(TextWrap::Word),
        ..Default::default()
    });
    text1.style = Some(Style {
        width: Some(Dimension::Fixed(20)),
        ..Default::default()
    });

    let mut text2 = RenderNode::text("Second text also wrapping");
    text2.text_style = Some(TextStyle {
        wrap: Some(TextWrap::Word),
        ..Default::default()
    });
    text2.style = Some(Style {
        width: Some(Dimension::Fixed(20)),
        ..Default::default()
    });

    let parent_rc = Rc::new(RefCell::new(parent));
    let text1_rc = Rc::new(RefCell::new(text1));
    let text2_rc = Rc::new(RefCell::new(text2));

    RenderNode::add_child_with_parent(&parent_rc, text1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, text2_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Both texts should wrap
    let text1_ref = text1_rc.borrow();
    let text2_ref = text2_rc.borrow();

    match &text1_ref.node_type {
        RenderNodeType::TextWrapped(lines) => {
            assert!(lines.len() > 1, "Text1 should wrap");
        }
        _ => panic!("Text1 should be wrapped"),
    }

    match &text2_ref.node_type {
        RenderNodeType::TextWrapped(lines) => {
            assert!(lines.len() > 1, "Text2 should wrap");
        }
        _ => panic!("Text2 should be wrapped"),
    }

    // Check positioning
    assert_eq!(text1_ref.x, 0, "Text1 should be at x=0");
    assert_eq!(text2_ref.x, 20, "Text2 should be at x=20");
}
