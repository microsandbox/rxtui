use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Component, Clone, Default)]
pub struct Page4BordersDemo {}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Page4BordersDemo {
    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: black, dir: vertical, pad: 1, w_pct: 1.0, h: 60) [
                // Title
                text("Page 4: Borders Demo", color: bright_white),
                spacer(1),

                // Border style examples
                div(dir: vertical, w_pct: 0.9, h: 45) [
                    // Row 1: Single and Double
                    hstack(w_pct: 1.0, h: 8) [
                        div(w_pct: 0.48, h: 8, border: cyan, pad: 1) [
                            text("Single Border (Default)", color: cyan)
                        ],
                        div(w_pct: 0.04) [], // Spacer
                        div(w_pct: 0.48, h: 8, border_style: (BorderStyle::Double, Color::Green), pad: 1) [
                            text("Double Border", color: green)
                        ]
                    ],
                    spacer(1),

                    // Row 2: Thick and Rounded
                    hstack(w_pct: 1.0, h: 8) [
                        div(w_pct: 0.48, h: 8, border_style: (BorderStyle::Thick, Color::Red), pad: 1) [
                            text("Thick Border", color: red)
                        ],
                        div(w_pct: 0.04) [], // Spacer
                        div(w_pct: 0.48, h: 8, border_style: (BorderStyle::Rounded, Color::Magenta), pad: 1) [
                            text("Rounded Border", color: magenta)
                        ]
                    ],
                    spacer(1),

                    // Row 3: Dashed and Mixed Example
                    hstack(w_pct: 1.0, h: 8) [
                        div(w_pct: 0.48, h: 8, border_style: (BorderStyle::Dashed, Color::Yellow), pad: 1) [
                            text("Dashed Border", color: yellow)
                        ],
                        div(w_pct: 0.04) [], // Spacer
                        div(w_pct: 0.48, h: 8, border_style: (BorderStyle::Double, Color::BrightBlue), bg: bright_black, pad: 1) [
                            text("With Background", color: bright_white)
                        ]
                    ],
                    spacer(1),

                    // Selective border edges
                    text("Selective Border Edges:", color: white),
                    spacer(1),
                    hstack(w_pct: 1.0, h: 6) [
                        div(w_pct: 0.23, h: 6, border_style: (BorderStyle::Single, Color::Cyan),
                             border_edges: (BorderEdges::HORIZONTAL), pad: 1) [
                            text("Horizontal", color: cyan)
                        ],
                        div(w_pct: 0.02) [], // Spacer
                        div(w_pct: 0.23, h: 6, border_style: (BorderStyle::Single, Color::Green),
                             border_edges: (BorderEdges::VERTICAL), pad: 1) [
                            text("Vertical", color: green)
                        ],
                        div(w_pct: 0.02) [], // Spacer
                        div(w_pct: 0.23, h: 6, border_style: (BorderStyle::Rounded, Color::Magenta),
                             border_edges: (BorderEdges::CORNERS), pad: 1) [
                            text("Corners", color: magenta)
                        ],
                        div(w_pct: 0.02) [], // Spacer
                        div(w_pct: 0.23, h: 6, border_style: (BorderStyle::Single, Color::Yellow),
                             border_edges: (BorderEdges::TOP | BorderEdges::RIGHT | BorderEdges::TOP_RIGHT), pad: 1) [
                            text("Custom", color: yellow)
                        ]
                    ],
                    spacer(1),

                    // Complex nested example with mixed border styles
                    text("Complex Nested Example with Mixed Styles:", color: white),
                    spacer(1),
                    div(w_pct: 0.95, h: 12, border_style: (BorderStyle::Double, Color::BrightBlue),
                        pad: 1, dir: horizontal) [
                        div(w_pct: 0.3, h_pct: 1.0, border_style: (BorderStyle::Rounded, Color::BrightGreen),
                            bg: bright_black, border_edges: (BorderEdges::TOP | BorderEdges::BOTTOM), pad: 1) [
                            text("Top/Bottom", color: bright_green)
                        ],
                        div(w: 2) [],
                        div(w_pct: 0.3, h_pct: 1.0, border_style: (BorderStyle::Thick, Color::BrightRed), pad: 1) [
                            text("Full Border", color: bright_red)
                        ],
                        div(w: 2) [],
                        div(w_pct: 0.3, h_pct: 1.0, border_style: (BorderStyle::Dashed, Color::BrightYellow),
                            bg: bright_black, border_edges: (BorderEdges::EDGES), pad: 1) [
                            text("No Corners", color: bright_yellow)
                        ]
                    ],
                    spacer(1),

                    // Example of no content space
                    text("Border & Padding with No Content Space:", color: white),
                    spacer(1),
                    hstack(w_pct: 1.0, h: 8) [
                        // Example with height=4, border=1x2, padding=1x2, leaving 0 height for content
                        div(w_pct: 0.3, h: 4, border: red, bg: bright_black, pad: 1) [
                            text("No space for text!", color: white)
                        ],
                        div(w_pct: 0.05) [], // Spacer
                        // Example with width too small for border+padding
                        div(w: 6, h: 6, border_style: (BorderStyle::Double, Color::Yellow),
                            bg: bright_black, overflow: hidden, pad: 1) [
                            text("Squished!", color: yellow)
                        ],
                        div(w_pct: 0.05) [], // Spacer
                        // Extreme case: exactly border+padding size
                        div(w: 4, h: 4, border_style: (BorderStyle::Thick, Color::Cyan),
                            overflow: hidden, bg: bright_black, pad: 1) [
                            text("Gone!", color: cyan)
                        ]
                    ]
                ]
            ]
        }
    }
}
