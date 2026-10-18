use crate::geometry::{Color, Rect};
use crate::widget::{DrawCommand, Widget};

/// Cycles through a fixed set of options (e.g. a character picker).
pub struct Selector {
    options: Vec<String>,
    index: usize,
    pub color: Color,
    bounds: Rect,
}

impl Selector {
    /// # Panics
    /// Panics if `options` is empty — a selector needs at least one option.
    pub fn new(options: Vec<String>) -> Self {
        assert!(!options.is_empty(), "Selector needs at least one option");
        Self { options, index: 0, color: Color::rgb(0.4, 0.4, 0.7), bounds: Rect::new(0.0, 0.0, 0.0, 0.0) }
    }

    pub fn selected(&self) -> &str {
        &self.options[self.index]
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.options.len();
    }

    pub fn prev(&mut self) {
        self.index = (self.index + self.options.len() - 1) % self.options.len();
    }
}

impl Widget for Selector {
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
    fn next_wraps_around() {
        let mut s = Selector::new(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(s.selected(), "a");
        s.next();
        assert_eq!(s.selected(), "b");
        s.next();
        s.next();
        assert_eq!(s.selected(), "a");
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn prev_wraps_around() {
        let mut s = Selector::new(vec!["a".into(), "b".into(), "c".into()]);
        s.prev();
        assert_eq!(s.selected(), "c");
    }

    // should_panic isn't supported by wasm_bindgen_test, so this one stays native-only.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[should_panic(expected = "at least one option")]
    fn empty_options_panics() {
        Selector::new(vec![]);
    }
}
