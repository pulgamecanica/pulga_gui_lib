use crate::geometry::{Color, Rect};
use crate::widget::{DrawCommand, Widget};

/// A line of text, centered vertically in its bounds and left-aligned, as
/// large as the bounds' height allows (see [`DrawCommand::centered_text`]).
pub struct Label {
    pub text: String,
    pub color: Color,
    bounds: Rect,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::rgb(0.9, 0.9, 0.9),
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
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
        let mut cmd = DrawCommand::centered_text(self.bounds, self.color, &self.text);
        if let DrawCommand::Text { x, .. } = &mut cmd {
            *x = self.bounds.x;
        }
        vec![cmd]
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
