//! GUI toolkit for the `42run` project: an immediate-mode [`Ui`] with the
//! widgets an editor needs, a customizable [`Theme`], a built-in bitmap
//! [`font`] and pixel [`icons`].
//!
//! The design keeps three concerns apart:
//!
//! - **Logic** ([`Ui`]): layout, interaction state, widgets. Plain Rust with
//!   no GL or windowing dependency, unit-tested on native and on
//!   `wasm32-unknown-unknown`.
//! - **Input** ([`Input`]): a plain snapshot the host fills from whatever
//!   window library it uses.
//! - **Output** ([`DrawCommand`]): what to draw, interpreted by a backend:
//!   [`render::GuiRenderer`] on native, through `pulga_win_lib`'s canvas.
//!
//! See [`ui`] for how a frame works.

pub mod draw;
pub mod font;
pub mod geometry;
pub mod icons;
pub mod input;
#[cfg(not(target_arch = "wasm32"))]
pub mod render;
pub mod theme;
pub mod ui;

pub use draw::DrawCommand;
pub use geometry::{Color, Rect};
pub use icons::Icon;
pub use input::{Input, UiKey};
pub use theme::Theme;
pub use ui::{ButtonKind, Id, Layer, Response, ToastKind, Ui};

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
