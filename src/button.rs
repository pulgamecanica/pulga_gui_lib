use crate::geometry::{Color, Rect};
use crate::widget::{DrawCommand, Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Hovered,
    Pressed,
}

pub struct Button {
    pub label: String,
    pub idle_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
    state: ButtonState,
    bounds: Rect,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            idle_color: Color::rgb(0.25, 0.25, 0.3),
            hovered_color: Color::rgb(0.35, 0.35, 0.45),
            pressed_color: Color::rgb(0.15, 0.6, 0.3),
            state: ButtonState::Idle,
            bounds: Rect::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn state(&self) -> ButtonState {
        self.state
    }

    pub fn on_mouse_move(&mut self, x: f32, y: f32) {
        if self.state == ButtonState::Pressed {
            return;
        }
        self.state = if self.bounds.contains(x, y) { ButtonState::Hovered } else { ButtonState::Idle };
    }

    pub fn on_mouse_down(&mut self, x: f32, y: f32) {
        if self.bounds.contains(x, y) {
            self.state = ButtonState::Pressed;
        }
    }

    /// Returns `true` if this release counts as a click (press and release
    /// both happened inside the button).
    pub fn on_mouse_up(&mut self, x: f32, y: f32) -> bool {
        let was_pressed = self.state == ButtonState::Pressed;
        let inside = self.bounds.contains(x, y);
        self.state = if inside { ButtonState::Hovered } else { ButtonState::Idle };
        was_pressed && inside
    }
}

impl Widget for Button {
    fn bounds(&self) -> Rect {
        self.bounds
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    fn draw_commands(&self) -> Vec<DrawCommand> {
        let color = match self.state {
            ButtonState::Idle => self.idle_color,
            ButtonState::Hovered => self.hovered_color,
            ButtonState::Pressed => self.pressed_color,
        };
        vec![DrawCommand::Rect { rect: self.bounds, color }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    fn button_at(x: f32, y: f32, w: f32, h: f32) -> Button {
        let mut b = Button::new("play");
        b.set_bounds(Rect::new(x, y, w, h));
        b
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn hover_tracks_mouse_inside_bounds() {
        let mut b = button_at(0.0, 0.0, 100.0, 40.0);
        assert_eq!(b.state(), ButtonState::Idle);
        b.on_mouse_move(50.0, 20.0);
        assert_eq!(b.state(), ButtonState::Hovered);
        b.on_mouse_move(500.0, 500.0);
        assert_eq!(b.state(), ButtonState::Idle);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn click_requires_press_and_release_inside() {
        let mut b = button_at(0.0, 0.0, 100.0, 40.0);
        b.on_mouse_down(50.0, 20.0);
        assert_eq!(b.state(), ButtonState::Pressed);
        assert!(b.on_mouse_up(50.0, 20.0));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn dragging_off_before_release_is_not_a_click() {
        let mut b = button_at(0.0, 0.0, 100.0, 40.0);
        b.on_mouse_down(50.0, 20.0);
        assert!(!b.on_mouse_up(500.0, 500.0));
        assert_eq!(b.state(), ButtonState::Idle);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn mouse_move_while_pressed_does_not_lose_press_state() {
        let mut b = button_at(0.0, 0.0, 100.0, 40.0);
        b.on_mouse_down(50.0, 20.0);
        b.on_mouse_move(51.0, 21.0);
        assert_eq!(b.state(), ButtonState::Pressed);
    }
}
