use crate::render_tree::RenderNode;
use crate::style::{Border, BorderStyle, Color, Dimension, Direction, Spacing, Style};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_child_respects_parent_content_area() {
    // Create a parent with fixed dimensions, border, and padding
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 20;
    parent.height = 10;
    parent.style = Some(Style {
        background: Some(Color::Blue),
        padding: Some(Spacing::all(2)),
        width: Some(Dimension::Fixed(20)),
        height: Some(Dimension::Fixed(10)),
        border: Some(Border {
            enabled: true,
            style: BorderStyle::Single,
            color: Color::Red,
            edges: crate::style::BorderEdges::ALL,
        }),
        ..Default::default()
    });

    // Create a child with 100% width and height
    let mut child = RenderNode::element();
    child.style = Some(Style {
        background: Some(Color::Green),
        width: Some(Dimension::Percentage(1.0)),
        height: Some(Dimension::Percentage(1.0)),
        ..Default::default()
    });

    // Add child to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let child_rc = Rc::new(RefCell::new(child));
    RenderNode::add_child_with_parent(&parent_rc, child_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check child dimensions
    let child_ref = child_rc.borrow();

    // Expected content area:
    // Width: 20 - 2 (border) - 4 (padding) = 14
    // Height: 10 - 2 (border) - 4 (padding) = 4
    assert_eq!(
        child_ref.width, 14,
        "Child width should be 14 (parent content width)"
    );
    assert_eq!(
        child_ref.height, 4,
        "Child height should be 4 (parent content height)"
    );

    // Check child position (should be offset by border + padding)
    assert_eq!(child_ref.x, 3, "Child x should be 3 (1 border + 2 padding)");
    assert_eq!(child_ref.y, 3, "Child y should be 3 (1 border + 2 padding)");
}

#[test]
fn test_auto_sizing_horizontal() {
    // Create a parent with horizontal layout
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 60;
    parent.height = 10;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(60)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Create three children: fixed, auto, auto
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Fixed(10)),
        height: Some(Dimension::Percentage(1.0)),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Auto),
        height: Some(Dimension::Percentage(1.0)),
        ..Default::default()
    });

    let mut child3 = RenderNode::element();
    child3.style = Some(Style {
        width: Some(Dimension::Auto),
        height: Some(Dimension::Percentage(1.0)),
        ..Default::default()
    });

    // Add children to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let child1_rc = Rc::new(RefCell::new(child1));
    let child2_rc = Rc::new(RefCell::new(child2));
    let child3_rc = Rc::new(RefCell::new(child3));

    RenderNode::add_child_with_parent(&parent_rc, child1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child2_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child3_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check child dimensions
    let child1_ref = child1_rc.borrow();
    let child2_ref = child2_rc.borrow();
    let child3_ref = child3_rc.borrow();

    // Child 1: fixed 10
    assert_eq!(child1_ref.width, 10, "Child 1 should have fixed width 10");

    // Child 2 & 3: auto should split remaining space (60 - 10 = 50, so 25 each)
    assert_eq!(child2_ref.width, 25, "Child 2 should have auto width 25");
    assert_eq!(child3_ref.width, 25, "Child 3 should have auto width 25");

    // All should have full height
    assert_eq!(child1_ref.height, 10, "Child 1 should have full height");
    assert_eq!(child2_ref.height, 10, "Child 2 should have full height");
    assert_eq!(child3_ref.height, 10, "Child 3 should have full height");

    // Check positions
    assert_eq!(child1_ref.x, 0, "Child 1 should be at x=0");
    assert_eq!(child2_ref.x, 10, "Child 2 should be at x=10");
    assert_eq!(child3_ref.x, 35, "Child 3 should be at x=35");
}

#[test]
fn test_auto_sizing_vertical() {
    // Create a parent with vertical layout
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 20;
    parent.height = 50;
    parent.style = Some(Style {
        direction: Some(Direction::Vertical),
        width: Some(Dimension::Fixed(20)),
        height: Some(Dimension::Fixed(50)),
        ..Default::default()
    });

    // Create three children: auto, percentage, fixed
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Percentage(1.0)),
        height: Some(Dimension::Auto),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Percentage(1.0)),
        height: Some(Dimension::Percentage(0.3)), // 30% = 15
        ..Default::default()
    });

    let mut child3 = RenderNode::element();
    child3.style = Some(Style {
        width: Some(Dimension::Percentage(1.0)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Add children to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let child1_rc = Rc::new(RefCell::new(child1));
    let child2_rc = Rc::new(RefCell::new(child2));
    let child3_rc = Rc::new(RefCell::new(child3));

    RenderNode::add_child_with_parent(&parent_rc, child1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child2_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child3_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 100);

    // Check child dimensions
    let child1_ref = child1_rc.borrow();
    let child2_ref = child2_rc.borrow();
    let child3_ref = child3_rc.borrow();

    // Child 1: auto should get remaining space (50 - 15 - 10 = 25)
    assert_eq!(child1_ref.height, 25, "Child 1 should have auto height 25");

    // Child 2: 30% of 50 = 15
    assert_eq!(child2_ref.height, 15, "Child 2 should have 30% height = 15");

    // Child 3: fixed 10
    assert_eq!(child3_ref.height, 10, "Child 3 should have fixed height 10");

    // All should have full width
    assert_eq!(child1_ref.width, 20, "Child 1 should have full width");
    assert_eq!(child2_ref.width, 20, "Child 2 should have full width");
    assert_eq!(child3_ref.width, 20, "Child 3 should have full width");

    // Check positions
    assert_eq!(child1_ref.y, 0, "Child 1 should be at y=0");
    assert_eq!(child2_ref.y, 25, "Child 2 should be at y=25");
    assert_eq!(child3_ref.y, 40, "Child 3 should be at y=40");
}

#[test]
fn test_multiple_auto_sizing() {
    // Create a parent with horizontal layout
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 100;
    parent.height = 10;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(100)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Create five children: fixed, auto, percentage, auto, fixed
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    let mut child3 = RenderNode::element();
    child3.style = Some(Style {
        width: Some(Dimension::Percentage(0.2)), // 20% = 20
        ..Default::default()
    });

    let mut child4 = RenderNode::element();
    child4.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    let mut child5 = RenderNode::element();
    child5.style = Some(Style {
        width: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Add children to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let children = vec![
        Rc::new(RefCell::new(child1)),
        Rc::new(RefCell::new(child2)),
        Rc::new(RefCell::new(child3)),
        Rc::new(RefCell::new(child4)),
        Rc::new(RefCell::new(child5)),
    ];

    for child in &children {
        RenderNode::add_child_with_parent(&parent_rc, child.clone());
    }

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check child dimensions
    // Fixed: 10 + 10 = 20
    // Percentage: 20% of 100 = 20
    // Remaining: 100 - 20 - 20 = 60
    // Auto (2 children): 60 / 2 = 30 each

    assert_eq!(
        children[0].borrow().width,
        10,
        "Child 1 should have fixed width 10"
    );
    assert_eq!(
        children[1].borrow().width,
        30,
        "Child 2 should have auto width 30"
    );
    assert_eq!(
        children[2].borrow().width,
        20,
        "Child 3 should have 20% width = 20"
    );
    assert_eq!(
        children[3].borrow().width,
        30,
        "Child 4 should have auto width 30"
    );
    assert_eq!(
        children[4].borrow().width,
        10,
        "Child 5 should have fixed width 10"
    );
}

#[test]
fn test_auto_sizing_with_padding() {
    // Create a parent with padding
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 50;
    parent.height = 10;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        padding: Some(Spacing::all(5)),
        width: Some(Dimension::Fixed(50)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Create two auto-sized children
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    // Add children to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let child1_rc = Rc::new(RefCell::new(child1));
    let child2_rc = Rc::new(RefCell::new(child2));

    RenderNode::add_child_with_parent(&parent_rc, child1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child2_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check child dimensions
    // Content width: 50 - 10 (padding) = 40
    // Each auto child: 40 / 2 = 20

    assert_eq!(
        child1_rc.borrow().width,
        20,
        "Child 1 should have auto width 20"
    );
    assert_eq!(
        child2_rc.borrow().width,
        20,
        "Child 2 should have auto width 20"
    );

    // Check positions (should account for padding)
    assert_eq!(
        child1_rc.borrow().x,
        5,
        "Child 1 should be at x=5 (padding)"
    );
    assert_eq!(child2_rc.borrow().x, 25, "Child 2 should be at x=25");
}

#[test]
fn test_no_space_for_auto() {
    // Create a parent with horizontal layout where fixed elements take all space
    let mut parent = RenderNode::element();
    parent.x = 0;
    parent.y = 0;
    parent.width = 30;
    parent.height = 10;
    parent.style = Some(Style {
        direction: Some(Direction::Horizontal),
        width: Some(Dimension::Fixed(30)),
        height: Some(Dimension::Fixed(10)),
        ..Default::default()
    });

    // Create three children: fixed 15, auto, fixed 15
    let mut child1 = RenderNode::element();
    child1.style = Some(Style {
        width: Some(Dimension::Fixed(15)),
        ..Default::default()
    });

    let mut child2 = RenderNode::element();
    child2.style = Some(Style {
        width: Some(Dimension::Auto),
        ..Default::default()
    });

    let mut child3 = RenderNode::element();
    child3.style = Some(Style {
        width: Some(Dimension::Fixed(15)),
        ..Default::default()
    });

    // Add children to parent
    let parent_rc = Rc::new(RefCell::new(parent));
    let child1_rc = Rc::new(RefCell::new(child1));
    let child2_rc = Rc::new(RefCell::new(child2));
    let child3_rc = Rc::new(RefCell::new(child3));

    RenderNode::add_child_with_parent(&parent_rc, child1_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child2_rc.clone());
    RenderNode::add_child_with_parent(&parent_rc, child3_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check that auto child gets 0 width
    assert_eq!(
        child2_rc.borrow().width,
        0,
        "Auto child should get 0 width when no space available"
    );
}
