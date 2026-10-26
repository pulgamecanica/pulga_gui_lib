//! Native GL backend: draws [`DrawCommand`]s with `pulga_win_lib`'s
//! [`Canvas2d`] (the font and icon atlas becomes one texture).
//!
//! ```no_run
//! # use pulga_gui_lib::{Input, Rect, Ui, render::GuiRenderer};
//! let mut ui = Ui::default();
//! # unsafe {
//! let mut gui = GuiRenderer::new(); // once, with a current GL context
//! // every frame, after drawing the 3D scene:
//! ui.begin_frame(Input::default(), Rect::new(0.0, 0.0, 1280.0, 720.0));
//! ui.label("Hello");
//! gui.draw(&ui.end_frame(), 1280.0, 720.0);
//! # }
//! ```

use pulga_win_lib::canvas::Canvas2d;
use pulga_win_lib::texture::{Filter, Texture};

use crate::draw::DrawCommand;
use crate::{font, icons};

/// See the [module docs](self).
pub struct GuiRenderer {
    canvas: Canvas2d,
    atlas: Texture,
}

impl GuiRenderer {
    /// Creates the canvas and uploads the font/icon atlas.
    ///
    /// # Safety
    /// Requires a current GL context.
    pub unsafe fn new() -> Self {
        let (pixels, w, h) = font::atlas();
        unsafe {
            Self {
                canvas: Canvas2d::new(),
                atlas: Texture::from_rgba8(w as u32, h as u32, &pixels, Filter::Nearest),
            }
        }
    }

    /// Draws `commands` in order over a `width`×`height` pixel window.
    ///
    /// # Safety
    /// Requires a current GL context.
    pub unsafe fn draw(&mut self, commands: &[DrawCommand], width: f32, height: f32) {
        for cmd in commands {
            match cmd {
                DrawCommand::Rect { rect, color } => {
                    self.canvas
                        .rect(rect.x, rect.y, rect.w, rect.h, color.to_array())
                }
                DrawCommand::RoundedRect {
                    rect,
                    radius,
                    fill,
                    border,
                    border_color,
                } => self.canvas.rounded_rect(
                    rect.to_array(),
                    *radius,
                    fill.to_array(),
                    *border,
                    border_color.to_array(),
                ),
                DrawCommand::Shadow {
                    rect,
                    radius,
                    blur,
                    color,
                } => self
                    .canvas
                    .shadow(rect.to_array(), *radius, *blur, color.to_array()),
                DrawCommand::Text {
                    x,
                    y,
                    size,
                    color,
                    text,
                } => {
                    for (dst, uv) in font::layout(*x, *y, *size, text) {
                        self.canvas.image(&self.atlas, dst, uv, color.to_array());
                    }
                }
                DrawCommand::Icon { rect, icon, color } => self.canvas.image(
                    &self.atlas,
                    rect.to_array(),
                    icons::uv(*icon),
                    color.to_array(),
                ),
                DrawCommand::Clip(clip) => self.canvas.set_clip(clip.map(|r| r.to_array())),
            }
        }
        unsafe { self.canvas.flush(width, height) };
    }
}
