mod demo_pages;

use demo_pages::*;
use rxtui::prelude::*;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum DemoMessage {
    SetPage(i32),
    NextPage,
    PrevPage,
    Exit,
}

#[derive(Debug, Clone)]
struct DemoState {
    current_page: i32,
}

#[derive(Component)]
struct Demo;

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for DemoState {
    fn default() -> Self {
        Self { current_page: 1 }
    }
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Demo {
    // Since demo only handles navigation topic messages, we'll simplify this
    #[update]
    fn update(&self, ctx: &Context, msg: DemoMessage, mut state: DemoState) -> Action {
        match msg {
            DemoMessage::SetPage(page) => {
                state.current_page = page;
            }
            DemoMessage::NextPage => {
                state.current_page = (state.current_page % 15) + 1;
            }
            DemoMessage::PrevPage => {
                state.current_page = if state.current_page == 1 {
                    15
                } else {
                    state.current_page - 1
                };
            }
            DemoMessage::Exit => {
                return Action::exit();
            }
        }

        Action::update(state)
    }

    #[view]
    fn view(&self, ctx: &Context, state: DemoState) -> Node {
        let page_content = match state.current_page {
            1 => node! { node(page1_overflow::Page1OverflowDemo::default()) },
            2 => node! { node(page2_direction::Page2DirectionDemo::default()) },
            3 => node! { node(page3_percentages::Page3PercentagesDemo::default()) },
            4 => node! { node(page4_borders::Page4BordersDemo::default()) },
            5 => node! { node(page5_absolute::Page5AbsoluteDemo::default()) },
            6 => node! { node(page6_text_styles::Page6TextStylesDemo::default()) },
            7 => node! { node(page7_auto_sizing::Page7AutoSizingDemo::default()) },
            8 => node! { node(page8_text_wrap::Page8TextWrapDemo::default()) },
            9 => node! { node(page9_element_wrap::Page9ElementWrapDemo::default()) },
            10 => node! { node(page10_unicode::Page10UnicodeDemo::default()) },
            11 => node! { node(page11_content_sizing::Page11ContentSizingDemo::default()) },
            12 => node! { node(page12_focus::Page12FocusDemo::default()) },
            13 => node! { node(page13_rich_text::Page13::default()) },
            14 => node! { node(page14_text_input::Page14TextInputDemo::default()) },
            15 => node! { node(page15_scrollable::Page15ScrollableDemo::default()) },
            _ => node! { node(page1_overflow::Page1OverflowDemo::default()) },
        };

        // Since node! macro doesn't support variables as children, I need to create this manually
        let header = node! {
            div(bg: bright_black, dir: horizontal, pad: 1, w_pct: 1.0, h: 3) [
                text("Radical TUI Demo", color: bright_cyan),
                div(w: 10) [],
                text("Use ← → or 1-9 to navigate, 'q' to quit", color: bright_yellow)
            ]
        };

        let tab_bar = node! { node(TabBar::new(state.current_page)) };

        // Combine using builder pattern
        let container = Div::default()
            .background(Color::Black)
            .direction(Direction::Vertical)
            .padding(Spacing::all(1))
            .width_percent(1.0)
            .height_percent(1.0)
            .children(vec![header, tab_bar, page_content])
            // Global event handlers
            .on_char_global('q', ctx.handler(DemoMessage::Exit))
            .on_key_global(Key::Esc, ctx.handler(DemoMessage::Exit))
            .on_char('1', ctx.handler(DemoMessage::SetPage(1)))
            .on_char('2', ctx.handler(DemoMessage::SetPage(2)))
            .on_char('3', ctx.handler(DemoMessage::SetPage(3)))
            .on_char('4', ctx.handler(DemoMessage::SetPage(4)))
            .on_char('5', ctx.handler(DemoMessage::SetPage(5)))
            .on_char('6', ctx.handler(DemoMessage::SetPage(6)))
            .on_char('7', ctx.handler(DemoMessage::SetPage(7)))
            .on_char('8', ctx.handler(DemoMessage::SetPage(8)))
            .on_char('9', ctx.handler(DemoMessage::SetPage(9)))
            .on_char('0', ctx.handler(DemoMessage::SetPage(10)))
            .on_char('-', ctx.handler(DemoMessage::SetPage(11)))
            .on_char('=', ctx.handler(DemoMessage::SetPage(12)))
            .on_char('[', ctx.handler(DemoMessage::SetPage(13)))
            .on_char(']', ctx.handler(DemoMessage::SetPage(14)))
            .on_char('\\', ctx.handler(DemoMessage::SetPage(15)))
            .on_key(Key::Right, ctx.handler(DemoMessage::NextPage))
            .on_key(Key::Left, ctx.handler(DemoMessage::PrevPage));

        container.into()
    }
}

//--------------------------------------------------------------------------------------------------
// Tab Bar Component
//--------------------------------------------------------------------------------------------------

#[derive(Component, Default)]
struct TabBar {
    current_page: i32,
}

impl TabBar {
    fn new(current_page: i32) -> Self {
        Self { current_page }
    }

    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, _ctx: &Context) -> Node {
        node! {
            div(bg: blue, dir: horizontal, h: 3, w_pct: 1.0) [
                node(Tab::new(1, "[1] Overflow", self.current_page)),
                node(Tab::new(2, "[2] Direction", self.current_page)),
                node(Tab::new(3, "[3] Percentages", self.current_page)),
                node(Tab::new(4, "[4] Borders", self.current_page)),
                node(Tab::new(5, "[5] Absolute", self.current_page)),
                node(Tab::new(6, "[6] Text Styles", self.current_page)),
                node(Tab::new(7, "[7] Auto Sizing", self.current_page)),
                node(Tab::new(8, "[8] Text Wrap", self.current_page)),
                node(Tab::new(9, "[9] Element Wrap", self.current_page)),
                node(Tab::new(10, "[0] Unicode", self.current_page)),
                node(Tab::new(11, "[-] Content Size", self.current_page)),
                node(Tab::new(12, "[=] Focus", self.current_page)),
                node(Tab::new(13, "[[] RichText", self.current_page)),
                node(Tab::new(14, "[]] TextInput", self.current_page)),
                node(Tab::new(15, "[\\] Scrollable", self.current_page))
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Individual Tab Component
//--------------------------------------------------------------------------------------------------

#[derive(Component, Default)]
struct Tab {
    page_num: i32,
    label: String,
    current_page: i32,
}

impl Tab {
    fn new(page_num: i32, label: &str, current_page: i32) -> Self {
        Self {
            page_num,
            label: label.to_string(),
            current_page,
        }
    }

    #[update]
    fn update(&self, _ctx: &Context, _msg: ()) -> Action {
        Action::none()
    }

    #[view]
    fn view(&self, ctx: &Context) -> Node {
        let is_current = self.current_page == self.page_num;
        let bg_color = if is_current { Color::Cyan } else { Color::Blue };
        let text_color = if is_current {
            Color::Black
        } else {
            Color::White
        };
        let label = self.label.clone();
        let page_num = self.page_num;

        node! {
            div(bg: (bg_color), pad: 1, h: 3, w_auto) [
                text(label, color: (text_color)),
                @click: ctx.topic_handler("navigation", DemoMessage::SetPage(page_num))
            ]
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

fn main() -> std::io::Result<()> {
    let mut app = App::new()?;
    app.run(Demo)
}
