//! GUI widgets for the `42run` project, built on `pulga_win_lib`.
//!
//! Widget logic (layout, hover/press state, selection) is plain Rust with no
//! GL or windowing dependency, so it's unit-tested the same way on native
//! and on `wasm32-unknown-unknown`. Rendering is a separate concern: widgets
//! emit [`widget::DrawCommand`]s that a backend (the native GL gallery
//! example today) turns into pixels.

pub mod button;
pub mod geometry;
pub mod label;
pub mod selector;
pub mod stack;
pub mod widget;

pub use button::{Button, ButtonState};
pub use geometry::{Color, Rect};
pub use label::Label;
pub use selector::Selector;
pub use widget::{DrawCommand, Widget};

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
