use crate::buffer::{Cell, ScreenBuffer};
use crate::render_tree::RenderNodeType;
use crate::utils::{display_width, substring_by_columns};
use crate::{Color, Overflow, Rect, RenderNode};

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Renders a node and its children to the screen buffer with clipping and background inheritance.
///
/// ## Clipping Strategy
///
/// This function uses two different clip rectangles:
///
/// 1. **element_clip**: Used for rendering the element itself (border, background)
///    - Always the intersection of node bounds and incoming clip_rect
///    - Ensures the element doesn't render outside its parent's clip area
///
/// 2. **children_clip**: Used for clipping child elements
///    - When overflow:hidden, clips to padding box (CSS behavior)
///    - When overflow:none, uses parent's clip_rect unchanged
///
/// ```text
/// CSS Box Model & Clipping:
/// ┌─────────────────────────┐ ← node_bounds
/// │ Border                  │
/// │ ┌─────────────────────┐ │ ← padding_box (overflow:hidden clips here)
/// │ │ Padding             │ │
/// │ │ ┌─────────────────┐ │ │ ← content_box
/// │ │ │                 │ │ │
/// │ │ │    Content      │ │ │
/// │ │ │                 │ │ │
/// │ │ └─────────────────┘ │ │
/// │ └─────────────────────┘ │
/// └─────────────────────────┘
///
/// overflow:hidden clips at padding edge (includes padding, excludes border)
/// overflow:none allows children to render outside all bounds
/// ```
pub fn render_node_to_buffer(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_bg: Option<Color>,
) {
    render_node_with_offset(node, buffer, clip_rect, parent_bg, 0);
}

/// Internal function that handles rendering with accumulated scroll offset
fn render_node_with_offset(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_bg: Option<Color>,
    parent_scroll_offset: i16,
) {
    // Calculate the rendered position with parent scroll offset applied
    // Using i32 to allow negative positions for proper clipping
    let rendered_y_i32 = node.y as i32 - parent_scroll_offset as i32;
    let rendered_x = node.x; // No horizontal scrolling

    // For bounds checking, we need to handle negative positions
    // Elements with negative y are partially or fully above the viewport
    let node_bounds = if rendered_y_i32 < 0 {
        // Node starts above viewport - check if it extends into view
        if rendered_y_i32 + node.height as i32 > 0 {
            // Partially visible - create bounds for the visible portion
            let visible_height = (rendered_y_i32 + node.height as i32) as u16;
            Rect::new(rendered_x, 0, node.width, visible_height)
        } else {
            // Completely above viewport
            Rect::empty()
        }
    } else {
        // Normal case - node starts within or below viewport
        Rect::new(rendered_x, rendered_y_i32 as u16, node.width, node.height)
    };

    // Calculate rendered_y for actual rendering (clamped to 0 for partially visible elements)
    let rendered_y = rendered_y_i32.max(0) as u16;

    // Check if node is visible within current clip rect
    if !node_bounds.intersects(clip_rect) {
        return; // Skip rendering if completely outside clip area
    }

    // Calculate clip rect for rendering this element (border, background)
    // This ensures the element itself doesn't render outside the parent's clip area
    let element_clip = node_bounds.intersection(clip_rect);

    // Calculate clip rect for children based on overflow setting
    let children_clip = if let Some(style) = &node.style {
        match style.overflow {
            Some(Overflow::Hidden) | Some(Overflow::Scroll) | Some(Overflow::Auto) => {
                // Clip children to the padding edge (CSS behavior)
                // This means children can render in padding area but not in border area
                let border_offset = if style.border.as_ref().is_some_and(|b| b.enabled) {
                    1
                } else {
                    0
                };

                // Calculate padding box bounds (inside border, includes padding)
                //
                // Example with border=1, padding=2:
                // ┌─────────────┐ (0,0,10x6) ← node bounds
                // │╔═══════════╗│ ← border at (0,0)
                // │║ ┌───────┐ ║│ ← padding box at (1,1,8x4)
                // │║ │content│ ║│ ← content at (3,3,4x0)
                // │║ └───────┘ ║│
                // │╚═══════════╝│
                // └─────────────┘
                let padding_box_x = rendered_x + border_offset;
                // Use actual position for padding box to ensure proper clipping
                let padding_box_y = (rendered_y_i32 + border_offset as i32).max(0) as u16;
                let padding_box_width = node.width.saturating_sub(border_offset * 2);
                // Adjust height if padding box starts above viewport
                let padding_box_height = if rendered_y_i32 + (border_offset as i32) < 0 {
                    // If padding box starts above viewport, reduce height
                    let below_viewport = rendered_y_i32 + node.height as i32;
                    if below_viewport > (border_offset as i32) {
                        (below_viewport - border_offset as i32)
                            .min(node.height as i32 - (border_offset as i32) * 2)
                            as u16
                    } else {
                        0
                    }
                } else {
                    node.height.saturating_sub(border_offset * 2)
                };

                let padding_box_bounds = Rect::new(
                    padding_box_x,
                    padding_box_y,
                    padding_box_width,
                    padding_box_height,
                );
                padding_box_bounds.intersection(clip_rect)
            }
            _ => {
                // If overflow is none (or not set), use parent's clip rect
                *clip_rect
            }
        }
    } else {
        *clip_rect
    };

    match &node.node_type {
        RenderNodeType::Element => {
            // Determine the effective background for the node's text children
            let mut effective_bg = None;

            // Render the element itself (border and background) using element_clip
            // This ensures the element doesn't render outside its parent's bounds
            //
            // Visual example of element_clip vs children_clip:
            //
            // Parent with overflow:hidden, child extends beyond:
            // ┌─────────────────────┐ ← Parent's node_bounds
            // │╔═══════════════════╗│ ← Parent's border (uses element_clip)
            // │║ padding           ║│ ← Parent's padding area
            // │║ ┌─────────────────┼──┐ ← Child extends beyond parent
            // │║ │ Child content   │  │
            // │║ │ is clipped at───┼──┘ ← children_clip (padding edge)
            // │║ └─────────────────┘│
            // │╚═══════════════════╝│
            // └─────────────────────┘
            //
            // - element_clip: Used to render parent's border/background
            // - children_clip: Used to clip child content (at padding edge when overflow:hidden)

            // Draw border if enabled
            if let Some(style) = &node.style {
                if let Some(border) = &style.border
                    && border.enabled
                    && node.width > 1
                    && node.height > 1
                {
                    // Get border characters based on style
                    let (top_left, top, top_right, left, right, bottom_left, bottom, bottom_right) =
                        match border.style {
                            crate::style::BorderStyle::Single => {
                                ('┌', '─', '┐', '│', '│', '└', '─', '┘')
                            }
                            crate::style::BorderStyle::Double => {
                                ('╔', '═', '╗', '║', '║', '╚', '═', '╝')
                            }
                            crate::style::BorderStyle::Thick => {
                                ('┏', '━', '┓', '┃', '┃', '┗', '━', '┛')
                            }
                            crate::style::BorderStyle::Rounded => {
                                ('╭', '─', '╮', '│', '│', '╰', '─', '╯')
                            }
                            crate::style::BorderStyle::Dashed => {
                                ('┌', '╌', '┐', '╎', '╎', '└', '╌', '┘')
                            }
                        };

                    // Draw border within the clipped area
                    let border_bounds = node_bounds.intersection(&element_clip);
                    use crate::style::BorderEdges;

                    // Top border
                    if border.edges.contains(BorderEdges::TOP)
                        && border_bounds.y == rendered_y
                        && border_bounds.height > 0
                    {
                        for x in border_bounds.x..border_bounds.right() {
                            let ch = if x == rendered_x
                                && x >= border_bounds.x
                                && border.edges.contains(BorderEdges::TOP_LEFT)
                            {
                                top_left // Top-left corner
                            } else if x == rendered_x + node.width - 1
                                && x < border_bounds.right()
                                && border.edges.contains(BorderEdges::TOP_RIGHT)
                            {
                                top_right // Top-right corner
                            } else if x != rendered_x && x != rendered_x + node.width - 1 {
                                top // Horizontal line (skip corners if they're not enabled)
                            } else {
                                ' ' // Empty space if corner not enabled
                            };
                            let mut cell = Cell::new(ch);
                            if ch != ' ' {
                                cell.fg = Some(border.color);
                            }
                            buffer.set_cell(x, rendered_y, cell);
                        }
                    }

                    // Bottom border
                    let bottom_y = rendered_y + node.height - 1;
                    if border.edges.contains(BorderEdges::BOTTOM)
                        && bottom_y < border_bounds.bottom()
                        && bottom_y >= border_bounds.y
                    {
                        for x in border_bounds.x..border_bounds.right() {
                            let ch = if x == rendered_x
                                && x >= border_bounds.x
                                && border.edges.contains(BorderEdges::BOTTOM_LEFT)
                            {
                                bottom_left // Bottom-left corner
                            } else if x == rendered_x + node.width - 1
                                && x < border_bounds.right()
                                && border.edges.contains(BorderEdges::BOTTOM_RIGHT)
                            {
                                bottom_right // Bottom-right corner
                            } else if x != rendered_x && x != rendered_x + node.width - 1 {
                                bottom // Horizontal line (skip corners if they're not enabled)
                            } else {
                                ' ' // Empty space if corner not enabled
                            };
                            let mut cell = Cell::new(ch);
                            if ch != ' ' {
                                cell.fg = Some(border.color);
                            }
                            buffer.set_cell(x, bottom_y, cell);
                        }
                    }

                    // Left and right borders
                    for y in (border_bounds.y.max(rendered_y + 1))
                        ..(border_bounds.bottom().min(rendered_y + node.height - 1))
                    {
                        // Left border
                        if border.edges.contains(BorderEdges::LEFT)
                            && rendered_x >= border_bounds.x
                            && rendered_x < border_bounds.right()
                        {
                            let mut cell = Cell::new(left);
                            cell.fg = Some(border.color);
                            buffer.set_cell(rendered_x, y, cell);
                        }

                        // Right border
                        let right_x = rendered_x + node.width - 1;
                        if border.edges.contains(BorderEdges::RIGHT)
                            && right_x >= border_bounds.x
                            && right_x < border_bounds.right()
                        {
                            let mut cell = Cell::new(right);
                            cell.fg = Some(border.color);
                            buffer.set_cell(right_x, y, cell);
                        }
                    }

                    // Draw standalone corners if edges are not present
                    if !border.edges.contains(BorderEdges::TOP)
                        && !border.edges.contains(BorderEdges::LEFT)
                        && border.edges.contains(BorderEdges::TOP_LEFT)
                        && rendered_x >= border_bounds.x
                        && rendered_x < border_bounds.right()
                        && rendered_y >= border_bounds.y
                        && rendered_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(top_left);
                        cell.fg = Some(border.color);
                        buffer.set_cell(rendered_x, rendered_y, cell);
                    }
                    let right_x = rendered_x + node.width - 1;
                    if !border.edges.contains(BorderEdges::TOP)
                        && !border.edges.contains(BorderEdges::RIGHT)
                        && border.edges.contains(BorderEdges::TOP_RIGHT)
                        && right_x >= border_bounds.x
                        && right_x < border_bounds.right()
                        && rendered_y >= border_bounds.y
                        && rendered_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(top_right);
                        cell.fg = Some(border.color);
                        buffer.set_cell(right_x, rendered_y, cell);
                    }
                    let bottom_y = rendered_y + node.height - 1;
                    if !border.edges.contains(BorderEdges::BOTTOM)
                        && !border.edges.contains(BorderEdges::LEFT)
                        && border.edges.contains(BorderEdges::BOTTOM_LEFT)
                        && rendered_x >= border_bounds.x
                        && rendered_x < border_bounds.right()
                        && bottom_y >= border_bounds.y
                        && bottom_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(bottom_left);
                        cell.fg = Some(border.color);
                        buffer.set_cell(rendered_x, bottom_y, cell);
                    }
                    let right_x = rendered_x + node.width - 1;
                    // bottom_y already calculated above
                    if !border.edges.contains(BorderEdges::BOTTOM)
                        && !border.edges.contains(BorderEdges::RIGHT)
                        && border.edges.contains(BorderEdges::BOTTOM_RIGHT)
                        && right_x >= border_bounds.x
                        && right_x < border_bounds.right()
                        && bottom_y >= border_bounds.y
                        && bottom_y < border_bounds.bottom()
                    {
                        let mut cell = Cell::new(bottom_right);
                        cell.fg = Some(border.color);
                        buffer.set_cell(right_x, bottom_y, cell);
                    }
                }

                // Fill the div area with background color if there's any effective background
                if let Some(bg) = style.background {
                    effective_bg = Some(bg);
                    // Fill within the clipped area, but skip border cells if border is enabled
                    let fill_bounds = node_bounds.intersection(&element_clip);
                    let has_border = style.border.as_ref().is_some_and(|b| b.enabled);

                    for y in fill_bounds.y..fill_bounds.bottom() {
                        for x in fill_bounds.x..fill_bounds.right() {
                            // Skip border cells if border is enabled
                            if has_border && node.width > 1 && node.height > 1 {
                                let is_border_cell = (y == rendered_y
                                    || y == rendered_y + node.height - 1)
                                    || (x == rendered_x || x == rendered_x + node.width - 1);
                                if is_border_cell {
                                    // Set background only if cell is empty (preserve border character)
                                    if let Some(cell) = buffer.get_cell_mut(x, y)
                                        && cell.bg.is_none()
                                    {
                                        cell.bg = Some(bg);
                                    }
                                    continue;
                                }
                            }

                            let mut cell = Cell::new(' ');
                            cell.bg = Some(bg);
                            buffer.set_cell(x, y, cell);
                        }
                    }
                }
            }

            // Calculate content area to check if we should render children
            // This prevents rendering when border and padding consume all available space
            //
            // Example: element with width=4, height=4, border=1, padding=1
            // ┌─────┐
            // │╔═══╗│ ← Border takes 1px on each side (2px total)
            // │║   ║│ ← Padding takes 1px on each side (2px total)
            // │╚═══╝│ ← Content area: 4 - 2 - 2 = 0 (no space!)
            // └─────┘
            //
            // In this case, content_width = 0 and content_height = 0,
            // so we skip rendering children entirely.
            let padding = node
                .style
                .as_ref()
                .and_then(|s| s.padding)
                .unwrap_or(crate::style::Spacing::all(0));
            let border_offset = if node
                .style
                .as_ref()
                .and_then(|s| s.border.as_ref())
                .is_some_and(|b| b.enabled)
            {
                1
            } else {
                0
            };

            let content_width = node
                .width
                .saturating_sub(padding.left + padding.right + (border_offset * 2));
            let content_height = node
                .height
                .saturating_sub(padding.top + padding.bottom + (border_offset * 2));

            // Only render children if there's content area available
            if content_width > 0 && content_height > 0 {
                // Sort children by z-index for proper layering
                let mut sorted_children: Vec<_> = node.children.iter().collect();
                sorted_children.sort_by_key(|child| child.borrow().z_index);

                // Render children in z-index order with the children clip rect and background
                // Calculate total scroll offset to pass to children
                let child_scroll_offset = if node.scrollable {
                    parent_scroll_offset + node.scroll_y as i16
                } else {
                    parent_scroll_offset
                };

                for child in sorted_children {
                    render_node_with_offset(
                        &child.borrow(),
                        buffer,
                        &children_clip,
                        effective_bg,
                        child_scroll_offset,
                    );
                }

                // Render scrollbars if needed (for Scroll and Auto modes)
                // Only show scrollbar if explicitly enabled via style
                if node.scrollable
                    && node
                        .style
                        .as_ref()
                        .and_then(|s| s.show_scrollbar)
                        .unwrap_or(true)
                {
                    render_scrollbars(node, buffer, &element_clip, parent_scroll_offset);
                }
            }
        }

        RenderNodeType::Text(text) => {
            // Only render text that's within the clip rect
            let text_width = display_width(text) as u16;
            let text_bounds = crate::bounds::Rect::new(rendered_x, rendered_y, text_width, 1);

            if text_bounds.intersects(clip_rect) {
                // Calculate visible portion of text in display columns
                let visible_start_col = if rendered_x < clip_rect.x {
                    (clip_rect.x - rendered_x) as usize
                } else {
                    0
                };

                let visible_end_col = if rendered_x + text_width > clip_rect.right() {
                    (clip_rect.right() - rendered_x) as usize
                } else {
                    display_width(text)
                };

                if visible_start_col < visible_end_col {
                    // Use substring_by_columns to extract the visible portion safely
                    let visible_text =
                        substring_by_columns(text, visible_start_col, visible_end_col);
                    let render_x = rendered_x.max(clip_rect.x);

                    // Use the full text style if available, otherwise fall back to individual color fields
                    if let Some(text_style) = &node.text_style {
                        // Create a merged text style with background inheritance
                        let mut merged_style = text_style.clone();
                        if merged_style.background.is_none() {
                            merged_style.background = parent_bg;
                        }
                        buffer.write_styled_str(
                            render_x,
                            rendered_y,
                            visible_text,
                            Some(&merged_style),
                        );
                    } else {
                        // Fallback to old method if no full text style
                        let text_bg = node.style.as_ref().and_then(|s| s.background).or(parent_bg);
                        buffer.write_str(
                            render_x,
                            rendered_y,
                            visible_text,
                            node.text_color,
                            text_bg,
                        );
                    }
                }
            }
        }

        RenderNodeType::TextWrapped(lines) => {
            // Render each line of wrapped text
            for (line_idx, line) in lines.iter().enumerate() {
                let line_y = rendered_y + line_idx as u16;

                // Check if this line is within the clip rect
                if line_y >= clip_rect.y && line_y < clip_rect.bottom() {
                    let line_width = display_width(line) as u16;
                    let text_bounds = crate::bounds::Rect::new(rendered_x, line_y, line_width, 1);

                    if text_bounds.intersects(clip_rect) {
                        // Calculate visible portion of this line in display columns
                        let visible_start_col = if rendered_x < clip_rect.x {
                            (clip_rect.x - rendered_x) as usize
                        } else {
                            0
                        };

                        let visible_end_col = if rendered_x + line_width > clip_rect.right() {
                            (clip_rect.right() - rendered_x) as usize
                        } else {
                            display_width(line)
                        };

                        if visible_start_col < visible_end_col {
                            // Use substring_by_columns to extract the visible portion safely
                            let visible_text =
                                substring_by_columns(line, visible_start_col, visible_end_col);
                            let render_x = rendered_x.max(clip_rect.x);

                            // Use the full text style if available
                            if let Some(text_style) = &node.text_style {
                                // Create a merged text style with background inheritance
                                let mut merged_style = text_style.clone();
                                if merged_style.background.is_none() {
                                    merged_style.background = parent_bg;
                                }
                                buffer.write_styled_str(
                                    render_x,
                                    line_y,
                                    visible_text,
                                    Some(&merged_style),
                                );
                            } else {
                                // Fallback to old method if no full text style
                                let text_bg =
                                    node.style.as_ref().and_then(|s| s.background).or(parent_bg);
                                buffer.write_str(
                                    render_x,
                                    line_y,
                                    visible_text,
                                    node.text_color,
                                    text_bg,
                                );
                            }
                        }
                    }
                }
            }
        }

        RenderNodeType::RichText(spans) => {
            // Only render styled text that's within the clip rect
            let text_width = node.width;
            let text_bounds = crate::bounds::Rect::new(rendered_x, rendered_y, text_width, 1);

            if text_bounds.intersects(clip_rect) {
                let mut current_x = rendered_x;

                // Render each span with its own style
                for span in spans {
                    let span_width = display_width(&span.content) as u16;

                    // Check if this span is visible
                    if current_x + span_width > clip_rect.x && current_x < clip_rect.right() {
                        // Calculate visible portion of span
                        let visible_start_col = if current_x < clip_rect.x {
                            (clip_rect.x - current_x) as usize
                        } else {
                            0
                        };

                        let visible_end_col = if current_x + span_width > clip_rect.right() {
                            (clip_rect.right() - current_x) as usize
                        } else {
                            display_width(&span.content)
                        };

                        if visible_start_col < visible_end_col {
                            let visible_text = substring_by_columns(
                                &span.content,
                                visible_start_col,
                                visible_end_col,
                            );
                            let render_x = current_x.max(clip_rect.x);

                            // Apply span's style, falling back to parent background
                            if let Some(span_style) = &span.style {
                                let mut merged_style = span_style.clone();
                                if merged_style.background.is_none() {
                                    merged_style.background = parent_bg;
                                }
                                buffer.write_styled_str(
                                    render_x,
                                    rendered_y,
                                    visible_text,
                                    Some(&merged_style),
                                );
                            } else {
                                // No style on this span - use default with parent background
                                buffer.write_str(
                                    render_x,
                                    rendered_y,
                                    visible_text,
                                    None,
                                    parent_bg,
                                );
                            }
                        }
                    }

                    current_x += span_width;
                }
            }
        }

        RenderNodeType::RichTextWrapped(lines) => {
            // Render each line of wrapped styled text
            for (line_idx, line_spans) in lines.iter().enumerate() {
                let line_y = rendered_y + line_idx as u16;

                // Check if this line is within the clip rect
                if line_y >= clip_rect.y && line_y < clip_rect.bottom() {
                    // Calculate total line width
                    let line_width: u16 = line_spans
                        .iter()
                        .map(|span| display_width(&span.content) as u16)
                        .sum();
                    let text_bounds = crate::bounds::Rect::new(rendered_x, line_y, line_width, 1);

                    if text_bounds.intersects(clip_rect) {
                        let mut current_x = rendered_x;

                        // Render each span in this line with its own style
                        for span in line_spans {
                            let span_width = display_width(&span.content) as u16;

                            // Check if this span is visible
                            if current_x + span_width > clip_rect.x && current_x < clip_rect.right()
                            {
                                // Calculate visible portion of span
                                let visible_start_col = if current_x < clip_rect.x {
                                    (clip_rect.x - current_x) as usize
                                } else {
                                    0
                                };

                                let visible_end_col = if current_x + span_width > clip_rect.right()
                                {
                                    (clip_rect.right() - current_x) as usize
                                } else {
                                    display_width(&span.content)
                                };

                                if visible_start_col < visible_end_col {
                                    let visible_text = substring_by_columns(
                                        &span.content,
                                        visible_start_col,
                                        visible_end_col,
                                    );
                                    let render_x = current_x.max(clip_rect.x);

                                    // Apply span's style, falling back to parent background
                                    if let Some(span_style) = &span.style {
                                        let mut merged_style = span_style.clone();
                                        if merged_style.background.is_none() {
                                            merged_style.background = parent_bg;
                                        }
                                        buffer.write_styled_str(
                                            render_x,
                                            line_y,
                                            visible_text,
                                            Some(&merged_style),
                                        );
                                    } else {
                                        // No style on this span - use default with parent background
                                        buffer.write_str(
                                            render_x,
                                            line_y,
                                            visible_text,
                                            None,
                                            parent_bg,
                                        );
                                    }
                                }
                            }

                            current_x += span_width;
                        }
                    }
                }
            }
        }
    }
}

/// Renders scrollbar indicators for a scrollable node.
///
/// Shows vertical scrollbar when content exceeds viewport.
fn render_scrollbars(
    node: &RenderNode,
    buffer: &mut ScreenBuffer,
    clip_rect: &Rect,
    parent_scroll_offset: i16,
) {
    // Determine if scrollbar is needed
    let needs_scrollbar = node.content_height > node.height;

    // Only show scrollbar for Auto mode if content overflows
    if let Some(style) = &node.style
        && let Some(Overflow::Auto) = style.overflow
        && !needs_scrollbar
    {
        return;
    }

    // Calculate rendered position with parent scroll offset
    let rendered_y = if parent_scroll_offset > 0 {
        node.y.saturating_sub(parent_scroll_offset as u16)
    } else {
        node.y
    };
    let rendered_x = node.x;

    // Vertical scrollbar
    if needs_scrollbar && node.height > 2 {
        let scrollbar_x = rendered_x + node.width.saturating_sub(1);
        let scrollbar_height = node.height;

        // Calculate thumb position and size
        let content_ratio = node.height as f32 / node.content_height as f32;
        let thumb_height = ((scrollbar_height as f32 * content_ratio).ceil() as u16).max(1);
        let scroll_ratio =
            node.scroll_y as f32 / node.content_height.saturating_sub(node.height) as f32;
        let thumb_y = rendered_y
            + ((scrollbar_height.saturating_sub(thumb_height) as f32 * scroll_ratio) as u16);

        // Draw scrollbar track
        for y in rendered_y..rendered_y + scrollbar_height {
            if clip_rect.contains_point(scrollbar_x, y) {
                let ch = if y >= thumb_y && y < thumb_y + thumb_height {
                    '█' // Thumb
                } else {
                    '│' // Track
                };
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::BrightBlack);
                buffer.set_cell(scrollbar_x, y, cell);
            }
        }
    }
}
