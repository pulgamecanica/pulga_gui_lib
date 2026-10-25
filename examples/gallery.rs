//! Opens a window and lays out one of each widget so you can eyeball them.
//! `cargo run --example gallery` (needs a display). Mouse hovers/clicks the
//! button, Space cycles the selector, Escape quits.

use pulga_gui_lib::{
    Button, Label, Selector, Widget, geometry::Rect, render::GuiRenderer, stack::layout_vertical,
};
use pulga_win_lib::{App, Event, Key, MouseButton, WindowConfig, render, run};

struct Gallery {
    label: Label,
    button: Button,
    selector: Selector,
    gui: Option<GuiRenderer>,
    size: (f32, f32),
    mouse: (f32, f32),
    quit: bool,
}

impl Gallery {
    fn new() -> Self {
        let mut label = Label::new("42RUN GALLERY");
        let mut button = Button::new("PLAY");
        let mut selector = Selector::new(vec!["Marvin".into(), "Robo".into(), "Pulga".into()]);

        let mut widgets: Vec<&mut dyn Widget> = vec![&mut label, &mut button, &mut selector];
        layout_vertical(
            Rect::new(40.0, 40.0, 300.0, 400.0),
            &mut widgets,
            60.0,
            20.0,
        );

        Self {
            label,
            button,
            selector,
            gui: None,
            size: (1280.0, 720.0),
            mouse: (0.0, 0.0),
            quit: false,
        }
    }
}

impl App for Gallery {
    fn init(&mut self) {
        self.gui = Some(unsafe { GuiRenderer::new() });
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::KeyDown(Key::Escape) => self.quit = true,
            Event::KeyDown(Key::Space) => self.selector.next(),
            Event::Resized { width, height } => self.size = (width as f32, height as f32),
            Event::MouseMoved { x, y } => {
                self.mouse = (x, y);
                self.button.on_mouse_move(x, y);
            }
            Event::MouseDown(MouseButton::Left) => {
                self.button.on_mouse_down(self.mouse.0, self.mouse.1)
            }
            Event::MouseUp(MouseButton::Left) => {
                let clicked = self.button.on_mouse_up(self.mouse.0, self.mouse.1);
                if clicked {
                    self.selector.next();
                }
            }
            _ => {}
        }
    }

    fn draw(&mut self) {
        let mut commands = self.label.draw_commands();
        commands.extend(self.button.draw_commands());
        commands.extend(self.selector.draw_commands());
        unsafe {
            render::clear([0.1, 0.1, 0.12, 1.0]);
            self.gui
                .as_mut()
                .unwrap()
                .draw(&commands, self.size.0, self.size.1);
        }
    }

    fn should_quit(&self) -> bool {
        self.quit
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(
        WindowConfig {
            title: "pulga_gui_lib gallery".into(),
            ..Default::default()
        },
        Gallery::new(),
    )
}
