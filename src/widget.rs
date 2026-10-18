use crate::geometry::{Color, Rect};

/// Rendering is kept out of the widget logic entirely: a widget just
/// describes what it looks like as a list of commands. A backend (the native
/// GL gallery example today, a WebGL2 one later) interprets them. This is
/// also what makes widget tests portable to wasm32 — no GL context needed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DrawCommand {
    Rect { rect: Rect, color: Color },
}

pub trait Widget {
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, bounds: Rect);
    fn draw_commands(&self) -> Vec<DrawCommand>;
}
