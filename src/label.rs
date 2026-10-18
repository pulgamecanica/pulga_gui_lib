use crate::geometry::{Color, Rect};
use crate::widget::{DrawCommand, Widget};

/// Text widget. Glyph rendering isn't built yet (that needs its own font
/// pipeline), so `draw_commands` just emits the text's background plate for
/// now — enough to see it positioned in the gallery.
pub struct Label {
    pub text: String,
    pub color: Color,
    bounds: Rect,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into(), color: Color::rgb(0.9, 0.9, 0.9), bounds: Rect::new(0.0, 0.0, 0.0, 0.0) }
    }
}

impl Widget for Label {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw_commands(&self) -> Vec<DrawCommand> {
        vec![DrawCommand::Rect { rect: self.bounds, color: self.color }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn layout_moves_bounds() {
        let mut l = Label::new("score: 0");
        l.set_bounds(Rect::new(5.0, 5.0, 100.0, 20.0));
        assert_eq!(l.bounds(), Rect::new(5.0, 5.0, 100.0, 20.0));
    }
}
