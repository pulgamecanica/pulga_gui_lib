//! The built-in bitmap font, drawn by hand as ASCII art, and the texture
//! atlas it shares with the [`crate::icons`].
//!
//! Defining glyphs in code keeps text rendering free of font files, font
//! libraries and image decoders (the subject only allows single-format image
//! libraries). Glyphs are up to 5 pixels wide and [`GLYPH_H`] tall: 7 rows
//! above the baseline (capitals, digits, ascenders) and 2 below it
//! (descenders of `g j p q y`). A line is [`CELL_H`] pixels: one more row of
//! line spacing.
//!
//! The font is proportional: each glyph advances by its own inked width plus
//! one column (digits keep a fixed width so numbers line up in columns).
//! Text is drawn at a line height `size` in pixels; font pixels scale by
//! `size / CELL_H`, so multiples of 10 stay crisp.
//!
//! Any unsupported character renders as `?`.

use crate::icons;

/// Glyph width in pixels (`#`/`.` per row).
pub const GLYPH_W: usize = 5;
/// Glyph height in pixels, descender rows included.
pub const GLYPH_H: usize = 9;
/// Rows above the baseline (capital height).
pub const CAP_H: usize = 7;
/// Line height: glyph plus one row of line spacing.
pub const CELL_H: usize = GLYPH_H + 1;
/// Width of a glyph's slot in the atlas: glyph plus one empty column, so
/// filtering never bleeds a neighbour in.
pub const SLOT_W: usize = GLYPH_W + 1;
/// Advance of a space, in font pixels.
const SPACE_ADVANCE: usize = 3;

/// Every supported character and its rows: `#` is on, `.` is off. Rows
/// past the 7th (descenders) may be left out.
#[rustfmt::skip]
pub const GLYPHS: &[(char, &[&str])] = &[
    ('0', &[".###.", "#...#", "#..##", "#.#.#", "##..#", "#...#", ".###."]),
    ('1', &["..#..", ".##..", "..#..", "..#..", "..#..", "..#..", ".###."]),
    ('2', &[".###.", "#...#", "....#", "...#.", "..#..", ".#...", "#####"]),
    ('3', &["#####", "...#.", "..#..", "...#.", "....#", "#...#", ".###."]),
    ('4', &["...#.", "..##.", ".#.#.", "#..#.", "#####", "...#.", "...#."]),
    ('5', &["#####", "#....", "####.", "....#", "....#", "#...#", ".###."]),
    ('6', &["..##.", ".#...", "#....", "####.", "#...#", "#...#", ".###."]),
    ('7', &["#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#..."]),
    ('8', &[".###.", "#...#", "#...#", ".###.", "#...#", "#...#", ".###."]),
    ('9', &[".###.", "#...#", "#...#", ".####", "....#", "...#.", ".##.."]),
    ('A', &[".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]),
    ('B', &["####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."]),
    ('C', &[".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."]),
    ('D', &["###..", "#..#.", "#...#", "#...#", "#...#", "#..#.", "###.."]),
    ('E', &["#####", "#....", "#....", "####.", "#....", "#....", "#####"]),
    ('F', &["#####", "#....", "#....", "####.", "#....", "#....", "#...."]),
    ('G', &[".###.", "#...#", "#....", "#.###", "#...#", "#...#", ".####"]),
    ('H', &["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]),
    ('I', &["###", ".#.", ".#.", ".#.", ".#.", ".#.", "###"]),
    ('J', &["..###", "...#.", "...#.", "...#.", "...#.", "#..#.", ".##.."]),
    ('K', &["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"]),
    ('L', &["#....", "#....", "#....", "#....", "#....", "#....", "#####"]),
    ('M', &["#...#", "##.##", "#.#.#", "#.#.#", "#...#", "#...#", "#...#"]),
    ('N', &["#...#", "#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#"]),
    ('O', &[".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]),
    ('P', &["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."]),
    ('Q', &[".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#"]),
    ('R', &["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"]),
    ('S', &[".####", "#....", "#....", ".###.", "....#", "....#", "####."]),
    ('T', &["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."]),
    ('U', &["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]),
    ('V', &["#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#.."]),
    ('W', &["#...#", "#...#", "#...#", "#.#.#", "#.#.#", "#.#.#", ".#.#."]),
    ('X', &["#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#"]),
    ('Y', &["#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#.."]),
    ('Z', &["#####", "....#", "...#.", "..#..", ".#...", "#....", "#####"]),
    ('a', &[".....", ".....", ".###.", "....#", ".####", "#...#", ".####"]),
    ('b', &["#....", "#....", "####.", "#...#", "#...#", "#...#", "####."]),
    ('c', &["....", "....", ".###", "#...", "#...", "#...", ".###"]),
    ('d', &["....#", "....#", ".####", "#...#", "#...#", "#...#", ".####"]),
    ('e', &[".....", ".....", ".###.", "#...#", "#####", "#....", ".###."]),
    ('f', &["..##", ".#..", ".#..", "###.", ".#..", ".#..", ".#.."]),
    ('g', &[".....", ".....", ".####", "#...#", "#...#", "#...#", ".####", "....#", ".###."]),
    ('h', &["#....", "#....", "####.", "#...#", "#...#", "#...#", "#...#"]),
    ('i', &["#", ".", "#", "#", "#", "#", "#"]),
    ('j', &["..#", "...", ".##", "..#", "..#", "..#", "..#", "#.#", ".#."]),
    ('k', &["#...", "#...", "#..#", "#.#.", "##..", "#.#.", "#..#"]),
    ('l', &["##.", ".#.", ".#.", ".#.", ".#.", ".#.", "..#"]),
    ('m', &[".....", ".....", "##.#.", "#.#.#", "#.#.#", "#.#.#", "#.#.#"]),
    ('n', &[".....", ".....", "####.", "#...#", "#...#", "#...#", "#...#"]),
    ('o', &[".....", ".....", ".###.", "#...#", "#...#", "#...#", ".###."]),
    ('p', &[".....", ".....", "####.", "#...#", "#...#", "#...#", "####.", "#....", "#...."]),
    ('q', &[".....", ".....", ".####", "#...#", "#...#", "#...#", ".####", "....#", "....#"]),
    ('r', &["....", "....", "#.##", "##..", "#...", "#...", "#..."]),
    ('s', &["....", "....", ".###", "#...", ".##.", "...#", "###."]),
    ('t', &[".#..", ".#..", "###.", ".#..", ".#..", ".#..", "..##"]),
    ('u', &[".....", ".....", "#...#", "#...#", "#...#", "#...#", ".####"]),
    ('v', &[".....", ".....", "#...#", "#...#", "#...#", ".#.#.", "..#.."]),
    ('w', &[".....", ".....", "#...#", "#...#", "#.#.#", "#.#.#", ".#.#."]),
    ('x', &[".....", ".....", "#...#", ".#.#.", "..#..", ".#.#.", "#...#"]),
    ('y', &[".....", ".....", "#...#", "#...#", "#...#", "#...#", ".####", "....#", ".###."]),
    ('z', &[".....", ".....", "#####", "...#.", "..#..", ".#...", "#####"]),
    ('.', &[".", ".", ".", ".", ".", ".", "#"]),
    (',', &["..", "..", "..", "..", "..", ".#", ".#", "#."]),
    (':', &[".", ".", "#", ".", ".", ".", "#"]),
    (';', &["..", "..", ".#", "..", "..", ".#", ".#", "#."]),
    ('!', &["#", "#", "#", "#", "#", ".", "#"]),
    ('?', &[".###.", "#...#", "....#", "...#.", "..#..", ".....", "..#.."]),
    ('-', &["....", "....", "....", "####", "....", "....", "...."]),
    ('+', &[".....", "..#..", "..#..", "#####", "..#..", "..#..", "....."]),
    ('=', &["....", "....", "####", "....", "####", "....", "...."]),
    ('/', &["....#", "....#", "...#.", "..#..", ".#...", "#....", "#...."]),
    ('\\', &["#....", "#....", ".#...", "..#..", "...#.", "....#", "....#"]),
    ('(', &["..#", ".#.", "#..", "#..", "#..", ".#.", "..#"]),
    (')', &["#..", ".#.", "..#", "..#", "..#", ".#.", "#.."]),
    ('[', &["###", "#..", "#..", "#..", "#..", "#..", "###"]),
    (']', &["###", "..#", "..#", "..#", "..#", "..#", "###"]),
    ('<', &["...#", "..#.", ".#..", "#...", ".#..", "..#.", "...#"]),
    ('>', &["#...", ".#..", "..#.", "...#", "..#.", ".#..", "#..."]),
    ('\'', &["#", "#", ".", ".", ".", ".", "."]),
    ('"', &["#.#", "#.#", "...", "...", "...", "...", "..."]),
    ('%', &["##...", "##..#", "...#.", "..#..", ".#...", "#..##", "...##"]),
    ('#', &[".#.#.", ".#.#.", "#####", ".#.#.", "#####", ".#.#.", ".#.#."]),
    ('*', &[".....", "#.#.#", ".###.", "#####", ".###.", "#.#.#", "....."]),
    ('_', &[".....", ".....", ".....", ".....", ".....", ".....", ".....", "#####"]),
    ('|', &["#", "#", "#", "#", "#", "#", "#", "#"]),
    ('&', &[".##..", "#..#.", "#.#..", ".#...", "#.#.#", "#..#.", ".##.#"]),
    ('@', &[".###.", "#...#", "#.###", "#.#.#", "#.###", "#....", ".###."]),
    ('·', &[".", ".", ".", "#", ".", ".", "."]),
];

/// Pixel `(x, y)` of glyph `index` (0-based from its left edge), off outside
/// the glyph's rows/columns.
fn glyph_pixel(index: usize, x: usize, y: usize) -> bool {
    GLYPHS[index]
        .1
        .get(y)
        .is_some_and(|row| row.as_bytes().get(x) == Some(&b'#'))
}

/// Index into [`GLYPHS`] used to draw `c`.
pub fn glyph_index(c: char) -> usize {
    GLYPHS
        .iter()
        .position(|&(g, _)| g == c)
        .or_else(|| GLYPHS.iter().position(|&(g, _)| g == '?'))
        .expect("'?' is in GLYPHS")
}

/// Width in font pixels of glyph `index`: its widest row (digits are padded
/// to the full width so numbers line up).
fn glyph_width(index: usize) -> usize {
    let (c, rows) = GLYPHS[index];
    if c.is_ascii_digit() {
        return GLYPH_W;
    }
    rows.iter().map(|r| r.len()).max().unwrap_or(0)
}

/// How far `c` moves the pen, in font pixels.
fn advance(c: char) -> usize {
    if c == ' ' {
        SPACE_ADVANCE
    } else {
        glyph_width(glyph_index(c)) + 1
    }
}

/// Atlas width: one slot per glyph, then the icons.
pub(crate) fn atlas_width() -> usize {
    GLYPHS.len() * SLOT_W + icons::atlas_width()
}

/// The font and icons as one RGBA image, [`atlas_width`] × [`CELL_H`],
/// rows from the top: opaque white where a pixel is on and transparent
/// elsewhere, so a draw color tints it. Glyph `i` starts at
/// `x = i * SLOT_W`; icons follow (see [`crate::icons`]).
pub fn atlas() -> (Vec<u8>, usize, usize) {
    let (w, h) = (atlas_width(), CELL_H);
    let mut pixels = vec![0u8; w * h * 4];
    let mut set = |x: usize, y: usize| {
        let at = (y * w + x) * 4;
        pixels[at..at + 4].copy_from_slice(&[255; 4]);
    };
    for i in 0..GLYPHS.len() {
        for y in 0..GLYPH_H {
            for x in 0..GLYPH_W {
                if glyph_pixel(i, x, y) {
                    set(i * SLOT_W + x, y);
                }
            }
        }
    }
    let icons_x = GLYPHS.len() * SLOT_W;
    icons::draw_into(icons_x, &mut set);
    (pixels, w, h)
}

/// Width in pixels of the longest line of `text` drawn `size` pixels per
/// line, for centering or right-aligning.
pub fn text_width(text: &str, size: f32) -> f32 {
    let px = size / CELL_H as f32;
    text.lines()
        .map(|line| {
            let pen: usize = line.chars().map(advance).sum();
            pen.saturating_sub(1) as f32 * px // no spacing after the last glyph
        })
        .fold(0.0, f32::max)
}

/// Where each visible glyph of `text` goes: `(dst, uv)` pairs, `dst` as
/// `[x, y, w, h]` pixels with the text's top-left at (`x`, `y`), `uv` as
/// `[u0, v0, u1, v1]` into [`atlas`]. `size` is the line height in pixels.
/// `\n` starts a new line; spaces produce nothing.
pub fn layout(x: f32, y: f32, size: f32, text: &str) -> Vec<([f32; 4], [f32; 4])> {
    let atlas_w = atlas_width() as f32;
    let px = size / CELL_H as f32; // screen pixels per font pixel
    let mut out = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        let mut pen = 0usize;
        for c in line.chars() {
            if c != ' ' {
                let i = glyph_index(c);
                let w = glyph_width(i);
                let u0 = (i * SLOT_W) as f32 / atlas_w;
                let dst = [
                    x + pen as f32 * px,
                    y + line_no as f32 * CELL_H as f32 * px,
                    w as f32 * px,
                    GLYPH_H as f32 * px,
                ];
                let uv = [
                    u0,
                    0.0,
                    u0 + w as f32 / atlas_w,
                    GLYPH_H as f32 / CELL_H as f32,
                ];
                out.push((dst, uv));
            }
            pen += advance(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn every_glyph_is_well_formed_and_unique() {
        for (i, (c, rows)) in GLYPHS.iter().enumerate() {
            assert!(rows.len() >= CAP_H && rows.len() <= GLYPH_H, "{c:?} rows");
            let width = rows[0].len();
            for row in *rows {
                assert!(row.len() == width && width <= GLYPH_W, "{c:?} row {row:?}");
                assert!(
                    row.bytes().all(|b| b == b'#' || b == b'.'),
                    "{c:?} row {row:?}"
                );
            }
            assert!(!GLYPHS[..i].iter().any(|(d, _)| d == c), "{c:?} twice");
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn ascii_letters_and_digits_are_all_covered() {
        let q = glyph_index('?');
        for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
            assert_ne!(glyph_index(c), q, "{c:?} missing");
        }
        assert_eq!(glyph_index('é'), q);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn atlas_places_glyph_pixels_in_their_slot() {
        let (pixels, w, h) = atlas();
        assert_eq!((w, h), (atlas_width(), CELL_H));
        let alpha = |x: usize, y: usize| pixels[(y * w + x) * 4 + 3];
        // '-' is a full middle row, nothing in the spacing column after it.
        let x0 = glyph_index('-') * SLOT_W;
        assert!((0..4).all(|x| alpha(x0 + x, 3) == 255));
        assert_eq!(alpha(x0 + GLYPH_W, 3), 0);
        // 'g' has a descender.
        assert!((0..GLYPH_W).any(|x| alpha(glyph_index('g') * SLOT_W + x, 8) == 255));
        // The line-spacing row stays empty.
        assert!((0..w).all(|x| alpha(x, CELL_H - 1) == 0));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn layout_is_proportional_and_skips_spaces() {
        // size 20 = 2 screen pixels per font pixel.
        let quads = layout(10.0, 20.0, 20.0, "il A\nB");
        let origins: Vec<(f32, f32)> = quads.iter().map(|(d, _)| (d[0], d[1])).collect();
        // 'i' is 1 wide (+1), 'l' 3 wide (+1), space 3: 'A' at pen 9.
        assert_eq!(
            origins,
            [(10.0, 20.0), (14.0, 20.0), (28.0, 20.0), (10.0, 40.0)]
        );
        assert_eq!(quads[2].0[2..], [10.0, 18.0]);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn text_width_uses_the_longest_line_without_trailing_spacing() {
        assert_eq!(text_width("AB\nA", 10.0), 11.0);
        assert_eq!(text_width("", 10.0), 0.0);
        // Digits are fixed-width: "11" is as wide as "88".
        assert_eq!(text_width("11", 10.0), text_width("88", 10.0));
    }
}
