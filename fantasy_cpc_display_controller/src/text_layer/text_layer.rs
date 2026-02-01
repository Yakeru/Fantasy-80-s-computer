use crate::{config::*, color_palettes::*};

const DEFAULT_COLOR: usize = WHITE;
const DEFAULT_BKG_COLOR: usize = BLACK;

#[derive(Clone, Copy)]
pub struct TextLayerChar {
    pub c: char,
    pub color: usize,
    pub bkg_color: usize,
    pub swap: bool,
    pub blink: bool,
    pub shadowed: bool
}

pub struct TextLayer {
    pub default_color: usize,
    pub default_bkg_color: usize,
    char_map: [Option<TextLayerChar>; TEXT_COLUMNS * TEXT_ROWS],
}

impl Default for TextLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextLayer {
    pub const fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            char_map: [None; TEXT_COLUMNS * TEXT_ROWS]
        }
    }

    pub fn clear(&mut self) {
        self.char_map = [None; TEXT_COLUMNS * TEXT_ROWS];
    }

    /// Returns the dimensions in columns and rowns of the text layer map.
    pub fn get_dimensions_xy(&self) -> (usize, usize) {
         (TEXT_COLUMNS, TEXT_ROWS)
    }

    /// Returns the lenght of the char_map array.
    pub fn get_len(&self) -> usize {
         self.char_map.len()
    }

    pub fn get_char_map(&self) -> &[Option<TextLayerChar>] {
         &self.char_map
    }

    pub fn get_char_map_mut(&mut self) -> &mut [Option<TextLayerChar>] {
         &mut self.char_map
    }

    /// Inserts a TextLayerChar in the char_map at the specified index.
    /// This is the mother of all text inserting functions, all 
    /// the insert or push functions end up calling this one. 
    pub fn insert_text_layer_char(&mut self, index: usize, text_layer_char: TextLayerChar) {
        let safe_index = index % self.get_len();
        self.char_map[safe_index] = Some(text_layer_char);
    }

    /// Inserts a character in the char_map at the specified index.
    pub fn insert_char(&mut self, index: usize, c: char, color: Option<usize>, bkg_color: Option<usize>, swap: bool, blink: bool, shadowed: bool) {
        self.insert_text_layer_char(index, TextLayerChar {c, color: color.unwrap_or(DEFAULT_COLOR), bkg_color: bkg_color.unwrap_or(DEFAULT_BKG_COLOR), swap, blink, shadowed});
    }

    /// Inserts a character in the char_map at the specified x and y position.
    pub fn insert_char_xy(&mut self, x: usize, y: usize, c: char, color: Option<usize>, bkg_color: Option<usize>, swap: bool, blink: bool, shadowed: bool) {
        let index = text_coord_to_index(x, y);
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    /// Inserts a TextLayerChar in the char_map at the specified x and y position.
    pub fn insert_text_layer_char_xy(&mut self, x: usize, y: usize, char: TextLayerChar) {
        let index = text_coord_to_index(x, y);
        self.insert_text_layer_char(index, char);
    }

    /// Inserts a string in the char_map at the specified index.
    pub fn insert_string(&mut self, index: usize, string: &str, color: Option<usize>, bkg_color: Option<usize>, swap: bool, blink: bool, shadowed: bool) {
        if !string.is_empty() {
            for (char_count, c) in string.chars().enumerate() {
                self.insert_char(index + char_count, c, color, bkg_color, swap, blink, shadowed);
            }
        }
    }

    /// Inserts a string in the char_map at the specified x and y position.
    pub fn insert_string_xy(&mut self, x: usize, y: usize, string: &str, color: Option<usize>, bkg_color: Option<usize>, swap: bool, blink: bool, shadowed: bool) {
        let index = text_coord_to_index(x, y);
        self.insert_string(index, string, color, bkg_color, swap, blink, shadowed);
    }
}

pub const fn text_coord_to_index(x: usize, y: usize) -> usize {
    (y * TEXT_COLUMNS + x) % (TEXT_COLUMNS * TEXT_ROWS)
}

pub const fn index_to_text_coord(index: usize) -> (usize, usize) {
    let y: usize = index / TEXT_COLUMNS;
    let x: usize = index % TEXT_COLUMNS;
    (x, y)
}

pub const fn text_coord_to_frame_coord(x: usize, y: usize) -> (usize, usize) {
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * 8) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * 8) / 2;
    let safe_x = x % TEXT_COLUMNS;
    let safe_y = y % TEXT_ROWS;
    let x_pos = horizontal_border + safe_x * 8;
    let y_pos = vertical_border + safe_y * 8;
    (x_pos, y_pos)
}

pub const fn text_index_to_frame_coord(index: usize) -> (usize, usize) {
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * 8) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * 8) / 2;
    let x = horizontal_border + (index % TEXT_COLUMNS) * 8;
    let y = vertical_border + ((index / TEXT_COLUMNS) % TEXT_ROWS) * 8;
    (x, y)
}