//! The widgets: each draws itself, handles its input and returns a
//! [`Response`].

use super::{Id, Response, TextEdit, Ui};
use crate::draw::DrawCommand;
use crate::font;
use crate::geometry::{Color, Rect};
use crate::icons::Icon;
use crate::input::UiKey;

/// The visual weight of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// A regular action.
    Normal,
    /// The main action of a group, in the accent color.
    Primary,
    /// A destructive action.
    Danger,
    /// No background until hovered: toolbars and inline actions.
    Ghost,
}

impl Ui {
    // ---------------------------------------------------------------- text

    /// A line of text.
    pub fn label(&mut self, text: &str) -> Response {
        let color = self.theme.text;
        self.label_colored(text, color)
    }

    /// A line of text in `color`.
    pub fn label_colored(&mut self, text: &str, color: Color) -> Response {
        let size = self.theme.text_size;
        let w = if self.in_free_row() {
            font::text_width(text, size)
        } else {
            f32::INFINITY
        };
        let rect = self.allocate(w, self.theme.row_height * 0.8);
        self.paint_text_centered_y(rect.x, rect, size, color, text);
        Response {
            rect,
            ..Response::default()
        }
    }

    /// Secondary text in the small size: hints, captions.
    pub fn caption(&mut self, text: &str) -> Response {
        let size = self.theme.small_text_size;
        let lines = text.lines().count().max(1) as f32;
        let w = if self.in_free_row() {
            font::text_width(text, size)
        } else {
            f32::INFINITY
        };
        let rect = self.allocate(w, size * lines + 2.0);
        let color = self.theme.text_dim;
        self.paint_text(rect.x, rect.y + 2.0, size, color, text);
        Response {
            rect,
            ..Response::default()
        }
    }

    /// A section title: dim text over a hairline.
    pub fn heading(&mut self, text: &str) -> Response {
        self.space(self.theme.spacing);
        let size = self.theme.text_size;
        let rect = self.allocate(f32::INFINITY, size + 6.0);
        let (dim, line) = (self.theme.text_dim, self.theme.separator);
        self.paint_text(rect.x, rect.y, size, dim, text);
        let tw = font::text_width(text, size) + 10.0;
        if rect.w > tw {
            let y = (rect.y + size * 0.35).round();
            self.paint(DrawCommand::Rect {
                rect: Rect::new(rect.x + tw, y, rect.w - tw, 1.0),
                color: line,
            });
        }
        Response {
            rect,
            ..Response::default()
        }
    }

    /// A horizontal hairline.
    pub fn separator(&mut self) {
        let rect = self.allocate(f32::INFINITY, 1.0);
        let color = self.theme.separator;
        self.paint(DrawCommand::Rect { rect, color });
    }

    /// A small rounded label in `color` (e.g. a kind or a count).
    pub fn badge(&mut self, text: &str, color: Color) -> Response {
        let size = self.theme.small_text_size;
        let w = font::text_width(text, size) + 12.0;
        let rect = self.allocate(w, self.theme.row_height);
        let pill = Rect::new(rect.x, rect.y + (rect.h - 16.0) / 2.0, w, 16.0);
        self.paint_rounded(
            pill,
            8.0,
            color.with_alpha(0.18),
            1.0,
            color.with_alpha(0.45),
        );
        self.paint_text_centered_y(pill.x + 6.0, pill, size, color, text);
        Response {
            rect,
            ..Response::default()
        }
    }

    // ------------------------------------------------------------- buttons

    /// A regular button.
    pub fn button(&mut self, text: &str) -> Response {
        self.button_with(ButtonKind::Normal, None, text)
    }

    /// A button of any [`ButtonKind`], with an optional leading icon.
    pub fn button_with(&mut self, kind: ButtonKind, icon: Option<Icon>, text: &str) -> Response {
        let size = self.theme.text_size;
        let pad = self.theme.padding * 1.2;
        let icon_w = icon.map_or(0.0, |_| {
            self.icon_size() + if text.is_empty() { 0.0 } else { 8.0 }
        });
        let content = font::text_width(text, size) + icon_w;
        let w = if self.in_free_row() {
            content + 2.0 * pad
        } else {
            f32::INFINITY
        };
        let rect = self.allocate(w, self.theme.row_height);
        let id = self.id(("button", text, icon.map(|i| i as u8)));
        let r = self.interact(id, rect);

        let t = &self.theme;
        let (fill, fg) = match kind {
            ButtonKind::Normal => (
                if r.dragging {
                    t.surface_active
                } else if r.hovered {
                    t.surface_hover
                } else {
                    t.surface
                },
                t.text,
            ),
            ButtonKind::Primary => (
                if r.hovered { t.accent_hover } else { t.accent },
                t.on_accent,
            ),
            ButtonKind::Danger => (
                if r.hovered { t.danger_hover } else { t.danger },
                t.on_accent,
            ),
            ButtonKind::Ghost => (
                if r.dragging {
                    t.surface_active
                } else if r.hovered {
                    t.surface_hover
                } else {
                    Color::TRANSPARENT
                },
                t.text,
            ),
        };
        let radius = t.radius_small;
        self.paint_rounded(rect, radius, fill, 0.0, Color::TRANSPARENT);
        let mut x = rect.x + (rect.w - content) / 2.0;
        if let Some(icon) = icon {
            let s = self.icon_size();
            self.paint_icon(Rect::new(x, rect.y, s, rect.h), icon, fg);
            x += icon_w;
        }
        self.paint_text_centered_y(x, rect, size, fg, text);
        r
    }

    /// A square icon-only button (toolbars). Shows `tooltip` on hover;
    /// `active` keeps it highlighted (a toggled tool).
    pub fn icon_button(&mut self, icon: Icon, tooltip: &str, active: bool) -> Response {
        let side = self.theme.row_height;
        let rect = self.allocate(side, side);
        let id = self.id(("icon", icon as u8, tooltip));
        let r = self.interact(id, rect);
        let t = &self.theme;
        let fill = if active {
            t.selection
        } else if r.dragging {
            t.surface_active
        } else if r.hovered {
            t.surface_hover
        } else {
            Color::TRANSPARENT
        };
        let fg = if active {
            t.accent_hover
        } else if r.hovered {
            t.text
        } else {
            t.text_dim
        };
        let radius = t.radius_small;
        self.paint_rounded(rect, radius, fill, 0.0, Color::TRANSPARENT);
        self.paint_icon(rect, icon, fg);
        r.on_hover_text(self, tooltip)
    }

    // ------------------------------------------------------------ toggles

    /// A labelled on/off switch; flips `value` when clicked.
    pub fn toggle(&mut self, value: &mut bool, label: &str) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("toggle", label));
        let mut r = self.interact(id, rect);
        if r.clicked {
            *value = !*value;
            r.changed = true;
        }
        let t = &self.theme;
        let (track_w, track_h) = (34.0, 18.0);
        let track = Rect::new(
            rect.right() - track_w,
            rect.y + (rect.h - track_h) / 2.0,
            track_w,
            track_h,
        );
        let fill = if *value {
            t.accent
        } else if r.hovered {
            t.surface_active
        } else {
            t.surface_hover
        };
        let knob_x = if *value {
            track.right() - track_h + 3.0
        } else {
            track.x + 3.0
        };
        let (text, knob) = (t.text, Color::hex(0xffffff));
        self.paint_rounded(track, track_h / 2.0, fill, 0.0, Color::TRANSPARENT);
        self.paint_rounded(
            Rect::new(knob_x, track.y + 3.0, track_h - 6.0, track_h - 6.0),
            6.0,
            knob,
            0.0,
            Color::TRANSPARENT,
        );
        let size = self.theme.text_size;
        self.paint_text_centered_y(rect.x, rect, size, text, label);
        r
    }

    /// A labelled check box; flips `value` when clicked.
    pub fn checkbox(&mut self, value: &mut bool, label: &str) -> Response {
        let size = self.theme.text_size;
        let box_side = 18.0;
        let w = if self.in_free_row() {
            box_side + 8.0 + font::text_width(label, size)
        } else {
            f32::INFINITY
        };
        let rect = self.allocate(w, self.theme.row_height);
        let id = self.id(("checkbox", label));
        let mut r = self.interact(id, rect);
        if r.clicked {
            *value = !*value;
            r.changed = true;
        }
        let t = &self.theme;
        let b = Rect::new(
            rect.x,
            rect.y + (rect.h - box_side) / 2.0,
            box_side,
            box_side,
        );
        let (fill, border) = if *value {
            (t.accent, t.accent)
        } else {
            (
                if r.hovered {
                    t.surface_hover
                } else {
                    t.surface
                },
                t.border,
            )
        };
        let (on_accent, text) = (t.on_accent, t.text);
        let radius = t.radius_small - 1.0;
        self.paint_rounded(b, radius, fill, 1.0, border);
        if *value {
            self.paint(DrawCommand::Icon {
                rect: b,
                icon: Icon::Check,
                color: on_accent,
            });
        }
        self.paint_text_centered_y(rect.x + box_side + 8.0, rect, size, text, label);
        r
    }

    /// A row of mutually exclusive options; sets `selected` to the clicked
    /// one.
    pub fn segmented(
        &mut self,
        key: &str,
        selected: &mut usize,
        options: &[(Option<Icon>, &str)],
    ) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let base = self.id(("segmented", key));
        let (surface, border, radius) = (
            self.theme.surface,
            self.theme.border,
            self.theme.radius_small,
        );
        self.paint_rounded(rect, radius, surface, 1.0, border);
        let n = options.len().max(1) as f32;
        let seg_w = rect.w / n;
        let mut out = Response {
            id: base,
            rect,
            ..Response::default()
        };
        for (i, (icon, text)) in options.iter().enumerate() {
            let seg = Rect::new(rect.x + i as f32 * seg_w, rect.y, seg_w, rect.h).shrink(2.0);
            let r = self.interact(base.wrapping_add(i as u64 + 1), seg);
            if r.clicked && *selected != i {
                *selected = i;
                out.changed = true;
            }
            out.hovered |= r.hovered;
            let t = &self.theme;
            let on = *selected == i;
            let fill = if on {
                t.accent
            } else if r.hovered {
                t.surface_hover
            } else {
                Color::TRANSPARENT
            };
            let fg = if on { t.on_accent } else { t.text_dim };
            let radius = t.radius_small - 1.0;
            self.paint_rounded(seg, radius, fill, 0.0, Color::TRANSPARENT);
            let size = self.theme.text_size;
            let icon_w = icon.map_or(0.0, |_| self.icon_size() + 6.0);
            let content = icon_w + font::text_width(text, size);
            let mut x = seg.x + (seg.w - content) / 2.0;
            if let Some(icon) = icon {
                let s = self.icon_size();
                self.paint_icon(Rect::new(x, seg.y, s, seg.h), *icon, fg);
                x += icon_w;
            }
            self.paint_text_centered_y(x, seg, size, fg, text);
        }
        out
    }

    /// A row of color swatches; sets `color` to the clicked one. The
    /// swatch matching `color` gets a ring.
    pub fn color_swatches(&mut self, color: &mut [f32; 3], palette: &[[f32; 3]]) -> Response {
        let side = 22.0;
        let rect = self.allocate(f32::INFINITY, side + 4.0);
        let base = self.id("swatches");
        let mut out = Response {
            id: base,
            rect,
            ..Response::default()
        };
        for (i, c) in palette.iter().enumerate() {
            let sw = Rect::new(
                rect.x + 2.0 + i as f32 * (side + 6.0),
                rect.y + 2.0,
                side,
                side,
            );
            if sw.right() > rect.right() {
                break;
            }
            let r = self.interact(base.wrapping_add(i as u64 + 1), sw);
            if r.clicked && color != c {
                *color = *c;
                out.changed = true;
            }
            let fill = Color::rgb(c[0], c[1], c[2]);
            let on = color == c;
            let ring = if on {
                self.theme.text
            } else if r.hovered {
                self.theme.text_dim
            } else {
                self.theme.border
            };
            if on {
                self.paint_rounded(
                    sw.shrink(-3.0),
                    7.0,
                    Color::TRANSPARENT,
                    2.0,
                    self.theme.accent,
                );
            }
            self.paint_rounded(
                sw,
                5.0,
                fill,
                1.0,
                ring.with_alpha(if on { 0.0 } else { 1.0 }),
            );
        }
        out
    }

    // -------------------------------------------------------------- fields

    /// A single-line text field editing `value` in place. `hint` shows
    /// while it's empty. Enter confirms (`submitted`), Escape reverts,
    /// clicking elsewhere keeps the edit.
    pub fn text_input(&mut self, key: &str, value: &mut String, hint: &str) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("text", key));
        let mut r = self.interact(id, rect);
        if r.pressed {
            self.begin_edit(id, value.clone(), value.chars().count());
        }
        let focused = self.focus == Some(id);
        if focused {
            self.focus_seen = true;
            if self.input.pressed[0] && !r.hovered {
                self.end_edit(); // clicked elsewhere: keep the text
            } else if let Some(edit) = self.text_edit.as_mut() {
                let (changed, done) = apply_typing(edit, value, &self.input.text, &self.input.keys);
                r.changed = changed;
                match done {
                    Some(true) => {
                        r.submitted = true;
                        self.end_edit();
                    }
                    Some(false) => {
                        *value = edit.original.clone();
                        r.changed = true;
                        self.end_edit();
                    }
                    None => {}
                }
            }
        }
        self.paint_field(rect, r.hovered, self.focus == Some(id));
        let size = self.theme.text_size;
        let x = rect.x + self.theme.padding;
        if value.is_empty() && self.focus != Some(id) {
            let faint = self.theme.text_faint;
            self.paint_text_centered_y(x, rect, size, faint, hint);
        } else {
            let text = self.theme.text;
            self.push_clip(rect.shrink(2.0));
            self.paint_text_centered_y(x, rect, size, text, value);
            self.paint_caret(id, x, rect, value);
            self.pop_clip();
        }
        r
    }

    /// A number field: drag sideways to change `value` in `step`s (Shift for
    /// fine steps), or click to type a value. `tag` puts a colored label in
    /// front (e.g. an axis name).
    pub fn drag_value(
        &mut self,
        key: &str,
        tag: Option<(&str, Color)>,
        value: &mut f32,
        step: f32,
        decimals: usize,
    ) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("drag", key));
        let mut r = self.interact(id, rect);
        let editing = self.focus == Some(id);

        if editing {
            self.focus_seen = true;
            let commit = |edit: &TextEdit, value: &mut f32| match edit.buffer.trim().parse::<f32>()
            {
                Ok(v) if v.is_finite() && *value != v => {
                    *value = v;
                    true
                }
                _ => false,
            };
            if self.input.pressed[0] && !r.hovered {
                let edit = self.text_edit.take().unwrap();
                r.changed = commit(&edit, value);
                self.focus = None;
            } else if let Some(edit) = self.text_edit.as_mut() {
                let mut buffer = std::mem::take(&mut edit.buffer);
                let (_, done) = apply_typing(edit, &mut buffer, &self.input.text, &self.input.keys);
                edit.buffer = buffer;
                match done {
                    Some(true) => {
                        let edit = self.text_edit.take().unwrap();
                        r.changed = commit(&edit, value);
                        r.submitted = true;
                        self.focus = None;
                    }
                    Some(false) => self.end_edit(),
                    None => {}
                }
            }
        } else {
            if r.pressed {
                self.drag_travel = 0.0;
                self.drag_acc = 0.0;
            }
            if r.dragging && r.drag_delta.0 != 0.0 {
                self.drag_travel += r.drag_delta.0.abs();
                let quantum = step * if self.input.shift { 0.1 } else { 1.0 };
                self.drag_acc += r.drag_delta.0 * quantum / 4.0; // 4 px per step
                let steps = (self.drag_acc / quantum).trunc();
                if steps != 0.0 {
                    self.drag_acc -= steps * quantum;
                    *value = ((*value + steps * quantum) / quantum).round() * quantum;
                    r.changed = true;
                }
            }
            if r.clicked && self.drag_travel < 3.0 {
                let text = format!("{:.*}", decimals, value);
                let len = text.chars().count();
                self.begin_edit(id, text.clone(), len);
                if let Some(edit) = self.text_edit.as_mut() {
                    edit.buffer = text;
                }
            }
        }

        let editing = self.focus == Some(id);
        self.paint_field(rect, r.hovered || r.dragging, editing);
        let size = self.theme.text_size;
        let mut x = rect.x + 8.0;
        if let Some((name, color)) = tag {
            let tag_rect = Rect::new(rect.x + 3.0, rect.y + 3.0, 18.0, rect.h - 6.0);
            self.paint_rounded(
                tag_rect,
                3.0,
                color.with_alpha(0.2),
                0.0,
                Color::TRANSPARENT,
            );
            let tw = font::text_width(name, size);
            self.paint_text_centered_y(
                tag_rect.x + (tag_rect.w - tw) / 2.0,
                rect,
                size,
                color,
                name,
            );
            x = tag_rect.right() + 8.0;
        }
        let text_color = self.theme.text;
        if editing {
            let buffer = self
                .text_edit
                .as_ref()
                .map(|e| e.buffer.clone())
                .unwrap_or_default();
            self.push_clip(rect.shrink(2.0));
            self.paint_text_centered_y(x, rect, size, text_color, &buffer);
            self.paint_caret(id, x, rect, &buffer);
            self.pop_clip();
        } else {
            let text = format!("{:.*}", decimals, value);
            self.paint_text_centered_y(x, rect, size, text_color, &text);
        }
        r
    }

    fn begin_edit(&mut self, id: Id, original: String, cursor: usize) {
        self.focus = Some(id);
        self.focus_seen = true;
        self.text_edit = Some(TextEdit {
            id,
            cursor,
            buffer: String::new(),
            original,
        });
    }

    fn end_edit(&mut self) {
        self.focus = None;
        self.text_edit = None;
    }

    pub(super) fn paint_field(&mut self, rect: Rect, hovered: bool, focused: bool) {
        let t = &self.theme;
        let fill = if focused {
            t.panel_header
        } else if hovered {
            t.surface_hover
        } else {
            t.surface
        };
        let (border, width) = if focused {
            (t.accent, 1.5)
        } else {
            (t.border, 1.0)
        };
        let radius = t.radius_small;
        self.paint_rounded(rect, radius, fill, width, border);
    }

    fn paint_caret(&mut self, id: Id, x: f32, rect: Rect, text: &str) {
        let Some(edit) = self.text_edit.as_ref().filter(|e| e.id == id) else {
            return;
        };
        if (self.time * 2.0).fract() > 0.6 {
            return; // blink
        }
        let size = self.theme.text_size;
        let before: String = text.chars().take(edit.cursor).collect();
        let cx = (x + font::text_width(&before, size) + if before.is_empty() { 0.0 } else { 2.0 })
            .round();
        let color = self.theme.accent_hover;
        self.paint(DrawCommand::Rect {
            rect: Rect::new(cx, rect.y + 6.0, 2.0, rect.h - 12.0),
            color,
        });
    }

    // --------------------------------------------------------------- lists

    /// A full-width list row: optional disclosure chevron (`open`), icon,
    /// text, optional trailing badge. `depth` indents it (trees). Clicking
    /// the chevron flips `open` (and doesn't select the row).
    #[allow(clippy::too_many_arguments)]
    pub fn list_row(
        &mut self,
        key: impl std::hash::Hash,
        depth: usize,
        open: Option<&mut bool>,
        icon: Option<Icon>,
        text: &str,
        selected: bool,
        badge: Option<(&str, Color)>,
    ) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("row", key));
        let indent = self.theme.indent * depth as f32;
        let chevron = Rect::new(rect.x + indent, rect.y, 18.0, rect.h);
        // Input first (the chevron before the row, so it wins the press),
        // then paint back to front.
        let mut disclosure = None;
        if let Some(open) = open {
            let cr = self.interact(id.wrapping_add(1), chevron);
            if cr.clicked {
                *open = !*open;
            }
            disclosure = Some((*open, cr.hovered, cr.clicked));
        }
        let mut r = self.interact(id, rect);
        if disclosure.is_some_and(|(_, _, toggled)| toggled) {
            r.clicked = false;
        }
        let t = &self.theme;
        if selected || r.hovered {
            let fill = if selected {
                t.selection
            } else {
                t.surface_hover.with_alpha(0.6)
            };
            let radius = t.radius_small;
            self.paint_rounded(rect, radius, fill, 0.0, Color::TRANSPARENT);
        }
        if let Some((is_open, hovered, _)) = disclosure {
            let color = if hovered {
                self.theme.text
            } else {
                self.theme.text_dim
            };
            let icon = if is_open {
                Icon::ChevronDown
            } else {
                Icon::ChevronRight
            };
            let icon_rect = Rect::new(chevron.x, rect.y + (rect.h - 18.0) / 2.0, 18.0, 18.0);
            self.paint(DrawCommand::Icon {
                rect: icon_rect,
                icon,
                color,
            });
        }
        let t = &self.theme;
        let fg = if selected {
            t.text
        } else {
            t.text_dim.lerp(t.text, 0.5)
        };
        let icon_color = if selected { t.accent_hover } else { t.text_dim };
        let mut x = chevron.right() + 2.0;
        if let Some(icon) = icon {
            let s = self.icon_size();
            self.paint_icon(Rect::new(x, rect.y, s, rect.h), icon, icon_color);
            x += s + 8.0;
        }
        let size = self.theme.text_size;
        let mut text_clip = rect;
        if let Some((badge, color)) = badge {
            let small = self.theme.small_text_size;
            let bw = font::text_width(badge, small) + 12.0;
            let pill = Rect::new(
                rect.right() - bw - 6.0,
                rect.y + (rect.h - 16.0) / 2.0,
                bw,
                16.0,
            );
            self.paint_rounded(pill, 8.0, color.with_alpha(0.16), 0.0, Color::TRANSPARENT);
            self.paint_text_centered_y(pill.x + 6.0, pill, small, color, badge);
            text_clip.w = pill.x - rect.x - 4.0;
        }
        self.push_clip(text_clip);
        self.paint_text_centered_y(x, rect, size, fg, text);
        self.pop_clip();
        r
    }
}

/// Applies a frame of typing to `value`. Returns whether it changed, and
/// `Some(true)` on Enter, `Some(false)` on Escape.
fn apply_typing(
    edit: &mut TextEdit,
    value: &mut String,
    typed: &str,
    keys: &[UiKey],
) -> (bool, Option<bool>) {
    let mut chars: Vec<char> = value.chars().collect();
    let mut changed = false;
    edit.cursor = edit.cursor.min(chars.len());
    for c in typed.chars() {
        chars.insert(edit.cursor, c);
        edit.cursor += 1;
        changed = true;
    }
    let mut done = None;
    for key in keys {
        match key {
            UiKey::Left => edit.cursor = edit.cursor.saturating_sub(1),
            UiKey::Right => edit.cursor = (edit.cursor + 1).min(chars.len()),
            UiKey::Home => edit.cursor = 0,
            UiKey::End => edit.cursor = chars.len(),
            UiKey::Backspace if edit.cursor > 0 => {
                edit.cursor -= 1;
                chars.remove(edit.cursor);
                changed = true;
            }
            UiKey::Delete if edit.cursor < chars.len() => {
                chars.remove(edit.cursor);
                changed = true;
            }
            UiKey::Enter => done = Some(true),
            UiKey::Escape => done = Some(false),
            _ => {}
        }
    }
    if changed {
        *value = chars.into_iter().collect();
    }
    (changed, done)
}
