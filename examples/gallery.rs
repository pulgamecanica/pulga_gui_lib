//! Every widget on one screen, for eyeballing the look and feel.
//! `cargo run --example gallery` (needs a display). Escape quits.

use pulga_gui_lib::{
    ButtonKind, Color, Icon, Input, Rect, Theme, ToastKind, Ui, UiKey, input::button,
    render::GuiRenderer,
};
use pulga_win_lib::{App, Event, Key, MouseButton, WindowConfig, render, run, time::Instant};

struct Gallery {
    ui: Ui,
    gui: Option<GuiRenderer>,
    input: Input,
    size: (f32, f32),
    last: Instant,
    quit: bool,
    // Demo state
    dark: bool,
    selected: usize,
    group_open: bool,
    name: String,
    position: [f32; 3],
    solid: bool,
    snap: bool,
    order: usize,
    kind: usize,
    color: [f32; 3],
    confirm: bool,
}

const PALETTE: [[f32; 3]; 6] = [
    [0.55, 0.55, 0.58],
    [0.86, 0.3, 0.3],
    [0.95, 0.75, 0.2],
    [0.23, 0.51, 0.96],
    [0.25, 0.7, 0.42],
    [0.92, 0.92, 0.94],
];

impl Gallery {
    fn new() -> Self {
        Self {
            ui: Ui::default(),
            gui: None,
            input: Input::default(),
            size: (1280.0, 760.0),
            last: Instant::now(),
            quit: false,
            dark: true,
            selected: 1,
            group_open: true,
            name: "Main hall".into(),
            position: [0.0, 0.5, -5.0],
            solid: true,
            snap: true,
            order: 0,
            kind: 0,
            color: PALETTE[3],
            confirm: false,
        }
    }

    fn build(&mut self) {
        let ui = &mut self.ui;
        let (w, h) = self.size;
        let theme_label = if self.dark {
            "Light theme"
        } else {
            "Dark theme"
        };

        ui.bar("toolbar", Rect::new(0.0, 0.0, w, 44.0), |ui| {
            ui.label_colored("pulga", ui.theme.accent_hover);
            ui.label("gallery");
            ui.space(16.0);
            ui.icon_button(Icon::Undo, "Undo (Ctrl+Z)", false);
            ui.icon_button(Icon::Redo, "Redo (Ctrl+Y)", false);
            ui.icon_button(Icon::Save, "Save (Ctrl+S)", false);
            ui.space(16.0);
            ui.button_with(ButtonKind::Primary, Some(Icon::Play), "Run");
        });
        if ui
            .put(Rect::new(w - 50.0, 7.0, 40.0, 30.0), |ui| {
                ui.icon_button(Icon::Theme, theme_label, false)
            })
            .clicked
        {
            self.dark = !self.dark;
            ui.theme = if self.dark {
                Theme::dark()
            } else {
                Theme::light()
            };
        }

        ui.panel(
            "scene",
            Rect::new(0.0, 44.0, 280.0, h - 44.0 - 28.0),
            |ui| {
                ui.horizontal(|ui| {
                    ui.label_colored("Scene", ui.theme.text_dim);
                    ui.space(70.0);
                    ui.menu_button(Some(Icon::Plus), "New", |ui| {
                        ui.menu_item(Some(Icon::Cube), "Zone");
                        ui.menu_item(Some(Icon::Folder), "Group");
                    });
                });
                ui.separator();
                ui.scroll_area("tree", f32::INFINITY, |ui| {
                    let blue = ui.theme.accent_hover;
                    let rows = [
                        ("Entrance", "straight"),
                        ("Corner", "left"),
                        ("Courtyard", "fork"),
                    ];
                    for (i, (name, kind)) in rows.iter().enumerate() {
                        if ui
                            .list_row(
                                i,
                                0,
                                None,
                                Some(Icon::Cube),
                                name,
                                self.selected == i,
                                Some((kind, blue)),
                            )
                            .clicked
                        {
                            self.selected = i;
                        }
                    }
                    let green = ui.theme.success;
                    if ui
                        .list_row(
                            "g",
                            0,
                            Some(&mut self.group_open),
                            Some(Icon::Folder),
                            "Cluster run",
                            self.selected == 9,
                            Some(("3", green)),
                        )
                        .clicked
                    {
                        self.selected = 9;
                    }
                    if self.group_open {
                        for (i, name) in ["Hall", "Stairs", "Hall"].iter().enumerate() {
                            let key = 10 + i;
                            if ui
                                .list_row(
                                    key,
                                    1,
                                    None,
                                    Some(Icon::Cube),
                                    name,
                                    self.selected == key,
                                    None,
                                )
                                .clicked
                            {
                                self.selected = key;
                            }
                        }
                    }
                });
            },
        );

        ui.panel("inspector", Rect::new(w - 320.0, 44.0, 320.0, h - 44.0 - 28.0), |ui| {
            ui.heading("Zone");
            ui.text_input("name", &mut self.name, "Zone name");
            ui.dropdown("kind", &mut self.kind, &["Straight", "Left turn", "Right turn", "Fork"]);
            ui.heading("Transform");
            let axes = ui.theme.axis;
            ui.columns(3, |ui| {
                for (i, axis) in ["X", "Y", "Z"].iter().enumerate() {
                    ui.drag_value(axis, Some((axis, axes[i])), &mut self.position[i], 0.25, 2);
                }
            });
            ui.heading("Physics");
            ui.toggle(&mut self.solid, "Solid (has mass)");
            ui.checkbox(&mut self.snap, "Snap to grid");
            ui.heading("Look");
            ui.color_swatches(&mut self.color, &PALETTE);
            ui.heading("Group order");
            ui.segmented("order", &mut self.order, &[(Some(Icon::Sequence), "Sequence"), (Some(Icon::Shuffle), "Shuffle")]);
            ui.space(8.0);
            ui.columns(2, |ui| {
                if ui.button_with(ButtonKind::Normal, Some(Icon::Copy), "Duplicate").clicked {
                    ui.notify("Zone duplicated", ToastKind::Success);
                }
                if ui.button_with(ButtonKind::Danger, Some(Icon::Trash), "Delete").clicked {
                    self.confirm = true;
                }
            });
            ui.caption("Drag a value sideways to change it, click it to type.\nShift drags in finer steps.");
        });

        // The "viewport" between the panels.
        let center = Rect::new(280.0, 44.0, w - 600.0, h - 72.0);
        let bg = ui.theme.background;
        ui.paint(pulga_gui_lib::DrawCommand::Rect {
            rect: center,
            color: bg,
        });
        let faint = ui.theme.text_faint;
        ui.text_at(center.x + 16.0, center.y + 16.0, 20.0, faint, "Viewport");

        ui.bar("status", Rect::new(0.0, h - 28.0, w, 28.0), |ui| {
            ui.caption("Ready  ·  3 zones, 1 group  ·  Right-drag to look, WASD to fly");
        });

        if self.confirm {
            let mut close = false;
            ui.modal("Delete zone?", 380.0, |ui| {
                ui.caption("\"Main hall\" and its 4 props will be removed.\nYou can undo this.");
                ui.space(10.0);
                ui.columns(2, |ui| {
                    close |= ui.button("Cancel").clicked;
                    if ui.button_with(ButtonKind::Danger, None, "Delete").clicked {
                        ui.notify("Zone deleted", ToastKind::Error);
                        close = true;
                    }
                });
            });
            if close || ui.input().keys.contains(&UiKey::Escape) {
                self.confirm = false;
            }
        }
    }
}

fn ui_key(key: Key) -> Option<UiKey> {
    Some(match key {
        Key::Left => UiKey::Left,
        Key::Right => UiKey::Right,
        Key::Up => UiKey::Up,
        Key::Down => UiKey::Down,
        Key::Home => UiKey::Home,
        Key::End => UiKey::End,
        Key::Backspace => UiKey::Backspace,
        Key::Delete => UiKey::Delete,
        Key::Enter => UiKey::Enter,
        Key::Escape => UiKey::Escape,
        Key::Tab => UiKey::Tab,
        _ => return None,
    })
}

impl App for Gallery {
    fn init(&mut self) {
        self.gui = Some(unsafe { GuiRenderer::new() });
    }

    fn event(&mut self, event: Event) {
        let index = |b: MouseButton| match b {
            MouseButton::Left => button::LEFT,
            MouseButton::Right => button::RIGHT,
            MouseButton::Middle => button::MIDDLE,
        };
        match event {
            Event::Resized { width, height } => self.size = (width as f32, height as f32),
            Event::MouseMoved { x, y } => self.input.mouse = (x, y),
            Event::MouseDown(b) => {
                self.input.down[index(b)] = true;
                self.input.pressed[index(b)] = true;
            }
            Event::MouseUp(b) => {
                self.input.down[index(b)] = false;
                self.input.released[index(b)] = true;
            }
            Event::Wheel(lines) => self.input.wheel += lines,
            Event::Text(c) => self.input.text.push(c),
            Event::KeyDown(Key::Shift) => self.input.shift = true,
            Event::KeyUp(Key::Shift) => self.input.shift = false,
            Event::KeyDown(Key::Escape) if !self.confirm && !self.ui.wants_keyboard() => {
                self.quit = true
            }
            Event::KeyDown(key) => self.input.keys.extend(ui_key(key)),
            _ => {}
        }
    }

    fn draw(&mut self) {
        let now = Instant::now();
        self.input.dt = (now - self.last).as_secs_f32();
        self.last = now;
        self.ui.begin_frame(
            self.input.clone(),
            Rect::new(0.0, 0.0, self.size.0, self.size.1),
        );
        self.input.end_frame();
        self.build();
        let commands = self.ui.end_frame();
        let Color { r, g, b, .. } = self.ui.theme.background;
        unsafe {
            render::clear([r, g, b, 1.0]);
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
            width: 1280,
            height: 760,
        },
        Gallery::new(),
    )
}
