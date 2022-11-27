use crate::{config::{self}, color_palettes::*, text_layer_char::{TextLayerChar}, console::Console};

const TEXT_LAYER_COLUMNS: usize = config::TEXT_COLUMNS;
const TEXT_LAYER_ROWS: usize = config::TEXT_ROWS;
const DEFAULT_COLOR: u8 = WHITE.0;
const DEFAULT_BKG_COLOR: u8 = BLACK.0;

pub const fn coord_to_vec_index(x: usize, y: usize) -> usize {
    (y * TEXT_LAYER_COLUMNS + x) % (TEXT_LAYER_COLUMNS * TEXT_LAYER_ROWS)
}

pub struct TextLayer {
    pub default_color: u8,
    pub default_bkg_color: u8,
    char_map: [Option<TextLayerChar>; TEXT_LAYER_COLUMNS * TEXT_LAYER_ROWS],
}

impl TextLayer {
    pub const fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            char_map: [None; TEXT_LAYER_COLUMNS * TEXT_LAYER_ROWS]
        }
    }

    pub fn clear(&mut self) {
        self.char_map = [None; TEXT_LAYER_COLUMNS * TEXT_LAYER_ROWS];
    }

    /// Returns the dimensions in columns and rowns of the text layer map.
    pub fn get_dimensions_xy(&self) -> (usize, usize) {
        return (TEXT_LAYER_COLUMNS, TEXT_LAYER_ROWS);
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
        let index = coord_to_vec_index(x, y);
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    /// Inserts a TextLayerChar in the char_map at the specified x and y position.
    pub fn insert_text_layer_char_xy(&mut self, x: usize, y: usize, char: TextLayerChar) {
        let index = coord_to_vec_index(x, y);
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
        let index = coord_to_vec_index(x, y);
        self.insert_string(index, string, color, bkg_color, swap, blink, shadowed);
    }

    /// Renders the content of the console in the char_map
    pub fn render_console(&mut self, console: &Console) {

        let top_left_corner_index = coord_to_vec_index(console.pos_x, console.pos_y);

        let mut char_index = 0;

        for i in 0..console.rows {
            for j in 0..console.columns {
                let index = top_left_corner_index + j + i * TEXT_LAYER_COLUMNS;
                
                match console.content.get(char_index) {
                    Some(char) => {
                        self.insert_text_layer_char(index, *char);
                    },
                    None => {
                        self.insert_char(index, ' ', Some(console.default_color), Some(console.default_bkg_color), false, false, false);
                    }
                }
                
                if char_index == console.content.len() {
                    self.insert_text_layer_char(index, console.cursor);
                }
                
                char_index = char_index + 1;
            }
        }
    }

}