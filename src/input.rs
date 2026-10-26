//! What the UI needs to know about the user each frame, independent of any
//! windowing library: the host (e.g. `pulga_engine`) fills an [`Input`] from
//! its own events.

/// Keys the UI reacts to (text editing, menus, dialogs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiKey {
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home.
    Home,
    /// End.
    End,
    /// Backspace.
    Backspace,
    /// Delete.
    Delete,
    /// Enter.
    Enter,
    /// Escape.
    Escape,
    /// Tab.
    Tab,
}

/// Mouse buttons, as indices into [`Input`]'s button arrays.
pub mod button {
    /// Primary button.
    pub const LEFT: usize = 0;
    /// Secondary button.
    pub const RIGHT: usize = 1;
    /// Wheel button.
    pub const MIDDLE: usize = 2;
}

/// One frame of input. State fields (`mouse`, `down`, modifiers) persist;
/// event fields (`pressed`, `released`, `wheel`, `text`, `keys`) describe
/// only what happened since the last frame; reset them with
/// [`Input::end_frame`] after each UI frame.
#[derive(Debug, Clone, Default)]
pub struct Input {
    /// Cursor position, pixels from the window's top-left.
    pub mouse: (f32, f32),
    /// Buttons held down (see [`button`]).
    pub down: [bool; 3],
    /// Buttons that went down this frame.
    pub pressed: [bool; 3],
    /// Buttons that went up this frame.
    pub released: [bool; 3],
    /// Wheel lines scrolled this frame, positive away from the user.
    pub wheel: f32,
    /// Characters typed this frame.
    pub text: String,
    /// Keys pressed this frame (including key repeats).
    pub keys: Vec<UiKey>,
    /// Shift held.
    pub shift: bool,
    /// Control held.
    pub ctrl: bool,
    /// Seconds since the last frame.
    pub dt: f32,
}

impl Input {
    /// Clears the per-frame event fields, keeping held state.
    pub fn end_frame(&mut self) {
        self.pressed = [false; 3];
        self.released = [false; 3];
        self.wheel = 0.0;
        self.text.clear();
        self.keys.clear();
    }
}
