use crate::geometry::{Color, Rect};

/// Rendering is kept out of the widget logic entirely: a widget just
/// describes what it looks like as a list of commands. A backend
/// ([`crate::render::GuiRenderer`] on native) interprets them. This is also
/// what makes widget tests portable to wasm32 — no GL context needed.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    /// A solid rectangle.
    Rect { rect: Rect, color: Color },
    /// Text in the built-in [`crate::font`], top-left at (`x`, `y`), `size`
    /// pixels per line.
    Text {
        x: f32,
        y: f32,
        size: f32,
        color: Color,
        text: String,
    },
}

impl DrawCommand {
    /// `text` centered inside `rect`, as large as fits in `rect`'s height
    /// while staying a crisp multiple of 8 pixels (at least 8).
    pub fn centered_text(rect: Rect, color: Color, text: &str) -> Self {
        let size = ((rect.h * 0.5 / 8.0).floor() * 8.0).max(8.0);
        let w = crate::font::text_width(text, size);
        DrawCommand::Text {
            x: rect.x + (rect.w - w) / 2.0,
            y: rect.y + (rect.h - size) / 2.0,
            size,
            color,
            text: text.to_owned(),
        }
    }
}

pub trait Widget {
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, bounds: Rect);
    fn draw_commands(&self) -> Vec<DrawCommand>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn centered_text_is_centered_and_crisp() {
        let white = Color::rgb(1.0, 1.0, 1.0);
        // 40px tall -> size 16; "AB" at 16px is 24px wide.
        let cmd = DrawCommand::centered_text(Rect::new(0.0, 0.0, 100.0, 40.0), white, "AB");
        let DrawCommand::Text { x, y, size, .. } = cmd else {
            panic!("expected text")
        };
        assert_eq!((x, y, size), (38.0, 12.0, 16.0));
    }
}
