use crate::Color;
use crate::node::RichText;
use crate::render_tree::{RenderNode, RenderNodeType};
use crate::style::{Dimension, Direction, Style, TextWrap};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_rich_text_render_node_creation() {
    let rich = RichText::new().text("Hello ").colored("world", Color::Red);

    let render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));

    match &render_node.node_type {
        RenderNodeType::RichText(spans) => {
            assert_eq!(spans.len(), 2);
            assert_eq!(spans[0].content, "Hello ");
            assert_eq!(spans[1].content, "world");
        }
        _ => panic!("Expected RichText node type"),
    }
}

#[test]
fn test_rich_text_width_calculation() {
    let rich = RichText::new()
        .text("Hello ")
        .colored("world", Color::Red)
        .text("!");

    let render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    let (width, height) = render_node.calculate_intrinsic_size();

    assert_eq!(width, 12); // "Hello world!" = 12 chars
    assert_eq!(height, 1); // Single line
}

#[test]
fn test_rich_text_wrapping_application() {
    let rich = RichText::new()
        .text("This is a long text that should wrap")
        .wrap(TextWrap::Word);

    let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    render_node.text_style = rich.style.clone();

    // Apply wrapping with a narrow width
    render_node.apply_text_wrapping(10);

    match &render_node.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            assert!(lines.len() > 1, "Text should wrap to multiple lines");
        }
        _ => panic!("Expected RichTextWrapped after applying wrapping"),
    }
}

#[test]
fn test_rich_text_wrapping_preserves_styles() {
    let rich = RichText::new()
        .text("Normal ")
        .colored("red text", Color::Red)
        .text(" more ")
        .colored("blue text", Color::Blue)
        .wrap(TextWrap::Word);

    let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    render_node.text_style = rich.style.clone();

    // Apply wrapping
    render_node.apply_text_wrapping(15);

    match &render_node.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            // Check that styles are preserved in wrapped lines
            for line in lines {
                for span in line {
                    if span.content.contains("red") {
                        assert_eq!(span.style.as_ref().unwrap().color, Some(Color::Red));
                    } else if span.content.contains("blue") {
                        assert_eq!(span.style.as_ref().unwrap().color, Some(Color::Blue));
                    }
                }
            }
        }
        _ => panic!("Expected RichTextWrapped"),
    }
}

#[test]
fn test_syntax_highlighting_with_word_break() {
    // Create RichText like the syntax highlighting example
    let rich = RichText::new()
        .text("        _ => ")
        .colored("calculate_fibonacci", Color::Yellow)
        .text("(n - ")
        .colored("1", Color::Cyan)
        .text(") + ")
        .colored("calculate_fibonacci", Color::Yellow)
        .text("(n - ")
        .colored("2", Color::Cyan)
        .text(")")
        .wrap(TextWrap::WordBreak);

    let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    render_node.text_style = rich.style.clone();
    render_node.style = Some(Style {
        width: Some(Dimension::Fixed(50)),
        ..Default::default()
    });

    // Apply text wrapping with the fixed width
    render_node.apply_text_wrapping(50);

    match &render_node.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            // The text should wrap into at least one line
            assert!(!lines.is_empty(), "Should have wrapped lines");

            // First line should preserve the leading spaces
            let first_line_text: String =
                lines[0].iter().map(|span| span.content.as_str()).collect();

            // Check that leading spaces are preserved
            assert!(
                first_line_text.starts_with("        "),
                "First line should preserve leading spaces, got: {:?}",
                first_line_text
            );

            // Check that "_ => " is present
            assert!(
                first_line_text.contains("_ => "),
                "First line should contain '_ => ', got: {:?}",
                first_line_text
            );

            // Check that the colored spans have their styles preserved
            for line in lines {
                for span in line {
                    if span.content.contains("calculate_fibonacci") {
                        assert_eq!(
                            span.style.as_ref().and_then(|s| s.color),
                            Some(Color::Yellow),
                            "calculate_fibonacci should be yellow"
                        );
                    } else if span.content == "1" || span.content == "2" {
                        assert_eq!(
                            span.style.as_ref().and_then(|s| s.color),
                            Some(Color::Cyan),
                            "Numbers should be cyan"
                        );
                    }
                }
            }
        }
        _ => panic!("Expected RichTextWrapped after applying wrapping"),
    }
}

#[test]
fn test_leading_spaces_preserved_in_richtext() {
    // Test specifically for leading space preservation
    let rich = RichText::new()
        .text("        ")
        .colored("match", Color::Magenta)
        .text(" n {")
        .wrap(TextWrap::WordBreak);

    let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    render_node.text_style = rich.style.clone();
    render_node.style = Some(Style {
        width: Some(Dimension::Fixed(20)),
        ..Default::default()
    });

    // Apply text wrapping
    render_node.apply_text_wrapping(20);

    match &render_node.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            assert!(!lines.is_empty(), "Should have wrapped lines");

            // Reconstruct the full text
            let full_text: String = lines[0].iter().map(|span| span.content.as_str()).collect();

            assert!(
                full_text.starts_with("        "),
                "Should preserve 8 leading spaces, got: {:?}",
                full_text
            );

            assert!(
                full_text.contains("match"),
                "Should contain 'match', got: {:?}",
                full_text
            );
        }
        _ => panic!("Expected RichTextWrapped"),
    }
}

#[test]
fn test_richtext_unicode_handling() {
    // Test that RichText properly handles Unicode characters
    let rich = RichText::new()
        .text("Hello ")
        .colored("世界", Color::Red) // "world" in Chinese (2 chars, 4 display width)
        .text(" ")
        .colored("😀", Color::Yellow) // emoji (1 char, 2 display width)
        .text(" test")
        .wrap(TextWrap::WordBreak);

    let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    render_node.text_style = rich.style.clone();
    render_node.style = Some(Style {
        width: Some(Dimension::Fixed(15)), // Force wrapping
        ..Default::default()
    });

    // Apply text wrapping
    render_node.apply_text_wrapping(15);

    match &render_node.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            // Verify that Unicode characters are preserved with correct styles
            let mut found_chinese = false;
            let mut found_emoji = false;

            for line in lines {
                for span in line {
                    if span.content.contains("世界") {
                        found_chinese = true;
                        assert_eq!(
                            span.style.as_ref().and_then(|s| s.color),
                            Some(Color::Red),
                            "Chinese text should be red"
                        );
                    }
                    if span.content.contains("😀") {
                        found_emoji = true;
                        assert_eq!(
                            span.style.as_ref().and_then(|s| s.color),
                            Some(Color::Yellow),
                            "Emoji should be yellow"
                        );
                    }
                }
            }

            assert!(found_chinese, "Chinese characters should be preserved");
            assert!(found_emoji, "Emoji should be preserved");
        }
        _ => panic!("Expected RichTextWrapped"),
    }
}

#[test]
fn test_wrapped_richtext_height_in_vertical_layout() {
    // Test that wrapped RichText properly accounts for height in vertical layout
    // This ensures subsequent elements don't overwrite wrapped lines

    // Create a parent div with vertical layout
    let mut parent = RenderNode::element();
    parent.style = Some(Style {
        direction: Some(Direction::Vertical),
        width: Some(Dimension::Fixed(50)),
        height: Some(Dimension::Fixed(30)),
        ..Default::default()
    });

    // Create a RichText that will wrap
    let rich = RichText::new()
        .text("        _ => ")
        .colored("calculate_fibonacci", Color::Yellow)
        .text("(n - ")
        .colored("1", Color::Cyan)
        .text(") + ")
        .colored("calculate_fibonacci", Color::Yellow)
        .text("(n - ")
        .colored("2", Color::Cyan)
        .text(")")
        .wrap(TextWrap::WordBreak);

    let mut rich_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
    rich_node.text_style = rich.style.clone();

    // Create a following text node
    let following_node = RenderNode::text("    }");

    // Set up the hierarchy
    let parent_rc = Rc::new(RefCell::new(parent));
    let rich_rc = Rc::new(RefCell::new(rich_node));
    let following_rc = Rc::new(RefCell::new(following_node));

    parent_rc.borrow_mut().children.push(rich_rc.clone());
    parent_rc.borrow_mut().children.push(following_rc.clone());

    // Layout the parent
    parent_rc.borrow_mut().layout_with_parent(100, 50);

    // Check positions
    let rich_ref = rich_rc.borrow();
    let following_ref = following_rc.borrow();

    // The RichText should be wrapped
    match &rich_ref.node_type {
        RenderNodeType::RichTextWrapped(lines) => {
            let wrapped_height = lines.len() as u16;
            assert!(wrapped_height > 1, "RichText should wrap to multiple lines");

            // The following node should be positioned after all wrapped lines
            assert_eq!(
                following_ref.y,
                rich_ref.y + wrapped_height,
                "Following element should be positioned after all wrapped lines, not overlapping"
            );
        }
        _ => panic!("Expected RichTextWrapped after layout"),
    }
}
