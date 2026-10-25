//! Native GL backend: draws [`DrawCommand`]s with `pulga_win_lib`'s
//! [`Canvas2d`], text included (the [`crate::font`] atlas becomes a
//! texture).
//!
//! ```no_run
//! # use pulga_gui_lib::{Button, Widget, geometry::Rect, render::GuiRenderer};
//! let mut button = Button::new("PLAY");
//! button.set_bounds(Rect::new(40.0, 40.0, 200.0, 48.0));
//! # unsafe {
//! let mut gui = GuiRenderer::new(); // once, with a current GL context
//! // every frame, after drawing the 3D scene:
//! gui.draw(&button.draw_commands(), 1280.0, 720.0);
//! # }
//! ```

use pulga_win_lib::canvas::Canvas2d;
use pulga_win_lib::texture::{Filter, Texture};

use crate::font;
use crate::geometry::Color;
use crate::widget::DrawCommand;

/// See the [module docs](self).
pub struct GuiRenderer {
    canvas: Canvas2d,
    font: Texture,
}

fn rgba(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

impl GuiRenderer {
    /// Creates the canvas and uploads the font atlas.
    ///
    /// # Safety
    /// Requires a current GL context.
    pub unsafe fn new() -> Self {
        let (pixels, w, h) = font::atlas();
        unsafe {
            Self {
                canvas: Canvas2d::new(),
                font: Texture::from_rgba8(w as u32, h as u32, &pixels, Filter::Nearest),
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
                        .rect(rect.x, rect.y, rect.w, rect.h, rgba(*color))
                }
                DrawCommand::Text {
                    x,
                    y,
                    size,
                    color,
                    text,
                } => {
                    for (dst, uv) in font::layout(*x, *y, *size, text) {
                        self.canvas.image(&self.font, dst, uv, rgba(*color));
                    }
                }
            }
        }
        unsafe { self.canvas.flush(width, height) };
    }
}
