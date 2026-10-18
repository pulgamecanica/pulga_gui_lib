//! Opens a window and lays out one of each widget so you can eyeball them.
//! `cargo run --example gallery` (needs a display).
//!
//! Rendering here is a crude scissor-rect clear per widget, not a real quad
//! pipeline — good enough to see layout/state visually; a textured/shader
//! renderer is a separate piece of work for when PulgaWinLib's shader
//! support lands.

use PulgaGuiLib::{Button, DrawCommand, Label, Selector, Widget, geometry::Rect, stack::layout_vertical};
use PulgaWinLib::{App, Event, Key, WindowConfig, gl, run};

struct Gallery {
    label: Label,
    button: Button,
    selector: Selector,
    // GL's scissor origin is bottom-left; widget layout is top-left, so we
    // need the window height to flip y when issuing scissor rects.
    window_height: u32,
}

impl Gallery {
    fn new() -> Self {
        let mut label = Label::new("42run");
        let mut button = Button::new("Play");
        let mut selector = Selector::new(vec!["Marvin".into(), "Robo".into(), "Pulga".into()]);

        let mut widgets: Vec<&mut dyn Widget> = vec![&mut label, &mut button, &mut selector];
        layout_vertical(Rect::new(40.0, 40.0, 300.0, 400.0), &mut widgets, 60.0, 20.0);

        Self { label, button, selector, window_height: 720 }
    }

    fn widgets(&self) -> [&dyn Widget; 3] {
        [&self.label, &self.button, &self.selector]
    }
}

impl App for Gallery {
    fn event(&mut self, event: Event) {
        if event == Event::KeyDown(Key::Escape) {
            std::process::exit(0);
        }
        if event == Event::KeyDown(Key::Space) {
            self.selector.next();
        }
        if let Event::Resized { height, .. } = event {
            self.window_height = height;
        }
    }

    fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.12, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Enable(gl::SCISSOR_TEST);
        }
        for widget in self.widgets() {
            for cmd in widget.draw_commands() {
                let DrawCommand::Rect { rect, color } = cmd;
                let flipped_y = self.window_height as f32 - rect.y - rect.h;
                unsafe {
                    gl::Scissor(rect.x as i32, flipped_y as i32, rect.w as i32, rect.h as i32);
                    gl::ClearColor(color.r, color.g, color.b, color.a);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
        }
        unsafe { gl::Disable(gl::SCISSOR_TEST) };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(WindowConfig { title: "PulgaGuiLib gallery".into(), ..Default::default() }, Gallery::new())
}
