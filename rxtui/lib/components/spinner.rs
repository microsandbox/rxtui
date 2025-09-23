use crate::Context;
use crate::component::{Action, Component, Message, MessageExt};
use crate::effect::Effect;
use crate::node::{Node, Text};
use crate::style::{Color, TextStyle};
use std::time::Duration;

//--------------------------------------------------------------------------------------------------
// Types: Internal
//--------------------------------------------------------------------------------------------------

/// Messages for Spinner component
#[derive(Debug, Clone)]
pub enum SpinnerMsg {
    /// Advance to the next frame
    Tick,
}

/// State for Spinner component
#[derive(Debug, Clone, Default)]
struct SpinnerState {
    /// Current frame index
    frame_index: usize,
}

/// Spinner pattern data
struct SpinnerPattern {
    frames: &'static [&'static str],
}

//--------------------------------------------------------------------------------------------------
// Types: Public API
//--------------------------------------------------------------------------------------------------

/// Animation speed for the spinner
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpinnerSpeed {
    /// Slow animation (150ms per frame)
    Slow,
    /// Normal animation (80ms per frame)
    Normal,
    /// Fast animation (50ms per frame)
    Fast,
    /// Custom interval in milliseconds
    Custom(u64),
}

/// Available spinner types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpinnerType {
    Dots,
    Dots2,
    Dots3,
    Line,
    Line2,
    Pipe,
    SimpleDots,
    SimpleDotsScrolling,
    Star,
    Star2,
    Flip,
    Hamburger,
    GrowVertical,
    GrowHorizontal,
    Balloon,
    Balloon2,
    Noise,
    Bounce,
    BoxBounce,
    BoxBounce2,
    Triangle,
    Binary,
    Arc,
    Circle,
    SquareCorners,
    CircleQuarters,
    CircleHalves,
    Squish,
    Toggle,
    Toggle2,
    Toggle3,
    Arrow,
    Arrow2,
    Arrow3,
    BouncingBar,
    BouncingBall,
    Clock,
    Earth,
    Moon,
    Hearts,
    Smiley,
    Monkey,
    Weather,
    Christmas,
    Point,
    Layer,
    BetaWave,
    Aesthetic,
    /// Custom spinner with user-defined frames
    Custom(Vec<String>),
}

/// A spinner component for loading animations
///
/// The Spinner component provides animated loading indicators with many built-in styles.
/// It's a self-contained component that manages its own animation timing.
///
/// # Example
///
/// ```ignore
/// use rxtui::prelude::*;
/// use rxtui::components::{Spinner, SpinnerType, SpinnerSpeed};
///
/// // Basic usage with defaults
/// let spinner = Spinner::new();
///
/// // Customized spinner
/// let spinner = Spinner::new()
///     .spinner_type(SpinnerType::Hearts)
///     .speed(SpinnerSpeed::Fast);
/// ```
#[derive(Clone)]
pub struct Spinner {
    spinner_type: SpinnerType,
    speed: SpinnerSpeed,
    color: Option<Color>,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: SpinnerSpeed
//--------------------------------------------------------------------------------------------------

impl Default for SpinnerSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

impl SpinnerSpeed {
    fn interval(&self) -> u64 {
        match self {
            Self::Slow => 150,
            Self::Normal => 80,
            Self::Fast => 50,
            Self::Custom(ms) => *ms,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: SpinnerType
//--------------------------------------------------------------------------------------------------

impl Default for SpinnerType {
    fn default() -> Self {
        Self::Dots
    }
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// Dots spinner pattern
const DOTS: SpinnerPattern = SpinnerPattern {
    frames: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
};

/// Dots2 spinner pattern
const DOTS2: SpinnerPattern = SpinnerPattern {
    frames: &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
};

/// Dots3 spinner pattern
const DOTS3: SpinnerPattern = SpinnerPattern {
    frames: &["⠋", "⠙", "⠚", "⠞", "⠖", "⠦", "⠴", "⠲", "⠳", "⠓"],
};

/// Line spinner pattern
const LINE: SpinnerPattern = SpinnerPattern {
    frames: &["-", "\\", "|", "/"],
};

/// Line2 spinner pattern
const LINE2: SpinnerPattern = SpinnerPattern {
    frames: &["⠂", "-", "–", "—", "–", "-"],
};

/// Pipe spinner pattern
const PIPE: SpinnerPattern = SpinnerPattern {
    frames: &["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
};

/// Simple dots spinner pattern
const SIMPLE_DOTS: SpinnerPattern = SpinnerPattern {
    frames: &[".  ", ".. ", "...", "   "],
};

/// Simple dots scrolling spinner pattern
const SIMPLE_DOTS_SCROLLING: SpinnerPattern = SpinnerPattern {
    frames: &[".  ", ".. ", "...", " ..", "  .", "   "],
};

/// Star spinner pattern
const STAR: SpinnerPattern = SpinnerPattern {
    frames: &["✶", "✸", "✹", "✺", "✹", "✷"],
};

/// Star2 spinner pattern
const STAR2: SpinnerPattern = SpinnerPattern {
    frames: &["+", "x", "*"],
};

/// Flip spinner pattern
const FLIP: SpinnerPattern = SpinnerPattern {
    frames: &["_", "_", "_", "-", "`", "`", "'", "´", "-", "_", "_", "_"],
};

/// Hamburger spinner pattern
const HAMBURGER: SpinnerPattern = SpinnerPattern {
    frames: &["☱", "☲", "☴"],
};

/// Grow vertical spinner pattern
const GROW_VERTICAL: SpinnerPattern = SpinnerPattern {
    frames: &["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃"],
};

/// Grow horizontal spinner pattern
const GROW_HORIZONTAL: SpinnerPattern = SpinnerPattern {
    frames: &["▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"],
};

/// Balloon spinner pattern
const BALLOON: SpinnerPattern = SpinnerPattern {
    frames: &[" ", ".", "o", "O", "@", "*", " "],
};

/// Balloon2 spinner pattern
const BALLOON2: SpinnerPattern = SpinnerPattern {
    frames: &[".", "o", "O", "°", "O", "o", "."],
};

/// Noise spinner pattern
const NOISE: SpinnerPattern = SpinnerPattern {
    frames: &["▓", "▒", "░"],
};

/// Bounce spinner pattern
const BOUNCE: SpinnerPattern = SpinnerPattern {
    frames: &["⠁", "⠂", "⠄", "⠂"],
};

/// Box bounce spinner pattern
const BOX_BOUNCE: SpinnerPattern = SpinnerPattern {
    frames: &["▖", "▘", "▝", "▗"],
};

/// Box bounce2 spinner pattern
const BOX_BOUNCE2: SpinnerPattern = SpinnerPattern {
    frames: &["▌", "▀", "▐", "▄"],
};

/// Triangle spinner pattern
const TRIANGLE: SpinnerPattern = SpinnerPattern {
    frames: &["◢", "◣", "◤", "◥"],
};

/// Binary spinner pattern
const BINARY: SpinnerPattern = SpinnerPattern {
    frames: &[
        "010010", "001100", "100101", "111010", "111101", "010111", "101011", "111000", "110011",
        "110101",
    ],
};

/// Arc spinner pattern
const ARC: SpinnerPattern = SpinnerPattern {
    frames: &["◜", "◠", "◝", "◞", "◡", "◟"],
};

/// Circle spinner pattern
const CIRCLE: SpinnerPattern = SpinnerPattern {
    frames: &["◡", "⊙", "◠"],
};

/// Square corners spinner pattern
const SQUARE_CORNERS: SpinnerPattern = SpinnerPattern {
    frames: &["◰", "◳", "◲", "◱"],
};

/// Circle quarters spinner pattern
const CIRCLE_QUARTERS: SpinnerPattern = SpinnerPattern {
    frames: &["◴", "◷", "◶", "◵"],
};

/// Circle halves spinner pattern
const CIRCLE_HALVES: SpinnerPattern = SpinnerPattern {
    frames: &["◐", "◓", "◑", "◒"],
};

/// Squish spinner pattern
const SQUISH: SpinnerPattern = SpinnerPattern {
    frames: &["╫", "╪"],
};

/// Toggle spinner pattern
const TOGGLE: SpinnerPattern = SpinnerPattern {
    frames: &["⊶", "⊷"],
};

/// Toggle2 spinner pattern
const TOGGLE2: SpinnerPattern = SpinnerPattern {
    frames: &["▫", "▪"],
};

/// Toggle3 spinner pattern
const TOGGLE3: SpinnerPattern = SpinnerPattern {
    frames: &["□", "■"],
};

/// Arrow spinner pattern
const ARROW: SpinnerPattern = SpinnerPattern {
    frames: &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
};

/// Arrow2 spinner pattern (with emoji)
const ARROW2: SpinnerPattern = SpinnerPattern {
    frames: &["⬆️ ", "↗️ ", "➡️ ", "↘️ ", "⬇️ ", "↙️ ", "⬅️ ", "↖️ "],
};

/// Arrow3 spinner pattern
const ARROW3: SpinnerPattern = SpinnerPattern {
    frames: &["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸"],
};

/// Bouncing bar spinner pattern
const BOUNCING_BAR: SpinnerPattern = SpinnerPattern {
    frames: &[
        "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]", "[    ]",
        "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
    ],
};

/// Bouncing ball spinner pattern
const BOUNCING_BALL: SpinnerPattern = SpinnerPattern {
    frames: &[
        "( ●    )",
        "(  ●   )",
        "(   ●  )",
        "(    ● )",
        "(     ●)",
        "(    ● )",
        "(   ●  )",
        "(  ●   )",
        "( ●    )",
        "(●     )",
    ],
};

/// Clock spinner pattern
const CLOCK: SpinnerPattern = SpinnerPattern {
    frames: &[
        "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ", "🕚 ",
    ],
};

/// Earth spinner pattern
const EARTH: SpinnerPattern = SpinnerPattern {
    frames: &["🌍 ", "🌎 ", "🌏 "],
};

/// Moon spinner pattern
const MOON: SpinnerPattern = SpinnerPattern {
    frames: &["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "],
};

/// Hearts spinner pattern
const HEARTS: SpinnerPattern = SpinnerPattern {
    frames: &["💛 ", "💙 ", "💜 ", "💚 ", "💗 "],
};

/// Smiley spinner pattern
const SMILEY: SpinnerPattern = SpinnerPattern {
    frames: &["😄 ", "😝 "],
};

/// Monkey spinner pattern
const MONKEY: SpinnerPattern = SpinnerPattern {
    frames: &["🙈 ", "🙈 ", "🙉 ", "🙊 "],
};

/// Weather spinner pattern
const WEATHER: SpinnerPattern = SpinnerPattern {
    frames: &[
        "☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️ ", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "⛈ ",
        "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️ ", "🌤 ", "☀️ ", "☀️ ",
    ],
};

/// Christmas spinner pattern
const CHRISTMAS: SpinnerPattern = SpinnerPattern {
    frames: &["🌲", "🎄"],
};

/// Point spinner pattern
const POINT: SpinnerPattern = SpinnerPattern {
    frames: &["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"],
};

/// Layer spinner pattern
const LAYER: SpinnerPattern = SpinnerPattern {
    frames: &["-", "=", "≡"],
};

/// Beta wave spinner pattern
const BETA_WAVE: SpinnerPattern = SpinnerPattern {
    frames: &[
        "ρββββββ",
        "βρβββββ",
        "ββρββββ",
        "βββρβββ",
        "ββββρββ",
        "βββββρβ",
        "ββββββρ",
    ],
};

/// Aesthetic spinner pattern
const AESTHETIC: SpinnerPattern = SpinnerPattern {
    frames: &[
        "▰▱▱▱▱▱▱",
        "▰▰▱▱▱▱▱",
        "▰▰▰▱▱▱▱",
        "▰▰▰▰▱▱▱",
        "▰▰▰▰▰▱▱",
        "▰▰▰▰▰▰▱",
        "▰▰▰▰▰▰▰",
        "▰▱▱▱▱▱▱",
    ],
};

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Spinner {
    /// Get the frames for the current spinner type
    fn get_frames(&self) -> Vec<String> {
        match &self.spinner_type {
            SpinnerType::Custom(frames) => frames.clone(),
            _ => {
                let pattern = match &self.spinner_type {
                    SpinnerType::Dots => &DOTS,
                    SpinnerType::Dots2 => &DOTS2,
                    SpinnerType::Dots3 => &DOTS3,
                    SpinnerType::Line => &LINE,
                    SpinnerType::Line2 => &LINE2,
                    SpinnerType::Pipe => &PIPE,
                    SpinnerType::SimpleDots => &SIMPLE_DOTS,
                    SpinnerType::SimpleDotsScrolling => &SIMPLE_DOTS_SCROLLING,
                    SpinnerType::Star => &STAR,
                    SpinnerType::Star2 => &STAR2,
                    SpinnerType::Flip => &FLIP,
                    SpinnerType::Hamburger => &HAMBURGER,
                    SpinnerType::GrowVertical => &GROW_VERTICAL,
                    SpinnerType::GrowHorizontal => &GROW_HORIZONTAL,
                    SpinnerType::Balloon => &BALLOON,
                    SpinnerType::Balloon2 => &BALLOON2,
                    SpinnerType::Noise => &NOISE,
                    SpinnerType::Bounce => &BOUNCE,
                    SpinnerType::BoxBounce => &BOX_BOUNCE,
                    SpinnerType::BoxBounce2 => &BOX_BOUNCE2,
                    SpinnerType::Triangle => &TRIANGLE,
                    SpinnerType::Binary => &BINARY,
                    SpinnerType::Arc => &ARC,
                    SpinnerType::Circle => &CIRCLE,
                    SpinnerType::SquareCorners => &SQUARE_CORNERS,
                    SpinnerType::CircleQuarters => &CIRCLE_QUARTERS,
                    SpinnerType::CircleHalves => &CIRCLE_HALVES,
                    SpinnerType::Squish => &SQUISH,
                    SpinnerType::Toggle => &TOGGLE,
                    SpinnerType::Toggle2 => &TOGGLE2,
                    SpinnerType::Toggle3 => &TOGGLE3,
                    SpinnerType::Arrow => &ARROW,
                    SpinnerType::Arrow2 => &ARROW2,
                    SpinnerType::Arrow3 => &ARROW3,
                    SpinnerType::BouncingBar => &BOUNCING_BAR,
                    SpinnerType::BouncingBall => &BOUNCING_BALL,
                    SpinnerType::Clock => &CLOCK,
                    SpinnerType::Earth => &EARTH,
                    SpinnerType::Moon => &MOON,
                    SpinnerType::Hearts => &HEARTS,
                    SpinnerType::Smiley => &SMILEY,
                    SpinnerType::Monkey => &MONKEY,
                    SpinnerType::Weather => &WEATHER,
                    SpinnerType::Christmas => &CHRISTMAS,
                    SpinnerType::Point => &POINT,
                    SpinnerType::Layer => &LAYER,
                    SpinnerType::BetaWave => &BETA_WAVE,
                    SpinnerType::Aesthetic => &AESTHETIC,
                    SpinnerType::Custom(_) => unreachable!(), // Already handled above
                };
                pattern.frames.iter().map(|&s| s.to_string()).collect()
            }
        }
    }

    /// Creates a new Spinner with default settings
    pub fn new() -> Self {
        Self {
            spinner_type: SpinnerType::default(),
            speed: SpinnerSpeed::default(),
            color: None,
        }
    }

    /// Set the spinner animation type
    pub fn spinner_type(mut self, spinner_type: SpinnerType) -> Self {
        self.spinner_type = spinner_type;
        self
    }

    /// Set the animation speed
    pub fn speed(mut self, speed: SpinnerSpeed) -> Self {
        self.speed = speed;
        self
    }

    /// Set the spinner color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set a custom pattern for the spinner
    ///
    /// # Example
    /// ```ignore
    /// let spinner = Spinner::new()
    ///     .custom_pattern(vec!["◐", "◓", "◑", "◒"])
    ///     .speed(SpinnerSpeed::Normal);
    /// ```
    pub fn custom_pattern<S>(mut self, frames: Vec<S>) -> Self
    where
        S: Into<String>,
    {
        let frames: Vec<String> = frames.into_iter().map(|s| s.into()).collect();
        self.spinner_type = SpinnerType::Custom(frames);
        self
    }

    fn update(&self, ctx: &Context, msg: Box<dyn Message>, _topic: Option<&str>) -> Action {
        if let Some(msg) = msg.downcast::<SpinnerMsg>() {
            let mut state = ctx.get_state::<SpinnerState>();
            match msg {
                SpinnerMsg::Tick => {
                    let frames = self.get_frames();
                    state.frame_index = (state.frame_index + 1) % frames.len();
                    return Action::update(state);
                }
            }
        }
        Action::none()
    }

    fn view(&self, ctx: &Context) -> Node {
        let state = ctx.get_state::<SpinnerState>();
        let frames = self.get_frames();

        // Get current frame
        let frame_index = state.frame_index % frames.len();
        let frame = &frames[frame_index];

        // Create text node with optional color
        let mut text = Text::new(frame);
        if let Some(color) = self.color {
            text.style = Some(TextStyle {
                color: Some(color),
                ..Default::default()
            });
        }

        text.into()
    }

    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        let ctx = ctx.clone();
        let interval = self.speed.interval();

        let effect = Box::pin(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(interval)).await;
                ctx.send(SpinnerMsg::Tick);
            }
        });

        vec![effect]
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations: Spinner
//--------------------------------------------------------------------------------------------------

impl Component for Spinner {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
        Spinner::update(self, ctx, msg, topic)
    }

    fn view(&self, ctx: &Context) -> Node {
        Spinner::view(self, ctx)
    }

    fn effects(&self, ctx: &Context) -> Vec<Effect> {
        Spinner::effects(self, ctx)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
