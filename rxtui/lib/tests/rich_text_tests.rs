use crate::node::RichText;
use crate::render_tree::{RenderNode, RenderNodeType};
use crate::style::{Dimension, Style, TextStyle, TextWrap};
use crate::utils::display_width;
use crate::{Color, Node};
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod rich_text_builder_tests {
    use super::*;

    #[test]
    fn test_rich_text_creation() {
        let rich = RichText::new()
            .text("Hello ")
            .colored("world", Color::Red)
            .text("!");

        assert_eq!(rich.spans.len(), 3);
        assert_eq!(rich.spans[0].content, "Hello ");
        assert_eq!(rich.spans[1].content, "world");
        assert_eq!(rich.spans[2].content, "!");
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().color,
            Some(Color::Red)
        );
    }

    #[test]
    fn test_rich_text_bold_italic() {
        let rich = RichText::new()
            .text("Normal ")
            .bold("Bold")
            .text(" ")
            .italic("Italic");

        assert_eq!(rich.spans.len(), 4);
        assert_eq!(rich.spans[1].style.as_ref().unwrap().bold, Some(true));
        assert_eq!(rich.spans[3].style.as_ref().unwrap().italic, Some(true));
    }

    #[test]
    fn test_rich_text_with_cursor() {
        // Cursor in middle
        let rich = RichText::with_cursor(
            "Hello",
            2,
            TextStyle {
                background: Some(Color::Blue),
                ..Default::default()
            },
        );

        assert_eq!(rich.spans.len(), 3);
        assert_eq!(rich.spans[0].content, "He");
        assert_eq!(rich.spans[1].content, "l");
        assert_eq!(rich.spans[2].content, "lo");
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().background,
            Some(Color::Blue)
        );

        // Cursor at end
        let rich_end = RichText::with_cursor(
            "Hi",
            2,
            TextStyle {
                background: Some(Color::Green),
                ..Default::default()
            },
        );

        assert_eq!(rich_end.spans.len(), 2);
        assert_eq!(rich_end.spans[0].content, "Hi");
        assert_eq!(rich_end.spans[1].content, " ");
        assert_eq!(
            rich_end.spans[1].style.as_ref().unwrap().background,
            Some(Color::Green)
        );
    }

    #[test]
    fn test_top_level_styling_methods() {
        let rich = RichText::new()
            .text("First")
            .text(" ")
            .text("Second")
            .color(Color::Yellow)
            .background(Color::Black);

        // All spans should have yellow text on black background
        for span in &rich.spans {
            assert_eq!(span.style.as_ref().unwrap().color, Some(Color::Yellow));
            assert_eq!(span.style.as_ref().unwrap().background, Some(Color::Black));
        }
    }

    #[test]
    fn test_rich_text_bold_all() {
        let rich = RichText::new()
            .text("One")
            .colored("Two", Color::Red)
            .text("Three")
            .bold_all();

        // All spans should be bold
        for span in &rich.spans {
            assert_eq!(span.style.as_ref().unwrap().bold, Some(true));
        }
        // Second span should retain its color
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().color,
            Some(Color::Red)
        );
    }

    #[test]
    fn test_rich_text_wrap() {
        let rich = RichText::new()
            .text("This is wrapped text")
            .wrap(TextWrap::Word);

        assert!(rich.style.is_some());
        assert_eq!(rich.style.as_ref().unwrap().wrap, Some(TextWrap::Word));
    }

    #[test]
    fn test_rich_text_helper_methods() {
        let mut rich = RichText::new().text("Hello").text(" ").text("World");

        // Test content()
        assert_eq!(rich.content(), "Hello World");

        // Test is_empty()
        assert!(!rich.is_empty());

        // Test clear()
        rich.clear();
        assert!(rich.is_empty());
        assert_eq!(rich.content(), "");

        // Test append()
        let mut rich1 = RichText::new().text("First");
        let mut rich2 = RichText::new().colored("Second", Color::Blue);
        rich1.append(&mut rich2);
        assert_eq!(rich1.spans.len(), 2);
        assert_eq!(rich1.content(), "FirstSecond");
        assert!(rich2.is_empty());
    }

    #[test]
    fn test_rich_text_from_traits() {
        // From String
        let from_string: RichText = String::from("test string").into();
        assert_eq!(from_string.spans.len(), 1);
        assert_eq!(from_string.content(), "test string");

        // From &str
        let from_str: RichText = "test str".into();
        assert_eq!(from_str.spans.len(), 1);
        assert_eq!(from_str.content(), "test str");
    }

    #[test]
    fn test_rich_text_default() {
        let rich = RichText::default();
        assert!(rich.is_empty());
        assert_eq!(rich.content(), "");
    }
}

#[cfg(test)]
mod rich_text_rendering_tests {
    use super::*;

    #[test]
    fn test_rich_text_render_node_creation() {
        let rich = RichText::new()
            .text("Hello ")
            .colored("colorful", Color::Green)
            .text(" world");

        let render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));

        match &render_node.node_type {
            RenderNodeType::RichText(spans) => {
                assert_eq!(spans.len(), 3);
                assert_eq!(spans[0].content, "Hello ");
                assert_eq!(spans[1].content, "colorful");
                assert_eq!(spans[2].content, " world");
            }
            _ => panic!("Expected RichText node type"),
        }
    }

    #[test]
    fn test_rich_text_width_calculation() {
        let rich = RichText::new().text("ABC").text("DEF").text("GHI");

        let total_width: u16 = rich
            .spans
            .iter()
            .map(|span| display_width(&span.content) as u16)
            .sum();

        assert_eq!(total_width, 9);
        assert_eq!(rich.content().len(), 9);
    }

    #[test]
    fn test_rich_text_wrapping_application() {
        let rich = RichText::new()
            .text("This is a long text ")
            .colored("with colored part ", Color::Red)
            .text("that should wrap nicely")
            .wrap(TextWrap::Word);

        let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
        render_node.text_style = rich.style.clone();
        render_node.style = Some(Style {
            width: Some(Dimension::Fixed(15)),
            ..Default::default()
        });

        // Apply text wrapping with the fixed width
        render_node.apply_text_wrapping(15);

        match &render_node.node_type {
            RenderNodeType::RichTextWrapped(lines) => {
                assert!(
                    lines.len() > 1,
                    "Text should be wrapped into multiple lines"
                );

                // Each line should preserve the original span styles
                for line in lines {
                    for span in line {
                        // Check if colored spans retained their color
                        if span.content.contains("colored") {
                            assert_eq!(span.style.as_ref().and_then(|s| s.color), Some(Color::Red));
                        }
                    }
                }
            }
            _ => panic!("Expected RichTextWrapped after applying wrapping"),
        }
    }

    #[test]
    fn test_rich_text_wrapping_preserves_styles() {
        let rich = RichText::new()
            .bold("Bold ")
            .italic("Italic ")
            .colored("Color", Color::Blue);

        let mut render_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
        render_node.text_style = Some(TextStyle {
            wrap: Some(TextWrap::Word),
            ..Default::default()
        });
        render_node.style = Some(Style {
            width: Some(Dimension::Fixed(10)),
            ..Default::default()
        });

        // Apply text wrapping with the fixed width
        render_node.apply_text_wrapping(10);

        match &render_node.node_type {
            RenderNodeType::RichTextWrapped(lines) => {
                // Collect all spans from wrapped lines
                let all_spans: Vec<_> = lines.iter().flat_map(|line| line.iter()).collect();

                // Find spans by content and check their styles
                for span in all_spans {
                    if span.content.contains("Bold") {
                        assert_eq!(span.style.as_ref().unwrap().bold, Some(true));
                    }
                    if span.content.contains("Italic") {
                        assert_eq!(span.style.as_ref().unwrap().italic, Some(true));
                    }
                    if span.content.contains("Color") {
                        assert_eq!(span.style.as_ref().unwrap().color, Some(Color::Blue));
                    }
                }
            }
            _ => panic!("Expected RichTextWrapped"),
        }
    }
}

#[cfg(test)]
mod rich_text_integration_tests {
    use super::*;
    use crate::node::Div;
    use crate::vnode::VNode;

    #[test]
    fn test_rich_text_in_div() {
        let rich = RichText::new().text("Hello ").colored("world", Color::Red);

        let container = Div::new().children(vec![VNode::RichText(rich.clone())]);

        assert_eq!(container.children.len(), 1);
        match &container.children[0] {
            VNode::RichText(s) => {
                assert_eq!(s.content(), "Hello world");
            }
            _ => panic!("Expected RichText VNode"),
        }
    }

    #[test]
    fn test_rich_text_with_parent_width() {
        let mut parent = RenderNode::element();
        parent.style = Some(Style {
            width: Some(Dimension::Fixed(20)),
            height: Some(Dimension::Content),
            ..Default::default()
        });

        let rich = RichText::new()
            .text("This is a very long rich text that needs wrapping")
            .wrap(TextWrap::Word);

        let mut rich_node = RenderNode::new(RenderNodeType::RichText(rich.spans.clone()));
        rich_node.text_style = rich.style.clone();

        let parent_rc = Rc::new(RefCell::new(parent));
        let rich_rc = Rc::new(RefCell::new(rich_node));

        RenderNode::add_child_with_parent(&parent_rc, rich_rc.clone());

        // Layout the parent
        parent_rc.borrow_mut().layout_with_parent(100, 50);

        // Check that rich text wrapped correctly
        let rich_ref = rich_rc.borrow();
        match &rich_ref.node_type {
            RenderNodeType::RichTextWrapped(lines) => {
                assert!(lines.len() > 1, "RichText should wrap to multiple lines");
                for line in lines {
                    let line_width: u16 = line
                        .iter()
                        .map(|span| display_width(&span.content) as u16)
                        .sum();
                    assert!(line_width <= 20, "Each line should fit within parent width");
                }
            }
            _ => panic!("RichText should be wrapped"),
        }
    }

    #[test]
    fn test_rich_text_mixed_with_regular_text() {
        let mut parent = RenderNode::element();
        parent.style = Some(Style {
            width: Some(Dimension::Fixed(40)),
            height: Some(Dimension::Content),
            ..Default::default()
        });

        // Add regular text
        let text = RenderNode::text("Regular text");

        // Add rich text
        let rich = RichText::new()
            .text("Styled ")
            .colored("colorful", Color::Green)
            .text(" text");
        let rich_node = RenderNode::new(RenderNodeType::RichText(rich.spans));

        let parent_rc = Rc::new(RefCell::new(parent));
        let text_rc = Rc::new(RefCell::new(text));
        let rich_rc = Rc::new(RefCell::new(rich_node));

        RenderNode::add_child_with_parent(&parent_rc, text_rc.clone());
        RenderNode::add_child_with_parent(&parent_rc, rich_rc.clone());

        // Layout the parent
        parent_rc.borrow_mut().layout_with_parent(100, 50);

        // Both should layout correctly
        assert_eq!(text_rc.borrow().y, 0);
        assert_eq!(rich_rc.borrow().y, 1);
    }

    #[test]
    fn test_rich_text_from_node() {
        let rich = RichText::new().text("Test ").colored("node", Color::Blue);

        let node: Node = rich.into();

        match node {
            Node::RichText(s) => {
                assert_eq!(s.content(), "Test node");
                assert_eq!(s.spans[1].style.as_ref().unwrap().color, Some(Color::Blue));
            }
            _ => panic!("Expected RichText node"),
        }
    }
}
