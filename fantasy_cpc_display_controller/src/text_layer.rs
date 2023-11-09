use crate::{characters_rom::*, color_palettes::*, config::*};

const DEFAULT_COLOR: usize = WHITE;
const DEFAULT_BKG_COLOR: usize = BLACK;

#[derive(Clone, Copy)]
pub struct TextLayerCell {
    pub c: char,
    pub pen: TextLayerPen,
}

#[derive(Clone, Copy)]
pub struct TextLayerPen {
    pub color: usize,
    pub bkg_color: usize,
    pub swap_color: bool,
    pub blink: bool,
    pub shadowed: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

const fn get_default_pen() -> TextLayerPen {
    TextLayerPen {
        color: DEFAULT_COLOR,
        bkg_color: DEFAULT_BKG_COLOR,
        swap_color: false,
        blink: false,
        shadowed: false,
        flip_h: false,
        flip_v: false,
    }
}

pub struct TextLayer {
    pub default_color: usize,
    pub default_bkg_color: usize,
    default_pen: TextLayerPen,
    current_pen: TextLayerPen,
    char_map: [[Option<TextLayerCell>; TEXT_COLUMNS]; TEXT_ROWS],
}

impl TextLayer {
    pub const fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            default_pen: get_default_pen(),
            current_pen: get_default_pen(),
            char_map: [[None; TEXT_COLUMNS]; TEXT_ROWS],
        }
    }

    pub fn clear(&mut self) {
        self.char_map = [[None; TEXT_COLUMNS]; TEXT_ROWS];
    }

    pub fn reset_pen(&mut self) {
        self.current_pen = get_default_pen();
    }

    pub fn set_pen_colors(&mut self, color: usize, bkg_color: usize) {
        self.current_pen.color = color;
        self.current_pen.bkg_color = bkg_color;
    }

    pub fn set_pen_color(&mut self, color: usize) {
        self.current_pen.color = color
    }

    pub fn set_pen_bkg_color(&mut self, bkg_color: usize) {
        self.current_pen.bkg_color = bkg_color
    }

    pub fn set_pen_style_effect(&mut self, swap_color: bool, blink: bool, shadowed: bool) {
        self.current_pen.swap_color = swap_color;
        self.current_pen.blink = blink;
        self.current_pen.shadowed = shadowed;
    }

    pub fn set_pen_render_effect(&mut self, flip_h: bool, flip_v: bool) {
        self.current_pen.flip_h = flip_h;
        self.current_pen.flip_v = flip_v;
    }

    /// Returns the dimensions in columns and rowns of the text layer map.
    pub fn get_dimensions_xy(&self) -> (usize, usize) {
        (TEXT_COLUMNS, TEXT_ROWS)
    }

    /// Returns the lenght of the char_map array.
    pub fn get_len(&self) -> usize {
        TEXT_COLUMNS * TEXT_ROWS
    }

    pub fn get_char_map(&self) -> &[[Option<TextLayerCell>; TEXT_COLUMNS]; TEXT_ROWS] {
        &self.char_map
    }

    pub fn get_char_map_mut(&mut self) -> &mut [[Option<TextLayerCell>; TEXT_COLUMNS]; TEXT_ROWS] {
        &mut self.char_map
    }

    /// Inserts a character in the char_map at the specified x and y position.
    pub fn write(&mut self, c: char, x: usize, y: usize) {
        let cell: TextLayerCell = TextLayerCell {
            c,
            pen: self.current_pen,
        };
        let safe_coord = safe_coord(x, y);
        self.char_map[safe_coord.1][safe_coord.0] = Some(cell);
    }

    /// Inserts a string in the char_map at the specified index.
    pub fn write_str(&mut self, x: usize, y: usize, string: &str) {
        if !string.is_empty() {
            for (char_count, c) in string.chars().enumerate() {
                self.write(c, x + char_count, y);
            }
        }
    }
}

// pub const fn text_coord_to_index(x: usize, y: usize) -> usize {
//     (y * TEXT_COLUMNS + x) % (TEXT_COLUMNS * TEXT_ROWS)
// }

const fn safe_coord(x: usize, y: usize) -> (usize, usize) {
    (x % TEXT_COLUMNS, (y + x / TEXT_COLUMNS) % TEXT_ROWS)
}

pub const fn text_coord_to_frame_coord(x: usize, y: usize) -> (usize, usize) {
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * CHARACTER_WIDTH) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * CHARACTER_HEIGHT) / 2;
    let safe_x = x % TEXT_COLUMNS;
    let safe_y = y % TEXT_ROWS;
    let x_pos = horizontal_border + safe_x * CHARACTER_WIDTH;
    let y_pos = vertical_border + safe_y * CHARACTER_HEIGHT;
    (x_pos, y_pos)
}
