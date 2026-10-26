//! The renderer-independent drawing vocabulary.
//!
//! The UI never draws pixels itself: it produces a list of
//! [`DrawCommand`]s that a backend interprets ([`crate::render::GuiRenderer`]
//! on native GL). This keeps widget logic testable without a GPU, on native
//! and on wasm32, and lets other backends exist without touching widgets.

use crate::geometry::{Color, Rect};
use crate::icons::Icon;

/// One thing to draw, in window pixels (origin top-left, y down).
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    /// A solid rectangle with square corners.
    Rect {
        /// Where.
        rect: Rect,
        /// Fill.
        color: Color,
    },
    /// A rectangle with rounded corners and an optional inner border.
    RoundedRect {
        /// Where.
        rect: Rect,
        /// Corner radius in pixels.
        radius: f32,
        /// Fill.
        fill: Color,
        /// Border width in pixels; `0.0` for none.
        border: f32,
        /// Border color.
        border_color: Color,
    },
    /// A soft drop shadow around a rounded rectangle.
    Shadow {
        /// The shape casting the shadow.
        rect: Rect,
        /// Its corner radius.
        radius: f32,
        /// How far the shadow fades out, in pixels.
        blur: f32,
        /// Shadow color (its alpha is the darkest point).
        color: Color,
    },
    /// Text in the built-in [`crate::font`].
    Text {
        /// Left edge.
        x: f32,
        /// Top of the line.
        y: f32,
        /// Line height in pixels.
        size: f32,
        /// Text color.
        color: Color,
        /// The text; `\n` starts a new line.
        text: String,
    },
    /// A built-in [`crate::icons`] icon stretched over `rect`.
    Icon {
        /// Where.
        rect: Rect,
        /// Which.
        icon: Icon,
        /// Tint.
        color: Color,
    },
    /// Restricts the commands after it to a rectangle (`None` lifts it).
    Clip(Option<Rect>),
}
