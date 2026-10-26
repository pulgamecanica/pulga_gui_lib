//! Containers and overlays: panels, scroll areas, popups (dropdowns and
//! menus), modal dialogs and notifications.

use super::{Dir, Frame, Layer, Response, TOAST_LIFETIME, Toast, Ui};
use crate::draw::DrawCommand;
use crate::font;
use crate::geometry::{Color, Rect};
use crate::icons::Icon;
use crate::input::UiKey;

/// The tone of a [`Ui::notify`] notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    /// Neutral information.
    Info,
    /// Something worked (saved, exported).
    Success,
    /// Something failed.
    Error,
}

impl Ui {
    /// A panel filling `rect`: background, outline, clipping, and a
    /// padded top-to-bottom layout for `f`'s widgets. Mouse input over it
    /// belongs to the UI (see [`Ui::wants_pointer`]).
    pub fn panel<R>(&mut self, key: &str, rect: Rect, f: impl FnOnce(&mut Ui) -> R) -> R {
        let (fill, border) = (self.theme.panel, self.theme.border);
        self.paint(DrawCommand::Rect { rect, color: fill });
        self.paint_outline(rect, border);
        self.cover(rect);
        let pad = self.theme.padding;
        self.push_clip(rect);
        let r = self.push_id(key, |ui| ui.put(rect.shrink(pad), f));
        self.pop_clip();
        r
    }

    /// A strip (toolbar, status bar) filling `rect`, laying `f`'s widgets
    /// out left to right, vertically centered.
    pub fn bar<R>(&mut self, key: &str, rect: Rect, f: impl FnOnce(&mut Ui) -> R) -> R {
        let (fill, border) = (self.theme.panel_header, self.theme.border);
        self.paint(DrawCommand::Rect { rect, color: fill });
        self.paint_outline(rect, border);
        self.cover(rect);
        let pad_y = ((rect.h - self.theme.row_height) / 2.0).max(0.0);
        let inner = Rect::new(
            rect.x + self.theme.padding,
            rect.y + pad_y,
            rect.w - 2.0 * self.theme.padding,
            rect.h - 2.0 * pad_y,
        );
        self.push_clip(rect);
        let r = self.push_id(key, |ui| ui.put_horizontal(inner, f));
        self.pop_clip();
        r
    }

    fn paint_outline(&mut self, rect: Rect, color: Color) {
        self.paint(DrawCommand::RoundedRect {
            rect,
            radius: 0.0,
            fill: Color::TRANSPARENT,
            border: 1.0,
            border_color: color,
        });
    }

    /// A vertically scrolling region `height` pixels tall
    /// (`f32::INFINITY`: the rest of the container). The wheel scrolls it
    /// while the mouse is over it; a thumb shows when content overflows.
    pub fn scroll_area<R>(&mut self, key: &str, height: f32, f: impl FnOnce(&mut Ui) -> R) -> R {
        let avail = self.available();
        let h = if height.is_infinite() {
            avail.h
        } else {
            height.min(avail.h.max(0.0))
        };
        let rect = self.allocate(f32::INFINITY, h);
        let id = self.id(("scroll", key));
        let mut offset = self.scroll.get(&id).copied().unwrap_or(0.0);

        self.push_clip(rect);
        let gutter = 10.0;
        let region = Rect::new(
            rect.x,
            rect.y - offset,
            (rect.w - gutter).max(0.0),
            f32::INFINITY,
        );
        let (r, extent) =
            self.with_frame(Frame::new(region, Dir::Vertical), |ui| ui.push_id(key, f));
        self.pop_clip();

        let content = extent.1 - region.y;
        let max = (content - rect.h).max(0.0);
        if self.pointer_reaches(rect) && self.input.wheel != 0.0 {
            offset -= self.input.wheel * self.theme.row_height * 1.5;
            self.input.wheel = 0.0; // consumed
        }
        if max > 0.0 {
            let thumb_h = (rect.h * rect.h / content).max(24.0);
            let thumb_y = rect.y + (offset / max).clamp(0.0, 1.0) * (rect.h - thumb_h);
            let thumb = Rect::new(rect.right() - 6.0, thumb_y, 4.0, thumb_h);
            let tr = self.interact(id.wrapping_add(1), thumb.shrink(-4.0));
            if tr.dragging {
                offset += tr.drag_delta.1 * content / rect.h;
            }
            let color = if tr.hovered || tr.dragging {
                self.theme.text_faint
            } else {
                self.theme.surface_active
            };
            self.paint(DrawCommand::RoundedRect {
                rect: thumb,
                radius: 2.0,
                fill: color,
                border: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
        self.scroll.insert(id, offset.clamp(0.0, max));
        r
    }

    /// Whether `id`'s disclosure state is open (`default` until toggled).
    pub fn is_open(&mut self, id: super::Id, default: bool) -> bool {
        *self.open.entry(id).or_insert(default)
    }

    /// Sets `id`'s disclosure state.
    pub fn set_open(&mut self, id: super::Id, open: bool) {
        self.open.insert(id, open);
    }

    // ------------------------------------------------------------- popups

    /// Runs `f` inside a popup anchored under `anchor` (in the popup
    /// layer). The popup closes on Escape or a click outside it and its
    /// anchor.
    fn popup(&mut self, id: super::Id, anchor: Rect, width: f32, f: impl FnOnce(&mut Ui)) {
        self.popup_seen = true;
        let prev = std::mem::replace(&mut self.layer, Layer::Popup);
        let pad = 4.0;
        let guess_h = self.sizes.get(&id).map_or(120.0, |s| s.1);
        let mut x = anchor.x.min(self.screen.right() - width - 4.0);
        x = x.max(4.0);
        let below = anchor.bottom() + 4.0;
        let y = if below + guess_h > self.screen.bottom() {
            (anchor.y - 4.0 - guess_h).max(4.0)
        } else {
            below
        };

        self.paint(DrawCommand::Clip(None));
        let bg_at = self.layers[Layer::Popup as usize].len();
        let saved_clips = std::mem::replace(&mut self.clips, vec![self.screen]);
        let region = Rect::new(x + pad, y + pad, width - 2.0 * pad, f32::INFINITY);
        let mut frame = Frame::new(region, Dir::Vertical);
        frame.extent = (region.x, region.y);
        self.frames.push(frame);
        let spacing = std::mem::replace(&mut self.theme.spacing, 2.0);
        f(self);
        self.theme.spacing = spacing;
        let extent = self.frames.pop().unwrap().extent;
        self.clips = saved_clips;

        let rect = Rect::new(x, y, width, extent.1 - y + pad);
        self.sizes.insert(id, (rect.w, rect.h));
        self.layers[Layer::Popup as usize].splice(
            bg_at..bg_at,
            [
                DrawCommand::Shadow {
                    rect,
                    radius: self.theme.radius,
                    blur: 14.0,
                    color: self.theme.shadow,
                },
                DrawCommand::RoundedRect {
                    rect,
                    radius: self.theme.radius_small + 1.0,
                    fill: self.theme.panel,
                    border: 1.0,
                    border_color: self.theme.border,
                },
            ],
        );
        self.cover(rect);
        let (mx, my) = self.input.mouse;
        let clicked_away =
            self.input.pressed[0] && !rect.contains(mx, my) && !anchor.contains(mx, my);
        if clicked_away || self.input.keys.contains(&UiKey::Escape) {
            self.open_popup = None;
        }
        self.layer = prev;
    }

    /// A field showing `options[*selected]`; clicking opens a list to pick
    /// another. `changed` is set when the selection changes.
    pub fn dropdown(&mut self, key: &str, selected: &mut usize, options: &[&str]) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("dropdown", key));
        let mut r = self.interact(id, rect);
        if r.clicked {
            self.open_popup = if self.open_popup == Some(id) {
                None
            } else {
                Some(id)
            };
        }
        let open = self.open_popup == Some(id);
        self.paint_field(rect, r.hovered, open);
        let size = self.theme.text_size;
        let text = options.get(*selected).copied().unwrap_or("");
        let color = self.theme.text;
        self.paint_text_centered_y(rect.x + self.theme.padding, rect, size, color, text);
        let s = self.icon_size();
        let dim = self.theme.text_dim;
        self.paint_icon(
            Rect::new(rect.right() - s - 8.0, rect.y, s, rect.h),
            Icon::ChevronDown,
            dim,
        );

        if open {
            let mut picked = None;
            self.popup(id, rect, rect.w, |ui| {
                for (i, option) in options.iter().enumerate() {
                    if ui.menu_row(i, None, option, i == *selected).clicked {
                        picked = Some(i);
                    }
                }
            });
            if let Some(i) = picked {
                r.changed = *selected != i;
                *selected = i;
                self.open_popup = None;
            }
        }
        r
    }

    /// A button that opens a menu built by `f` (with [`Ui::menu_item`]).
    pub fn menu_button(
        &mut self,
        icon: Option<Icon>,
        text: &str,
        f: impl FnOnce(&mut Ui),
    ) -> Response {
        let r = self.button_with(super::ButtonKind::Ghost, icon, text);
        let id = r.id;
        if r.clicked {
            self.open_popup = if self.open_popup == Some(id) {
                None
            } else {
                Some(id)
            };
        }
        if self.open_popup == Some(id) {
            let width = 240.0f32.max(r.rect.w);
            self.popup(id, r.rect, width, f);
        }
        r
    }

    /// An entry of the open menu; returns `true` (and closes the menu) when
    /// clicked.
    pub fn menu_item(&mut self, icon: Option<Icon>, text: &str) -> bool {
        let clicked = self.menu_row(text, icon, text, false).clicked;
        if clicked {
            self.open_popup = None;
        }
        clicked
    }

    fn menu_row(
        &mut self,
        key: impl std::hash::Hash,
        icon: Option<Icon>,
        text: &str,
        selected: bool,
    ) -> Response {
        let rect = self.allocate(f32::INFINITY, self.theme.row_height);
        let id = self.id(("menu", key));
        let r = self.interact(id, rect);
        let t = &self.theme;
        if r.hovered || selected {
            let fill = if r.hovered { t.accent } else { t.selection };
            let radius = t.radius_small - 1.0;
            self.paint_rounded(rect, radius, fill, 0.0, Color::TRANSPARENT);
        }
        let fg = if r.hovered {
            self.theme.on_accent
        } else {
            self.theme.text
        };
        let mut x = rect.x + 8.0;
        if let Some(icon) = icon {
            let s = self.icon_size();
            self.paint_icon(Rect::new(x, rect.y, s, rect.h), icon, fg);
            x += s + 10.0;
        }
        let size = self.theme.text_size;
        self.paint_text_centered_y(x, rect, size, fg, text);
        r
    }

    // -------------------------------------------------------------- modals

    /// A modal dialog: dims and blocks everything else while it's being
    /// drawn, centered card `width` pixels wide titled `title`, content
    /// from `f`. Draw it every frame it should stay open; close it by not
    /// calling it any more.
    pub fn modal<R>(&mut self, title: &str, width: f32, f: impl FnOnce(&mut Ui) -> R) -> R {
        self.modal_open = true;
        let id = self.id(("modal", title));
        let prev = std::mem::replace(&mut self.layer, Layer::Modal);
        let screen = self.screen;
        self.paint(DrawCommand::Clip(None));
        let backdrop = self.theme.backdrop;
        self.paint(DrawCommand::Rect {
            rect: screen,
            color: backdrop,
        });
        self.cover(screen);

        let pad = self.theme.padding * 1.8;
        let h = self.sizes.get(&id).map_or(160.0, |s| s.1);
        let card = Rect::new(
            (screen.x + (screen.w - width) / 2.0).round(),
            (screen.y + (screen.h - h) / 2.0).round(),
            width,
            h,
        );
        let bg_at = self.layers[Layer::Modal as usize].len();
        let saved_clips = std::mem::replace(&mut self.clips, vec![screen]);
        let size = self.theme.text_size;
        let title_color = self.theme.text;
        self.paint_text(card.x + pad, card.y + pad, size, title_color, title);
        let region = Rect::new(
            card.x + pad,
            card.y + pad + size + 14.0,
            width - 2.0 * pad,
            f32::INFINITY,
        );
        let (r, extent) =
            self.with_frame(Frame::new(region, Dir::Vertical), |ui| ui.push_id(id, f));
        self.clips = saved_clips;

        let rect = Rect::new(card.x, card.y, width, extent.1 - card.y + pad);
        self.sizes.insert(id, (rect.w, rect.h));
        self.layers[Layer::Modal as usize].splice(
            bg_at..bg_at,
            [
                DrawCommand::Shadow {
                    rect,
                    radius: self.theme.radius,
                    blur: 28.0,
                    color: self.theme.shadow,
                },
                DrawCommand::RoundedRect {
                    rect,
                    radius: self.theme.radius,
                    fill: self.theme.panel,
                    border: 1.0,
                    border_color: self.theme.border,
                },
            ],
        );
        self.layer = prev;
        r
    }

    // ------------------------------------------------------- notifications

    /// Shows `text` in a notification at the bottom-right for a few
    /// seconds.
    pub fn notify(&mut self, text: impl Into<String>, kind: ToastKind) {
        self.toasts.push(Toast {
            text: text.into(),
            kind,
            age: 0.0,
        });
        if self.toasts.len() > 5 {
            self.toasts.remove(0);
        }
    }

    pub(super) fn draw_toasts(&mut self) {
        let dt = self.input.dt;
        self.toasts.retain_mut(|t| {
            t.age += dt;
            t.age < TOAST_LIFETIME
        });
        let prev = std::mem::replace(&mut self.layer, Layer::Tooltip);
        self.paint(DrawCommand::Clip(None));
        let size = self.theme.text_size;
        let pad = self.theme.padding;
        let mut bottom = self.screen.bottom() - 48.0;
        let toasts: Vec<(String, ToastKind, f32)> = self
            .toasts
            .iter()
            .rev()
            .map(|t| (t.text.clone(), t.kind, t.age))
            .collect();
        for (text, kind, age) in toasts {
            let fade =
                ((TOAST_LIFETIME - age) / 0.4).clamp(0.0, 1.0) * (age / 0.15).clamp(0.0, 1.0);
            let w = font::text_width(&text, size) + 2.0 * pad + 14.0;
            let h = self.theme.row_height + 10.0;
            let rect = Rect::new(self.screen.right() - w - 16.0, bottom - h, w, h);
            let accent = match kind {
                ToastKind::Info => self.theme.accent,
                ToastKind::Success => self.theme.success,
                ToastKind::Error => self.theme.danger,
            };
            let t = &self.theme;
            let cmds = [
                DrawCommand::Shadow {
                    rect,
                    radius: t.radius,
                    blur: 16.0,
                    color: t.shadow.with_alpha(t.shadow.a * fade),
                },
                DrawCommand::RoundedRect {
                    rect,
                    radius: t.radius,
                    fill: t.surface.with_alpha(fade),
                    border: 1.0,
                    border_color: t.border.with_alpha(fade),
                },
                DrawCommand::RoundedRect {
                    rect: Rect::new(rect.x + 6.0, rect.y + 8.0, 4.0, rect.h - 16.0),
                    radius: 2.0,
                    fill: accent.with_alpha(fade),
                    border: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            ];
            for c in cmds {
                self.paint(c);
            }
            let color = self.theme.text.with_alpha(fade);
            self.paint_text_centered_y(rect.x + pad + 10.0, rect, size, color, &text);
            bottom = rect.y - 8.0;
        }
        self.layer = prev;
    }
}
