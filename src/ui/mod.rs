//! The immediate-mode UI.
//!
//! Every frame, the application describes its whole interface by calling
//! widget functions on a [`Ui`]; each call draws the widget and returns a
//! [`Response`] saying what the user did to it *this frame*. There is no
//! widget tree to keep in sync with application data: the code that draws a
//! checkbox is the code that reads and writes the `bool` behind it.
//!
//! ```
//! use pulga_gui_lib::{Input, Rect, Ui};
//!
//! let mut ui = Ui::default();
//! let mut sound = true;
//! let mut name = String::from("Marvin");
//!
//! // Once per frame:
//! ui.begin_frame(Input::default(), Rect::new(0.0, 0.0, 1280.0, 720.0));
//! ui.panel("settings", Rect::new(0.0, 0.0, 300.0, 720.0), |ui| {
//!     ui.heading("Player");
//!     ui.text_input("name", &mut name, "Name");
//!     ui.toggle(&mut sound, "Sound");
//!     if ui.button("Start").clicked {
//!         // start the game
//!     }
//! });
//! let commands = ui.end_frame(); // hand these to a renderer
//! assert!(!commands.is_empty());
//! ```
//!
//! How it works, briefly (the classic IMGUI design):
//!
//! - Widgets are identified by an [`Id`] hashed from their label or key and
//!   the ids of the containers they're in ([`Ui::push_id`] disambiguates
//!   repeated labels, e.g. rows of a list).
//! - The widget under the mouse is *hot*; the one the mouse went down on is
//!   *active* until release (so a drag keeps its widget even off its
//!   rectangle); the text field taking keystrokes has *focus*.
//! - Containers ([`Ui::panel`], [`Ui::horizontal`], [`Ui::scroll_area`], ...)
//!   push a layout frame; widgets ask it for space in order.
//! - Output is sorted into [`Layer`]s (base, modal, popup, tooltip) so
//!   popups draw over panels and block the mouse from reaching them.
//! - The little state that must outlive a frame (scroll offsets, open tree
//!   nodes, a text cursor) lives in the `Ui`, keyed by id.

mod containers;
mod widgets;

pub use containers::ToastKind;
pub use widgets::ButtonKind;

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::draw::DrawCommand;
use crate::font;
use crate::geometry::{Color, Rect};
use crate::icons::{ICON_SIZE, Icon};
use crate::input::{Input, UiKey};
use crate::theme::Theme;

/// A widget's identity across frames.
pub type Id = u64;

/// Drawing and input depth: higher layers draw on top and keep the mouse
/// from reaching lower ones where they cover them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    /// Panels and everything in them.
    Base = 0,
    /// Modal dialogs (they also block the whole base layer).
    Modal = 1,
    /// Dropdowns and menus.
    Popup = 2,
    /// Tooltips and notifications (never take input).
    Tooltip = 3,
}

/// What happened to a widget this frame.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Response {
    /// The widget's id.
    pub id: Id,
    /// Where it was drawn.
    pub rect: Rect,
    /// The mouse is over it (and nothing on top blocks it).
    pub hovered: bool,
    /// The primary button went down on it this frame.
    pub pressed: bool,
    /// Pressed and released on it: a click.
    pub clicked: bool,
    /// The second click of a quick double click.
    pub double_clicked: bool,
    /// The secondary button was pressed on it (e.g. for a context menu).
    pub secondary_clicked: bool,
    /// Held down by the primary button (possibly dragged off it).
    pub dragging: bool,
    /// Mouse movement since last frame while dragging.
    pub drag_delta: (f32, f32),
    /// The value it edits changed this frame.
    pub changed: bool,
    /// A text field was confirmed with Enter this frame.
    pub submitted: bool,
}

impl Response {
    /// Shows `text` in a tooltip once the mouse rests on the widget.
    pub fn on_hover_text(self, ui: &mut Ui, text: &str) -> Self {
        if self.hovered {
            ui.tooltip_request(self.id, text);
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Vertical,
    Horizontal,
}

/// A layout frame: the space a container offers, and how far its children
/// have filled it.
#[derive(Debug, Clone, Copy)]
struct Frame {
    region: Rect,
    cursor: (f32, f32),
    dir: Dir,
    /// Horizontal frames: split the width into this many equal cells.
    cells: Option<usize>,
    /// Bottom-right corner of everything placed so far.
    extent: (f32, f32),
}

impl Frame {
    fn new(region: Rect, dir: Dir) -> Self {
        Self {
            region,
            cursor: (region.x, region.y),
            dir,
            cells: None,
            extent: (region.x, region.y),
        }
    }
}

struct TextEdit {
    id: Id,
    /// Cursor position, in chars.
    cursor: usize,
    /// For numeric fields: the text being typed (strings edit in place).
    buffer: String,
    /// What Escape restores.
    original: String,
}

struct Toast {
    text: String,
    kind: ToastKind,
    age: f32,
}

/// Seconds a toast stays on screen.
const TOAST_LIFETIME: f32 = 3.5;
/// Seconds the mouse must rest on a widget before its tooltip shows.
const TOOLTIP_DELAY: f32 = 0.45;
/// Seconds between the clicks of a double click.
const DOUBLE_CLICK: f32 = 0.35;

/// See the [module docs](self).
pub struct Ui {
    /// The look; change it any time.
    pub theme: Theme,
    input: Input,
    prev_mouse: (f32, f32),
    screen: Rect,
    time: f32,
    layers: [Vec<DrawCommand>; 4],
    layer: Layer,
    frames: Vec<Frame>,
    ids: Vec<Id>,
    clips: Vec<Rect>,
    active: Option<Id>,
    focus: Option<Id>,
    focus_seen: bool,
    text_edit: Option<TextEdit>,
    covers: Vec<(Layer, Rect)>,
    prev_covers: Vec<(Layer, Rect)>,
    modal_open: bool,
    prev_modal_open: bool,
    open_popup: Option<Id>,
    popup_seen: bool,
    hover: Option<(Id, f32)>,
    tooltip: Option<String>,
    last_click: Option<(Id, f32)>,
    /// Pixels the active drag has moved (tells a click from a drag).
    drag_travel: f32,
    /// Sub-step drag movement not yet applied to a [`Ui::drag_value`].
    drag_acc: f32,
    scroll: HashMap<Id, f32>,
    open: HashMap<Id, bool>,
    sizes: HashMap<Id, (f32, f32)>,
    toasts: Vec<Toast>,
}

impl Default for Ui {
    fn default() -> Self {
        Self::new(Theme::default())
    }
}

impl Ui {
    /// A UI drawn with `theme`.
    pub fn new(theme: Theme) -> Self {
        Self {
            theme,
            input: Input::default(),
            prev_mouse: (0.0, 0.0),
            screen: Rect::default(),
            time: 0.0,
            layers: Default::default(),
            layer: Layer::Base,
            frames: Vec::new(),
            ids: Vec::new(),
            clips: Vec::new(),
            active: None,
            focus: None,
            focus_seen: false,
            text_edit: None,
            covers: Vec::new(),
            prev_covers: Vec::new(),
            modal_open: false,
            prev_modal_open: false,
            open_popup: None,
            popup_seen: false,
            hover: None,
            tooltip: None,
            last_click: None,
            drag_travel: 0.0,
            drag_acc: 0.0,
            scroll: HashMap::new(),
            open: HashMap::new(),
            sizes: HashMap::new(),
            toasts: Vec::new(),
        }
    }

    // ----------------------------------------------------------------- frame

    /// Starts a frame: `input` is what happened since the last one, `screen`
    /// the window's rectangle.
    pub fn begin_frame(&mut self, input: Input, screen: Rect) {
        self.prev_mouse = self.input.mouse;
        self.input = input;
        self.time += self.input.dt;
        self.screen = screen;
        for layer in &mut self.layers {
            layer.clear();
        }
        self.layer = Layer::Base;
        self.frames = vec![Frame::new(screen, Dir::Vertical)];
        self.ids.clear();
        self.clips = vec![screen];
        self.prev_covers = std::mem::take(&mut self.covers);
        self.prev_modal_open = std::mem::replace(&mut self.modal_open, false);
        self.focus_seen = false;
        self.popup_seen = false;
        self.tooltip = None;
        if !self.input.down[0] && !self.input.released[0] {
            self.active = None; // the widget that took the press is gone
        }
    }

    /// Ends the frame and returns everything to draw, bottom layer first.
    pub fn end_frame(&mut self) -> Vec<DrawCommand> {
        if !self.focus_seen {
            self.focus = None;
            self.text_edit = None;
        }
        if !self.popup_seen {
            self.open_popup = None;
        }
        if self.input.released[0] {
            self.active = None;
        }
        self.draw_toasts();
        self.draw_tooltip();
        let mut out = Vec::new();
        for layer in &mut self.layers {
            out.append(layer);
            out.push(DrawCommand::Clip(None));
        }
        out
    }

    // ------------------------------------------------------------ queries

    /// This frame's input.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// Whether `key` was pressed this frame and no text field is typing.
    pub fn key_pressed(&self, key: UiKey) -> bool {
        self.focus.is_none() && self.input.keys.contains(&key)
    }

    /// Whether the mouse is over the UI or dragging one of its widgets, so
    /// the application should ignore it (e.g. not orbit a 3D view).
    pub fn wants_pointer(&self) -> bool {
        let (mx, my) = self.input.mouse;
        self.active.is_some()
            || self.prev_modal_open
            || self
                .covers
                .iter()
                .chain(&self.prev_covers)
                .any(|(_, r)| r.contains(mx, my))
    }

    /// Whether a text field has keyboard focus, so the application should
    /// ignore key shortcuts.
    pub fn wants_keyboard(&self) -> bool {
        self.focus.is_some()
    }

    /// The window rectangle.
    pub fn screen(&self) -> Rect {
        self.screen
    }

    // ---------------------------------------------------------------- ids

    /// The id `key` gets inside the current id scope.
    pub fn id(&self, key: impl Hash) -> Id {
        let mut h = DefaultHasher::new();
        self.ids.last().hash(&mut h);
        key.hash(&mut h);
        h.finish()
    }

    /// Runs `f` with ids scoped under `key`, so identical labels in
    /// different rows don't collide.
    pub fn push_id<R>(&mut self, key: impl Hash, f: impl FnOnce(&mut Ui) -> R) -> R {
        let id = self.id(key);
        self.ids.push(id);
        let r = f(self);
        self.ids.pop();
        r
    }

    // ------------------------------------------------------------ painting

    /// Queues a raw draw command on the current layer.
    pub fn paint(&mut self, cmd: DrawCommand) {
        self.layers[self.layer as usize].push(cmd);
    }

    fn paint_rounded(
        &mut self,
        rect: Rect,
        radius: f32,
        fill: Color,
        border: f32,
        border_color: Color,
    ) {
        self.paint(DrawCommand::RoundedRect {
            rect,
            radius,
            fill,
            border,
            border_color,
        });
    }

    fn paint_text(&mut self, x: f32, y: f32, size: f32, color: Color, text: &str) {
        self.paint(DrawCommand::Text {
            x: x.round(),
            y: y.round(),
            size,
            color,
            text: text.to_owned(),
        });
    }

    /// Text vertically centered in `rect` (on its capitals), left-aligned
    /// at `x`.
    fn paint_text_centered_y(&mut self, x: f32, rect: Rect, size: f32, color: Color, text: &str) {
        let cap = size * font::CAP_H as f32 / font::CELL_H as f32;
        self.paint_text(x, rect.y + (rect.h - cap) / 2.0, size, color, text);
    }

    /// An icon centered in `rect` at the theme's icon scale.
    fn paint_icon(&mut self, rect: Rect, icon: Icon, color: Color) {
        let s = ICON_SIZE as f32 * self.theme.icon_scale;
        let (cx, cy) = rect.center();
        self.paint(DrawCommand::Icon {
            rect: Rect::new((cx - s / 2.0).round(), (cy - s / 2.0).round(), s, s),
            icon,
            color,
        });
    }

    fn icon_size(&self) -> f32 {
        ICON_SIZE as f32 * self.theme.icon_scale
    }

    /// Text at an absolute position, for HUDs and overlays.
    pub fn text_at(&mut self, x: f32, y: f32, size: f32, color: Color, text: &str) {
        self.paint_text(x, y, size, color, text);
    }

    // ------------------------------------------------------------ clipping

    fn clip(&self) -> Rect {
        *self.clips.last().unwrap()
    }

    fn push_clip(&mut self, rect: Rect) {
        let clip = self.clip().intersect(&rect);
        self.clips.push(clip);
        self.paint(DrawCommand::Clip(Some(clip)));
    }

    fn pop_clip(&mut self) {
        self.clips.pop();
        let clip = self.clip();
        self.paint(DrawCommand::Clip(Some(clip)));
    }

    // -------------------------------------------------------------- layout

    fn frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    /// The space left in the current container, from the cursor on.
    pub fn available(&self) -> Rect {
        let f = self.frames.last().unwrap();
        Rect::new(
            f.cursor.0,
            f.cursor.1,
            (f.region.right() - f.cursor.0).max(0.0),
            (f.region.bottom() - f.cursor.1).max(0.0),
        )
    }

    /// Takes the next `w`×`h` of the current container (`w =
    /// f32::INFINITY` fills the remaining width) and returns where it is.
    pub fn allocate(&mut self, w: f32, h: f32) -> Rect {
        let spacing = self.theme.spacing;
        let f = self.frame();
        let width = match (f.dir, f.cells) {
            (Dir::Horizontal, Some(n)) => (f.region.w - spacing * (n as f32 - 1.0)) / n as f32,
            _ if w.is_infinite() => (f.region.right() - f.cursor.0).max(0.0),
            _ => w,
        };
        let rect = Rect::new(f.cursor.0, f.cursor.1, width, h);
        match f.dir {
            Dir::Vertical => f.cursor.1 += h + spacing,
            Dir::Horizontal => f.cursor.0 += width + spacing,
        }
        f.extent = (f.extent.0.max(rect.right()), f.extent.1.max(rect.bottom()));
        rect
    }

    /// Whether the current container lays widgets out side by side with
    /// free widths (so widgets should size to their content).
    fn in_free_row(&self) -> bool {
        let f = self.frames.last().unwrap();
        f.dir == Dir::Horizontal && f.cells.is_none()
    }

    /// Leaves `px` pixels of empty space.
    pub fn space(&mut self, px: f32) {
        let f = self.frame();
        match f.dir {
            Dir::Vertical => f.cursor.1 += px,
            Dir::Horizontal => f.cursor.0 += px,
        }
    }

    /// Runs `f` in a child frame over `region`, returning its result and
    /// the bottom-right of what it placed.
    fn with_frame<R>(&mut self, mut frame: Frame, f: impl FnOnce(&mut Ui) -> R) -> (R, (f32, f32)) {
        frame.extent = (frame.region.x, frame.region.y);
        self.frames.push(frame);
        let r = f(self);
        let extent = self.frames.pop().unwrap().extent;
        (r, extent)
    }

    /// Lays the widgets `f` adds out left to right, each sized to its
    /// content; the row is as tall as its tallest widget.
    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.row_impl(None, f)
    }

    /// Lays the widgets `f` adds out left to right in `cells` equal-width
    /// columns spanning the container.
    pub fn columns<R>(&mut self, cells: usize, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.row_impl(Some(cells.max(1)), f)
    }

    fn row_impl<R>(&mut self, cells: Option<usize>, f: impl FnOnce(&mut Ui) -> R) -> R {
        let avail = self.available();
        let mut frame = Frame::new(
            Rect::new(avail.x, avail.y, avail.w, f32::INFINITY),
            Dir::Horizontal,
        );
        frame.cells = cells;
        let (r, extent) = self.with_frame(frame, f);
        self.allocate(f32::INFINITY, (extent.1 - avail.y).max(0.0));
        r
    }

    /// Lays `f`'s widgets out top to bottom, shifted right by `px`.
    pub fn indent<R>(&mut self, px: f32, f: impl FnOnce(&mut Ui) -> R) -> R {
        let avail = self.available();
        let region = Rect::new(
            avail.x + px,
            avail.y,
            (avail.w - px).max(0.0),
            f32::INFINITY,
        );
        let (r, extent) = self.with_frame(Frame::new(region, Dir::Vertical), f);
        let used = (extent.1 - avail.y - self.theme.spacing).max(0.0);
        self.allocate(f32::INFINITY, used);
        r
    }

    /// Lays `f`'s widgets out top to bottom inside `rect`, wherever it is,
    /// without taking space from the current container (overlays,
    /// toolbars aligned to an edge).
    pub fn put<R>(&mut self, rect: Rect, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.with_frame(Frame::new(rect, Dir::Vertical), f).0
    }

    /// Like [`Ui::put`], but left to right.
    pub fn put_horizontal<R>(&mut self, rect: Rect, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.with_frame(Frame::new(rect, Dir::Horizontal), f).0
    }

    // ---------------------------------------------------------- interaction

    /// Marks `rect` as covered by the current layer this frame: the mouse
    /// over it belongs to the UI, and lower layers don't get it next frame.
    fn cover(&mut self, rect: Rect) {
        self.covers.push((self.layer, rect));
    }

    /// Whether the mouse can reach a widget at `rect` on the current layer.
    fn pointer_reaches(&self, rect: Rect) -> bool {
        let (mx, my) = self.input.mouse;
        rect.contains(mx, my)
            && self.clip().contains(mx, my)
            && !(self.prev_modal_open && self.layer < Layer::Modal)
            && !self
                .prev_covers
                .iter()
                .any(|(l, r)| *l > self.layer && r.contains(mx, my))
    }

    /// The hot/active state machine behind every interactive widget.
    pub fn interact(&mut self, id: Id, rect: Rect) -> Response {
        let reaches = self.pointer_reaches(rect);
        let hovered = reaches && self.active.is_none_or(|a| a == id);
        let mut r = Response {
            id,
            rect,
            hovered,
            ..Response::default()
        };
        if hovered && self.input.pressed[0] {
            self.active = Some(id);
            r.pressed = true;
        }
        if hovered && self.input.pressed[1] {
            r.secondary_clicked = true;
        }
        if self.active == Some(id) {
            if self.input.down[0] {
                r.dragging = true;
            }
            if self.input.down[0] && !r.pressed {
                // Movement counts from the frame after the press.
                r.drag_delta = (
                    self.input.mouse.0 - self.prev_mouse.0,
                    self.input.mouse.1 - self.prev_mouse.1,
                );
            }
            if self.input.released[0] {
                self.active = None;
                r.clicked = reaches;
                if r.clicked {
                    r.double_clicked = self
                        .last_click
                        .is_some_and(|(last, t)| last == id && self.time - t < DOUBLE_CLICK);
                    self.last_click = Some((id, self.time));
                }
            }
        }
        r
    }

    fn tooltip_request(&mut self, id: Id, text: &str) {
        let since = match self.hover {
            Some((h, t)) if h == id => t,
            _ => {
                self.hover = Some((id, self.time));
                self.time
            }
        };
        if self.time - since >= TOOLTIP_DELAY {
            self.tooltip = Some(text.to_owned());
        }
    }

    fn draw_tooltip(&mut self) {
        let Some(text) = self.tooltip.take() else {
            return;
        };
        let size = self.theme.text_size;
        let pad = self.theme.padding * 0.8;
        let w = font::text_width(&text, size) + 2.0 * pad;
        let h = size + pad;
        let (mx, my) = self.input.mouse;
        let x = (mx + 14.0).min(self.screen.right() - w - 4.0);
        let y = (my + 20.0).min(self.screen.bottom() - h - 4.0);
        let rect = Rect::new(x, y, w, h);
        let prev = std::mem::replace(&mut self.layer, Layer::Tooltip);
        self.paint(DrawCommand::Clip(None));
        self.paint(DrawCommand::Shadow {
            rect,
            radius: self.theme.radius_small,
            blur: 10.0,
            color: self.theme.shadow,
        });
        let (fill, border, text_color) = (self.theme.surface, self.theme.border, self.theme.text);
        self.paint_rounded(rect, self.theme.radius_small, fill, 1.0, border);
        self.paint_text_centered_y(x + pad, rect, size, text_color, &text);
        self.layer = prev;
    }
}

#[cfg(test)]
mod tests;
