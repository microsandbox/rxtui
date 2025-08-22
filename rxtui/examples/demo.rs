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

#[derive(Component, Clone)]
struct Demo {
    id: Option<ComponentId>,
}

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
    fn new() -> Self {
        Self { id: None }
    }

    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        if let Some(topic) = topic
            && topic != "navigation"
        {
            return Action::None;
        }

        if let Some(msg) = msg.downcast::<DemoMessage>() {
            let mut state = ctx.get_state::<DemoState>();

            match msg {
                DemoMessage::SetPage(page) => {
                    state.current_page = *page;
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
                    return Action::Exit;
                }
            }

            return Action::Update(Box::new(state));
        }

        Action::None
    }

    fn view(&self, ctx: &Context) -> Node {
        let state = ctx.get_state::<DemoState>();

        let page_content = match state.current_page {
            1 => tui! { node(page1_overflow::Page1OverflowDemo::new()) },
            2 => tui! { node(page2_direction::Page2DirectionDemo::new()) },
            3 => tui! { node(page3_percentages::Page3PercentagesDemo::new()) },
            4 => tui! { node(page4_borders::Page4BordersDemo::new()) },
            5 => tui! { node(page5_absolute::Page5AbsoluteDemo::new()) },
            6 => tui! { node(page6_text_styles::Page6TextStylesDemo::new()) },
            7 => tui! { node(page7_auto_sizing::Page7AutoSizingDemo::new()) },
            8 => tui! { node(page8_text_wrap::Page8TextWrapDemo::new()) },
            9 => tui! { node(page9_element_wrap::Page9ElementWrapDemo::new()) },
            10 => tui! { node(page10_unicode::Page10UnicodeDemo::new()) },
            11 => tui! { node(page11_content_sizing::Page11ContentSizingDemo::new()) },
            12 => tui! { node(page12_focus::Page12FocusDemo::new()) },
            13 => tui! { node(page13_rich_text::Page13::new()) },
            14 => tui! { node(page14_text_input::Page14TextInputDemo::new()) },
            15 => tui! { node(page15_scrollable::Page15ScrollableDemo::new()) },
            _ => tui! { node(page1_overflow::Page1OverflowDemo::new()) },
        };

        // Since tui! macro doesn't support variables as children, I need to create this manually
        let header = tui! {
            div(bg: bright_black, dir: horizontal, pad: 1, w_pct: 1.0, h: 3) [
                text("Radical TUI Demo", color: bright_cyan),
                div(w: 10) [],
                text("Use ← → or 1-9 to navigate, 'q' to quit", color: bright_yellow)
            ]
        };

        let tab_bar = tui! { node(TabBar::new(state.current_page)) };

        // Combine using builder pattern
        let container = Div::new()
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

#[derive(Component, Clone)]
struct TabBar {
    id: Option<ComponentId>,
    current_page: i32,
}

impl TabBar {
    fn new(current_page: i32) -> Self {
        Self {
            id: None,
            current_page,
        }
    }

    fn update(&self, _ctx: &Context, _msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        Action::None
    }

    fn view(&self, _ctx: &Context) -> Node {
        tui! {
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

#[derive(Component, Clone)]
struct Tab {
    id: Option<ComponentId>,
    page_num: i32,
    label: String,
    current_page: i32,
}

impl Tab {
    fn new(page_num: i32, label: &str, current_page: i32) -> Self {
        Self {
            id: None,
            page_num,
            label: label.to_string(),
            current_page,
        }
    }

    fn update(&self, _ctx: &Context, _msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        Action::None
    }

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

        tui! {
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
    let root = Demo::new();
    app.run(root)
}
