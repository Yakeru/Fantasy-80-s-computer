use crate::{config, color_palettes::*};

const TEXT_COLUMNS: usize = config::TEXT_COLUMNS;
const TEXT_ROWS: usize = config::TEXT_ROWS;

const DEFAULT_COLOR: u8 = WHITE.0;
const DEFAULT_BKG_COLOR: u8 = BLACK.0;

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
    default_color: u8,
    default_bkg_color: u8,
    char_map: [Option<TextLayerChar>; TEXT_COLUMNS * TEXT_ROWS],
    last_insert_index: Option<usize>
}

impl TextLayer {
    pub const fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            char_map: [None; TEXT_COLUMNS * TEXT_ROWS],
            last_insert_index: None
        }
    }

    pub fn clear(&mut self) {
        self.char_map = [None; TEXT_COLUMNS * TEXT_ROWS];
        self.last_insert_index = None;
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        return (TEXT_COLUMNS, TEXT_ROWS);
    }

    pub fn get_len(&self) -> usize {
        return TEXT_COLUMNS * TEXT_ROWS;
    }

    pub fn get_char_map(&self) -> &[Option<TextLayerChar>] {
        return &self.char_map;
    }

    pub fn get_char_map_mut(&mut self) -> &mut [Option<TextLayerChar>] {
        return &mut self.char_map;
    }

    pub fn set_default_color(&mut self, color: u8) {
        self.default_color = color;
    }

    pub fn set_default_bkg_color(&mut self, bkg_color: u8) {
        self.default_bkg_color = bkg_color;
    }

    pub fn get_default_color(&self) -> u8 {
        self.default_color
    }

    pub fn get_default_bkg_color(&self) -> u8 {
        self.default_bkg_color
    }

    pub fn coord_to_vec_index(&self, x: usize, y: usize) -> usize {
        (y * TEXT_COLUMNS + x) % self.get_len()
    }

    pub fn insert_text_layer_char(&mut self, index: usize, char: TextLayerChar) {

        let safe_index = index % self.get_len();
    
        self.char_map[safe_index] = Some(char);
        self.last_insert_index = Some(safe_index);
    }

    pub fn insert_char(&mut self, index: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        
        let char: TextLayerChar = TextLayerChar {c, 
            color: color.unwrap_or(self.default_color), 
            bkg_color: bkg_color.unwrap_or(self.default_bkg_color), 
            swap, blink, shadowed};

            self.insert_text_layer_char(index, char);
    }

    pub fn push_text_layer_char(&mut self, char: TextLayerChar) {

        let index = match self.last_insert_index {
            Some(i) => i + 1,
            None => 0
        };

        self.insert_text_layer_char(index, char);
    }

    pub fn push_char(&mut self, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        let index = match self.last_insert_index {
            Some(i) => i + 1,
            None => 0
        };

        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    pub fn pop_char(&mut self) {
        match self.last_insert_index {
            Some(i) => {
                self.char_map[i] = None;
                self.last_insert_index = if i == 0 { None } else { Some(i - 1) };
            }
            None => ()
        }
    }

    pub fn pop_chars(&mut self, how_many: usize) {
        if how_many > 0 {
            for _c in 0..how_many {
                match self.last_insert_index {
                    Some(i) => {
                        self.char_map[i] = None;
                        self.last_insert_index = if i == 0 { None } else { Some(i - 1) };
                    }
                    None => ()
                }
            }
        }
    }

    pub fn insert_char_coord(&mut self, x: usize, y: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        let index = self.coord_to_vec_index(x, y);
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
        self.last_insert_index = Some(index);
    }

    pub fn insert_text_layer_char_coord(&mut self, x: usize, y: usize, char: TextLayerChar) {
        let index = self.coord_to_vec_index(x, y);
        self.insert_text_layer_char(index, char);
        self.last_insert_index = Some(index);
    }

    pub fn push_string(&mut self, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        for c in string.chars() {
            self.insert_char(self.last_insert_index.unwrap_or(0) + 1, c, color, bkg_color, swap, blink, shadowed);
        } 
    }

    pub fn insert_string(&mut self, index: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        if !string.is_empty() {
            let mut char_count = 0;
            
            for c in string.chars() {
                if char_count == 0 {
                    self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
                } else {
                    self.push_char(c, color, bkg_color, swap, blink, shadowed);
                }

                char_count = char_count + 1;
            }
        }
    }

    pub fn insert_string_coord(&mut self, x: usize, y: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        let index = self.coord_to_vec_index(x, y);
        self.insert_string(index, string, color, bkg_color, swap, blink, shadowed);
    }
}