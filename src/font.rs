//! The built-in bitmap font, drawn by hand as 5×7 ASCII art.
//!
//! Defining glyphs in code keeps text rendering free of font files, font
//! libraries and image decoders (the subject only allows single-format image
//! libraries). Each glyph sits in a [`CELL_W`]×[`CELL_H`] cell, which leaves
//! one blank column and row as letter/line spacing.
//!
//! Lowercase letters render as uppercase; any other unsupported character
//! renders as `?`.

/// Glyph width in pixels (`#`/`.` per row).
pub const GLYPH_W: usize = 5;
/// Glyph height in pixels (rows per glyph).
pub const GLYPH_H: usize = 7;
/// Cell width: glyph plus one column of spacing.
pub const CELL_W: usize = GLYPH_W + 1;
/// Cell height: glyph plus one row of line spacing.
pub const CELL_H: usize = GLYPH_H + 1;

/// Every supported character and its pixels: `#` is on, `.` is off.
#[rustfmt::skip]
pub const GLYPHS: &[(char, [&str; GLYPH_H])] = &[
    (' ', [".....", ".....", ".....", ".....", ".....", ".....", "....."]),
    ('0', [".###.", "#...#", "#..##", "#.#.#", "##..#", "#...#", ".###."]),
    ('1', ["..#..", ".##..", "..#..", "..#..", "..#..", "..#..", ".###."]),
    ('2', [".###.", "#...#", "....#", "...#.", "..#..", ".#...", "#####"]),
    ('3', ["#####", "...#.", "..#..", "...#.", "....#", "#...#", ".###."]),
    ('4', ["...#.", "..##.", ".#.#.", "#..#.", "#####", "...#.", "...#."]),
    ('5', ["#####", "#....", "####.", "....#", "....#", "#...#", ".###."]),
    ('6', ["..##.", ".#...", "#....", "####.", "#...#", "#...#", ".###."]),
    ('7', ["#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#..."]),
    ('8', [".###.", "#...#", "#...#", ".###.", "#...#", "#...#", ".###."]),
    ('9', [".###.", "#...#", "#...#", ".####", "....#", "...#.", ".##.."]),
    ('A', [".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]),
    ('B', ["####.", "#...#", "#...#", "####.", "#...#", "#...#", "####."]),
    ('C', [".###.", "#...#", "#....", "#....", "#....", "#...#", ".###."]),
    ('D', ["###..", "#..#.", "#...#", "#...#", "#...#", "#..#.", "###.."]),
    ('E', ["#####", "#....", "#....", "####.", "#....", "#....", "#####"]),
    ('F', ["#####", "#....", "#....", "####.", "#....", "#....", "#...."]),
    ('G', [".###.", "#...#", "#....", "#.###", "#...#", "#...#", ".####"]),
    ('H', ["#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#"]),
    ('I', [".###.", "..#..", "..#..", "..#..", "..#..", "..#..", ".###."]),
    ('J', ["..###", "...#.", "...#.", "...#.", "...#.", "#..#.", ".##.."]),
    ('K', ["#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#"]),
    ('L', ["#....", "#....", "#....", "#....", "#....", "#....", "#####"]),
    ('M', ["#...#", "##.##", "#.#.#", "#.#.#", "#...#", "#...#", "#...#"]),
    ('N', ["#...#", "#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#"]),
    ('O', [".###.", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]),
    ('P', ["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."]),
    ('Q', [".###.", "#...#", "#...#", "#...#", "#.#.#", "#..#.", ".##.#"]),
    ('R', ["####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#"]),
    ('S', [".####", "#....", "#....", ".###.", "....#", "....#", "####."]),
    ('T', ["#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."]),
    ('U', ["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".###."]),
    ('V', ["#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#.."]),
    ('W', ["#...#", "#...#", "#...#", "#.#.#", "#.#.#", "#.#.#", ".#.#."]),
    ('X', ["#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#"]),
    ('Y', ["#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#.."]),
    ('Z', ["#####", "....#", "...#.", "..#..", ".#...", "#....", "#####"]),
    ('.', [".....", ".....", ".....", ".....", ".....", ".##..", ".##.."]),
    (',', [".....", ".....", ".....", ".....", ".##..", "..#..", ".#..."]),
    (':', [".....", ".##..", ".##..", ".....", ".##..", ".##..", "....."]),
    ('!', ["..#..", "..#..", "..#..", "..#..", "..#..", ".....", "..#.."]),
    ('?', [".###.", "#...#", "....#", "...#.", "..#..", ".....", "..#.."]),
    ('-', [".....", ".....", ".....", "#####", ".....", ".....", "....."]),
    ('+', [".....", "..#..", "..#..", "#####", "..#..", "..#..", "....."]),
    ('=', [".....", ".....", "#####", ".....", "#####", ".....", "....."]),
    ('/', [".....", "....#", "...#.", "..#..", ".#...", "#....", "....."]),
    ('(', ["...#.", "..#..", ".#...", ".#...", ".#...", "..#..", "...#."]),
    (')', [".#...", "..#..", "...#.", "...#.", "...#.", "..#..", ".#..."]),
    ('<', ["...#.", "..#..", ".#...", "#....", ".#...", "..#..", "...#."]),
    ('>', [".#...", "..#..", "...#.", "....#", "...#.", "..#..", ".#..."]),
    ('\'', ["..#..", "..#..", ".#...", ".....", ".....", ".....", "....."]),
    ('%', ["##...", "##..#", "...#.", "..#..", ".#...", "#..##", "...##"]),
    ('#', [".#.#.", ".#.#.", "#####", ".#.#.", "#####", ".#.#.", ".#.#."]),
    ('*', [".....", "#.#.#", ".###.", "#####", ".###.", "#.#.#", "....."]),
    ('_', [".....", ".....", ".....", ".....", ".....", ".....", "#####"]),
];

/// Index into [`GLYPHS`] used to draw `c`.
pub fn glyph_index(c: char) -> usize {
    let c = c.to_ascii_uppercase();
    GLYPHS
        .iter()
        .position(|&(g, _)| g == c)
        .or_else(|| GLYPHS.iter().position(|&(g, _)| g == '?'))
        .expect("'?' is in GLYPHS")
}

/// The whole font as one row of cells: an RGBA image
/// `GLYPHS.len() * CELL_W` wide and [`CELL_H`] tall, rows from the top,
/// opaque white where a glyph pixel is on and transparent elsewhere (so a
/// draw color tints it).
pub fn atlas() -> (Vec<u8>, usize, usize) {
    let (w, h) = (GLYPHS.len() * CELL_W, CELL_H);
    let mut pixels = vec![0u8; w * h * 4];
    for (i, (_, rows)) in GLYPHS.iter().enumerate() {
        for (y, row) in rows.iter().enumerate() {
            for (x, px) in row.bytes().enumerate() {
                if px == b'#' {
                    let at = (y * w + i * CELL_W + x) * 4;
                    pixels[at..at + 4].copy_from_slice(&[255; 4]);
                }
            }
        }
    }
    (pixels, w, h)
}

/// Width in pixels of the longest line of `text` drawn `size` pixels tall,
/// for centering or right-aligning.
pub fn text_width(text: &str, size: f32) -> f32 {
    let longest = text.lines().map(|l| l.chars().count()).max().unwrap_or(0);
    longest as f32 * CELL_W as f32 * size / CELL_H as f32
}

/// Where each visible glyph of `text` goes: `(dst, uv)` pairs, `dst` as
/// `[x, y, w, h]` pixels with the text's top-left at (`x`, `y`), `uv` as
/// `[u0, v0, u1, v1]` into [`atlas`]. `size` is the line height in pixels;
/// font pixels scale by `size / 8`, so multiples of 8 stay crisp. `\n`
/// starts a new line; spaces produce nothing.
pub fn layout(x: f32, y: f32, size: f32, text: &str) -> Vec<([f32; 4], [f32; 4])> {
    let atlas_w = (GLYPHS.len() * CELL_W) as f32;
    let px = size / CELL_H as f32; // screen pixels per font pixel
    let (cw, ch) = (CELL_W as f32 * px, CELL_H as f32 * px);
    let mut out = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == ' ' {
                continue;
            }
            let u0 = (glyph_index(c) * CELL_W) as f32 / atlas_w;
            let dst = [x + col as f32 * cw, y + line_no as f32 * ch, cw, ch];
            out.push((dst, [u0, 0.0, u0 + CELL_W as f32 / atlas_w, 1.0]));
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
            for row in rows {
                assert_eq!(row.len(), GLYPH_W, "{c:?} row {row:?}");
                assert!(
                    row.bytes().all(|b| b == b'#' || b == b'.'),
                    "{c:?} row {row:?}"
                );
            }
            assert!(
                !GLYPHS[..i].iter().any(|(d, _)| d == c),
                "{c:?} defined twice"
            );
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn lowercase_maps_to_uppercase_and_unknown_to_question_mark() {
        assert_eq!(glyph_index('a'), glyph_index('A'));
        assert_eq!(glyph_index('é'), glyph_index('?'));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn atlas_places_glyph_pixels_in_their_cell() {
        let (pixels, w, h) = atlas();
        assert_eq!((w, h), (GLYPHS.len() * CELL_W, CELL_H));
        let alpha = |x: usize, y: usize| pixels[(y * w + x) * 4 + 3];
        // '-' is a full middle row: on across the glyph, off in the spacing column.
        let x0 = glyph_index('-') * CELL_W;
        assert!((0..GLYPH_W).all(|x| alpha(x0 + x, 3) == 255));
        assert_eq!(alpha(x0 + GLYPH_W, 3), 0);
        // The spacing row under every glyph stays empty.
        assert!((0..w).all(|x| alpha(x, CELL_H - 1) == 0));
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn layout_skips_spaces_and_advances_by_cell_and_line() {
        // size 16 = 2 screen pixels per font pixel: cells are 12 wide, 16 tall.
        let quads = layout(10.0, 20.0, 16.0, "A B\nC");
        let origins: Vec<(f32, f32)> = quads.iter().map(|(d, _)| (d[0], d[1])).collect();
        assert_eq!(origins, [(10.0, 20.0), (34.0, 20.0), (10.0, 36.0)]);
        assert_eq!(quads[0].0[2..], [12.0, 16.0]);
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn text_width_uses_the_longest_line() {
        assert_eq!(text_width("ABC\nA", 8.0), 18.0);
        assert_eq!(text_width("", 8.0), 0.0);
    }
}
