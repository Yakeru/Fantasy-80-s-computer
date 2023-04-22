use crate::{config::*, color_palettes::*, characters_rom::*};

const DEFAULT_COLOR: u8 = WHITE;
const DEFAULT_BKG_COLOR: u8 = BLACK;

#[derive(Clone, Copy)]
pub struct TextLayerChar {
    pub c: char,
    pub color: u8,
    pub bkg_color: u8,
    pub swap: bool,
    pub blink: bool,
    pub shadowed: bool
}

pub struct TextLayer {
    pub default_color: u8,
    pub default_bkg_color: u8,
    char_map: [Option<TextLayerChar>; TEXT_COLUMNS * TEXT_ROWS],
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
        return (TEXT_COLUMNS, TEXT_ROWS);
    }

    /// Returns the lenght of the char_map array.
    pub fn get_len(&self) -> usize {
        return self.char_map.len();
    }

    pub fn get_char_map(&self) -> &[Option<TextLayerChar>] {
        return &self.char_map;
    }

    pub fn get_char_map_mut(&mut self) -> &mut [Option<TextLayerChar>] {
        return &mut self.char_map;
    }

    /// Inserts a TextLayerChar in the char_map at the specified index.
    /// This is the mother of all text inserting functions, all 
    /// the insert or push functions end up calling this one. 
    pub fn insert_text_layer_char(&mut self, index: usize, text_layer_char: TextLayerChar) {
        let safe_index = index % self.get_len();
        self.char_map[safe_index] = Some(text_layer_char);
    }

    /// Inserts a character in the char_map at the specified index.
    pub fn insert_char(&mut self, index: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        self.insert_text_layer_char(index, TextLayerChar {c, color: color.unwrap_or(DEFAULT_COLOR), bkg_color: bkg_color.unwrap_or(DEFAULT_BKG_COLOR), swap, blink, shadowed});
    }

    /// Inserts a character in the char_map at the specified x and y position.
    pub fn insert_char_xy(&mut self, x: usize, y: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        let index = text_coord_to_index(x, y);
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    /// Inserts a TextLayerChar in the char_map at the specified x and y position.
    pub fn insert_text_layer_char_xy(&mut self, x: usize, y: usize, char: TextLayerChar) {
        let index = text_coord_to_index(x, y);
        self.insert_text_layer_char(index, char);
    }

    /// Inserts a string in the char_map at the specified index.
    pub fn insert_string(&mut self, index: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        if !string.is_empty() {
            let mut char_count = 0;
            for c in string.chars() {
                self.insert_char(index + char_count, c, color, bkg_color, swap, blink, shadowed);
                char_count = char_count + 1;
            }
        }
    }

    /// Inserts a string in the char_map at the specified x and y position.
    pub fn insert_string_xy(&mut self, x: usize, y: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
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
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * CHARACTER_WIDTH) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * CHARACTER_HEIGHT) / 2;
    let safe_x = x % TEXT_COLUMNS;
    let safe_y = y % TEXT_ROWS;
    let x_pos = horizontal_border + safe_x * CHARACTER_WIDTH;
    let y_pos = vertical_border + safe_y * CHARACTER_HEIGHT;
    (x_pos, y_pos)
}

pub const fn text_index_to_frame_coord(index: usize) -> (usize, usize) {
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * CHARACTER_WIDTH) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * CHARACTER_HEIGHT) / 2;
    let x = horizontal_border + (index % TEXT_COLUMNS) * CHARACTER_WIDTH;
    let y = vertical_border + ((index / TEXT_COLUMNS) % TEXT_ROWS) * CHARACTER_HEIGHT;
    (x, y)
}