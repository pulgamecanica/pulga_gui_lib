//! Rectangles and colors.

/// An axis-aligned rectangle in pixels: top-left corner plus size, y down.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    /// Left edge.
    pub x: f32,
    /// Top edge.
    pub y: f32,
    /// Width.
    pub w: f32,
    /// Height.
    pub h: f32,
}

impl Rect {
    /// A rectangle from its top-left corner and size.
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Whether the point is inside. Half-open: the right and bottom edges
    /// belong to the next rectangle over.
    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    /// Right edge.
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Bottom edge.
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    /// The center point.
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    /// Shrunk by `d` on every side (grown if negative). Never inverts.
    pub fn shrink(&self, d: f32) -> Self {
        Self::new(
            self.x + d,
            self.y + d,
            (self.w - 2.0 * d).max(0.0),
            (self.h - 2.0 * d).max(0.0),
        )
    }

    /// The overlap of two rectangles (zero-sized if they don't overlap).
    pub fn intersect(&self, other: &Rect) -> Self {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        Self::new(
            x,
            y,
            (self.right().min(other.right()) - x).max(0.0),
            (self.bottom().min(other.bottom()) - y).max(0.0),
        )
    }

    /// `[x, y, w, h]`, the layout the GL canvas takes.
    pub fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

/// A straight (non-premultiplied) RGBA color, each channel 0..1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red.
    pub r: f32,
    /// Green.
    pub g: f32,
    /// Blue.
    pub b: f32,
    /// Alpha (opacity).
    pub a: f32,
}

impl Color {
    /// Fully transparent.
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

    /// An opaque color.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// A color with explicit alpha.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// An opaque color from `0xRRGGBB`.
    pub const fn hex(rgb: u32) -> Self {
        Self::rgb(
            ((rgb >> 16) & 0xff) as f32 / 255.0,
            ((rgb >> 8) & 0xff) as f32 / 255.0,
            (rgb & 0xff) as f32 / 255.0,
        )
    }

    /// The same color with alpha replaced.
    pub const fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    /// Linear blend: `t = 0` is `self`, `t = 1` is `other`.
    pub fn lerp(self, other: Color, t: f32) -> Self {
        let m = |a: f32, b: f32| a + (b - a) * t;
        Self::rgba(
            m(self.r, other.r),
            m(self.g, other.g),
            m(self.b, other.b),
            m(self.a, other.a),
        )
    }

    /// `[r, g, b, a]`, the layout the GL canvas takes.
    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn contains_is_half_open() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(0.0, 0.0));
        assert!(r.contains(9.9, 9.9));
        assert!(!r.contains(10.0, 10.0));
        assert!(!r.contains(-0.1, 5.0));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn shrink_and_intersect() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert_eq!(r.shrink(2.0), Rect::new(2.0, 2.0, 6.0, 6.0));
        assert_eq!(r.shrink(20.0).w, 0.0);
        let o = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert_eq!(r.intersect(&o), Rect::new(5.0, 5.0, 5.0, 5.0));
        assert_eq!(r.intersect(&Rect::new(20.0, 0.0, 5.0, 5.0)).w, 0.0);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn hex_and_lerp() {
        assert_eq!(Color::hex(0xff0000), Color::rgb(1.0, 0.0, 0.0));
        let mid = Color::rgb(0.0, 0.0, 0.0).lerp(Color::rgb(1.0, 1.0, 1.0), 0.5);
        assert_eq!(mid, Color::rgb(0.5, 0.5, 0.5));
    }
}
