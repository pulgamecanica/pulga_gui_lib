//! Built-in 9×9 pixel icons, drawn as ASCII art like the [`crate::font`] and
//! stored in the same atlas, so a whole UI draws from one texture.
//!
//! Some icons are derived from others (mirrored, flipped or transposed) so
//! a pair like undo/redo always matches exactly.

use crate::font;

/// Icon size in pixels (square).
pub const ICON_SIZE: usize = 9;
/// Width of an icon's slot in the atlas: icon plus one empty column.
const SLOT_W: usize = ICON_SIZE + 1;

/// Every built-in icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    /// Curved arrow back.
    Undo,
    /// Curved arrow forward.
    Redo,
    /// Floppy disk.
    Save,
    /// Right-pointing triangle.
    Play,
    /// Square.
    Stop,
    /// Plus sign.
    Plus,
    /// Trash can.
    Trash,
    /// Two overlapping sheets.
    Copy,
    /// Folder.
    Folder,
    /// Isometric cube.
    Cube,
    /// Crosshair.
    Focus,
    /// Chevron pointing right (collapsed).
    ChevronRight,
    /// Chevron pointing down (expanded).
    ChevronDown,
    /// Arrow pointing up.
    ArrowUp,
    /// Arrow pointing down.
    ArrowDown,
    /// Cross.
    Close,
    /// Check mark.
    Check,
    /// Two opposing arrows.
    Shuffle,
    /// Numbered list.
    Sequence,
    /// Running figure.
    Run,
    /// Half-filled circle.
    Theme,
    /// Small filled box.
    Box,
}

impl Icon {
    /// Every icon, in atlas order.
    pub const ALL: [Icon; 22] = [
        Icon::Undo,
        Icon::Redo,
        Icon::Save,
        Icon::Play,
        Icon::Stop,
        Icon::Plus,
        Icon::Trash,
        Icon::Copy,
        Icon::Folder,
        Icon::Cube,
        Icon::Focus,
        Icon::ChevronRight,
        Icon::ChevronDown,
        Icon::ArrowUp,
        Icon::ArrowDown,
        Icon::Close,
        Icon::Check,
        Icon::Shuffle,
        Icon::Sequence,
        Icon::Run,
        Icon::Theme,
        Icon::Box,
    ];
}

type Art = [&'static str; ICON_SIZE];

#[rustfmt::skip]
const UNDO: Art = [
    ".........",
    "..#......",
    ".##......",
    "#######..",
    ".##....#.",
    "..#.....#",
    "........#",
    ".......#.",
    "...####..",
];
#[rustfmt::skip]
const SAVE: Art = [
    "#######..",
    "#.#...##.",
    "#.#...#.#",
    "#.....#.#",
    "#######.#",
    "#.......#",
    "#.#####.#",
    "#.#####.#",
    "#########",
];
#[rustfmt::skip]
const PLAY: Art = [
    ".#.......",
    ".###.....",
    ".#####...",
    ".#######.",
    ".########",
    ".#######.",
    ".#####...",
    ".###.....",
    ".#.......",
];
#[rustfmt::skip]
const STOP: Art = [
    ".........",
    ".#######.",
    ".#######.",
    ".#######.",
    ".#######.",
    ".#######.",
    ".#######.",
    ".#######.",
    ".........",
];
#[rustfmt::skip]
const PLUS: Art = [
    ".........",
    "....#....",
    "....#....",
    "....#....",
    ".#######.",
    "....#....",
    "....#....",
    "....#....",
    ".........",
];
#[rustfmt::skip]
const TRASH: Art = [
    "...###...",
    "#########",
    ".#.....#.",
    ".#.#.#.#.",
    ".#.#.#.#.",
    ".#.#.#.#.",
    ".#.#.#.#.",
    ".#.....#.",
    "..#####..",
];
#[rustfmt::skip]
const COPY: Art = [
    "######...",
    "#....#...",
    "#..######",
    "#..#....#",
    "#..#....#",
    "####....#",
    "...#....#",
    "...#....#",
    "...######",
];
#[rustfmt::skip]
const FOLDER: Art = [
    ".........",
    "####.....",
    "#...#####",
    "#.......#",
    "#.......#",
    "#.......#",
    "#.......#",
    "#########",
    ".........",
];
#[rustfmt::skip]
const CUBE: Art = [
    "...###...",
    ".##...##.",
    "#..###..#",
    "#...#...#",
    "#...#...#",
    "#...#...#",
    ".##.#.##.",
    "...###...",
    ".........",
];
#[rustfmt::skip]
const FOCUS: Art = [
    "....#....",
    "..#####..",
    ".#..#..#.",
    ".#.....#.",
    "###.#.###",
    ".#.....#.",
    ".#..#..#.",
    "..#####..",
    "....#....",
];
#[rustfmt::skip]
const CHEVRON_RIGHT: Art = [
    ".........",
    "...#.....",
    "...##....",
    "....##...",
    ".....##..",
    "....##...",
    "...##....",
    "...#.....",
    ".........",
];
#[rustfmt::skip]
const ARROW_UP: Art = [
    "....#....",
    "...###...",
    "..#.#.#..",
    ".#..#..#.",
    "....#....",
    "....#....",
    "....#....",
    "....#....",
    "....#....",
];
#[rustfmt::skip]
const CLOSE: Art = [
    ".........",
    ".#.....#.",
    "..#...#..",
    "...#.#...",
    "....#....",
    "...#.#...",
    "..#...#..",
    ".#.....#.",
    ".........",
];
#[rustfmt::skip]
const CHECK: Art = [
    ".........",
    "........#",
    ".......##",
    "......##.",
    "#....##..",
    "##..##...",
    ".####....",
    "..##.....",
    ".........",
];
#[rustfmt::skip]
const SHUFFLE: Art = [
    "......#..",
    "......##.",
    "#########",
    "......##.",
    "..#...#..",
    ".##......",
    "#########",
    ".##......",
    "..#......",
];
#[rustfmt::skip]
const SEQUENCE: Art = [
    ".........",
    "##.######",
    ".........",
    ".........",
    "##.######",
    ".........",
    ".........",
    "##.######",
    ".........",
];
#[rustfmt::skip]
const RUN: Art = [
    ".....##..",
    ".....##..",
    "...###...",
    "..#.##.#.",
    "....##...",
    "...#..#..",
    "..#....#.",
    ".#.....#.",
    "#........",
];
#[rustfmt::skip]
const THEME: Art = [
    "...###...",
    ".##.####.",
    ".#..####.",
    "#...#####",
    "#...#####",
    "#...#####",
    ".#..####.",
    ".##.####.",
    "...###...",
];
#[rustfmt::skip]
const BOX: Art = [
    ".........",
    ".........",
    "..#####..",
    "..#####..",
    "..#####..",
    "..#####..",
    "..#####..",
    ".........",
    ".........",
];

/// Whether pixel `(x, y)` of `icon` is on.
pub fn pixel(icon: Icon, x: usize, y: usize) -> bool {
    let at = |art: &Art, x: usize, y: usize| art[y].as_bytes()[x] == b'#';
    let last = ICON_SIZE - 1;
    match icon {
        Icon::Undo => at(&UNDO, x, y),
        Icon::Redo => at(&UNDO, last - x, y),
        Icon::Save => at(&SAVE, x, y),
        Icon::Play => at(&PLAY, x, y),
        Icon::Stop => at(&STOP, x, y),
        Icon::Plus => at(&PLUS, x, y),
        Icon::Trash => at(&TRASH, x, y),
        Icon::Copy => at(&COPY, x, y),
        Icon::Folder => at(&FOLDER, x, y),
        Icon::Cube => at(&CUBE, x, y),
        Icon::Focus => at(&FOCUS, x, y),
        Icon::ChevronRight => at(&CHEVRON_RIGHT, x, y),
        Icon::ChevronDown => at(&CHEVRON_RIGHT, y, x),
        Icon::ArrowUp => at(&ARROW_UP, x, y),
        Icon::ArrowDown => at(&ARROW_UP, x, last - y),
        Icon::Close => at(&CLOSE, x, y),
        Icon::Check => at(&CHECK, x, y),
        Icon::Shuffle => at(&SHUFFLE, x, y),
        Icon::Sequence => at(&SEQUENCE, x, y),
        Icon::Run => at(&RUN, x, y),
        Icon::Theme => at(&THEME, x, y),
        Icon::Box => at(&BOX, x, y),
    }
}

/// Atlas columns the icons take.
pub(crate) fn atlas_width() -> usize {
    Icon::ALL.len() * SLOT_W
}

/// Paints every icon with `set(x, y)`, starting at atlas column `x0`.
pub(crate) fn draw_into(x0: usize, set: &mut impl FnMut(usize, usize)) {
    for (i, &icon) in Icon::ALL.iter().enumerate() {
        for y in 0..ICON_SIZE {
            for x in 0..ICON_SIZE {
                if pixel(icon, x, y) {
                    set(x0 + i * SLOT_W + x, y);
                }
            }
        }
    }
}

/// Where `icon` sits in [`font::atlas`], as `[u0, v0, u1, v1]`.
pub fn uv(icon: Icon) -> [f32; 4] {
    let i = Icon::ALL.iter().position(|&c| c == icon).unwrap();
    let (w, h) = (font::atlas_width() as f32, font::CELL_H as f32);
    let x0 = (font::GLYPHS.len() * font::SLOT_W + i * SLOT_W) as f32;
    [
        x0 / w,
        0.0,
        (x0 + ICON_SIZE as f32) / w,
        ICON_SIZE as f32 / h,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::wasm_bindgen_test;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn art_is_well_formed() {
        let arts = [
            UNDO,
            SAVE,
            PLAY,
            STOP,
            PLUS,
            TRASH,
            COPY,
            FOLDER,
            CUBE,
            FOCUS,
            CHEVRON_RIGHT,
            ARROW_UP,
            CLOSE,
            CHECK,
            SHUFFLE,
            SEQUENCE,
            RUN,
            THEME,
            BOX,
        ];
        for art in arts {
            for row in art {
                assert_eq!(row.len(), ICON_SIZE, "{row:?}");
                assert!(row.bytes().all(|b| b == b'#' || b == b'.'), "{row:?}");
            }
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn derived_icons_mirror_their_source() {
        for y in 0..ICON_SIZE {
            for x in 0..ICON_SIZE {
                assert_eq!(
                    pixel(Icon::Redo, x, y),
                    pixel(Icon::Undo, ICON_SIZE - 1 - x, y)
                );
                assert_eq!(
                    pixel(Icon::ChevronDown, x, y),
                    pixel(Icon::ChevronRight, y, x)
                );
            }
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn every_icon_has_ink_and_its_own_atlas_slot() {
        let (pixels, w, _) = font::atlas();
        for icon in Icon::ALL {
            let [u0, ..] = uv(icon);
            let x0 = (u0 * w as f32).round() as usize;
            let inked = (0..ICON_SIZE)
                .flat_map(|y| (0..ICON_SIZE).map(move |x| (x, y)))
                .any(|(x, y)| pixels[(y * w + x0 + x) * 4 + 3] == 255);
            assert!(inked, "{icon:?} is blank in the atlas");
        }
    }
}
