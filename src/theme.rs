//! How the UI looks: every color and metric widgets use, in one place.
//!
//! A [`Theme`] is plain data. Pick a built-in one ([`Theme::dark`],
//! [`Theme::light`]), tweak any field, or build your own; assign it to
//! [`crate::Ui::theme`] at any time and the next frame uses it.
//!
//! ```
//! use pulga_gui_lib::{Color, Theme};
//!
//! let mut theme = Theme::dark();
//! theme.accent = Color::hex(0x00babc); // 42 teal
//! theme.radius = 2.0;                  // sharper corners
//! ```

use crate::geometry::Color;

/// See the [module docs](self).
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// Window background behind panels (e.g. a 3D viewport's clear color).
    pub background: Color,
    /// Panel fill.
    pub panel: Color,
    /// Panel title bars, toolbars, status bars.
    pub panel_header: Color,
    /// Interactive surfaces at rest: buttons, fields.
    pub surface: Color,
    /// Interactive surfaces under the mouse.
    pub surface_hover: Color,
    /// Interactive surfaces being pressed.
    pub surface_active: Color,
    /// Outlines of panels, fields and popups.
    pub border: Color,
    /// Thin divider lines.
    pub separator: Color,
    /// Main text.
    pub text: Color,
    /// Secondary text: labels, captions.
    pub text_dim: Color,
    /// Hints and disabled text.
    pub text_faint: Color,
    /// Primary action / focus color.
    pub accent: Color,
    /// Accent under the mouse.
    pub accent_hover: Color,
    /// Text and icons drawn on the accent color.
    pub on_accent: Color,
    /// Background of selected rows.
    pub selection: Color,
    /// Destructive actions, errors.
    pub danger: Color,
    /// Destructive actions under the mouse.
    pub danger_hover: Color,
    /// Success notifications.
    pub success: Color,
    /// Warnings.
    pub warning: Color,
    /// Tags of X, Y and Z value fields.
    pub axis: [Color; 3],
    /// Drop shadows of popups, modals and notifications.
    pub shadow: Color,
    /// Dims everything behind a modal dialog.
    pub backdrop: Color,
    /// Line height of normal text, in pixels (multiples of 10 stay crisp).
    pub text_size: f32,
    /// Line height of captions and hints.
    pub small_text_size: f32,
    /// Height of buttons, fields and list rows.
    pub row_height: f32,
    /// Space between a container's edge and its content.
    pub padding: f32,
    /// Space between consecutive widgets.
    pub spacing: f32,
    /// Indentation per tree level.
    pub indent: f32,
    /// Corner radius of panels, popups and cards.
    pub radius: f32,
    /// Corner radius of buttons and fields.
    pub radius_small: f32,
    /// Icons are 9 pixel-art pixels; this is screen pixels per icon pixel.
    pub icon_scale: f32,
}

impl Default for Theme {
    /// [`Theme::dark`].
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Neutral dark grays with a blue accent: the default.
    pub fn dark() -> Self {
        Self {
            background: Color::hex(0x16181d),
            panel: Color::hex(0x1f2228),
            panel_header: Color::hex(0x1a1d22),
            surface: Color::hex(0x2a2e36),
            surface_hover: Color::hex(0x333842),
            surface_active: Color::hex(0x3c4250),
            border: Color::hex(0x323741),
            separator: Color::hex(0x2b2f37),
            text: Color::hex(0xe4e7ec),
            text_dim: Color::hex(0x9ba3af),
            text_faint: Color::hex(0x5f6774),
            accent: Color::hex(0x3b82f6),
            accent_hover: Color::hex(0x5a96f8),
            on_accent: Color::hex(0xffffff),
            selection: Color::hex(0x3b82f6).with_alpha(0.22),
            danger: Color::hex(0xdc4c4c),
            danger_hover: Color::hex(0xe56767),
            success: Color::hex(0x3fb46c),
            warning: Color::hex(0xe8a33d),
            axis: [
                Color::hex(0xe05555),
                Color::hex(0x5cbf5c),
                Color::hex(0x4f8ff0),
            ],
            shadow: Color::rgba(0.0, 0.0, 0.0, 0.45),
            backdrop: Color::rgba(0.0, 0.0, 0.0, 0.5),
            text_size: 20.0,
            small_text_size: 10.0,
            row_height: 30.0,
            padding: 10.0,
            spacing: 6.0,
            indent: 16.0,
            radius: 8.0,
            radius_small: 5.0,
            icon_scale: 2.0,
        }
    }

    /// Light grays with the same accent, for bright rooms.
    pub fn light() -> Self {
        Self {
            background: Color::hex(0xdfe2e7),
            panel: Color::hex(0xf6f7f9),
            panel_header: Color::hex(0xeceef2),
            surface: Color::hex(0xe4e7ec),
            surface_hover: Color::hex(0xd8dce3),
            surface_active: Color::hex(0xc9ced8),
            border: Color::hex(0xd0d4dc),
            separator: Color::hex(0xdfe2e8),
            text: Color::hex(0x1d2129),
            text_dim: Color::hex(0x555d6b),
            text_faint: Color::hex(0x98a0ad),
            accent: Color::hex(0x2563eb),
            accent_hover: Color::hex(0x3b78f0),
            on_accent: Color::hex(0xffffff),
            selection: Color::hex(0x2563eb).with_alpha(0.16),
            danger: Color::hex(0xc93636),
            danger_hover: Color::hex(0xd65050),
            success: Color::hex(0x2f9a59),
            warning: Color::hex(0xc4821c),
            axis: [
                Color::hex(0xc93c3c),
                Color::hex(0x3c9a3c),
                Color::hex(0x2f6fd6),
            ],
            shadow: Color::rgba(0.1, 0.12, 0.18, 0.22),
            backdrop: Color::rgba(0.1, 0.12, 0.18, 0.35),
            ..Self::dark()
        }
    }
}
