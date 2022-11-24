use crate::{config, color_palettes::*};

const TEXT_COLUMNS: usize = config::TEXT_COLUMNS;
const TEXT_ROWS: usize = config::TEXT_ROWS;

const DEFAULT_COLOR: u8 = WHITE.0;
const DEFAULT_BKG_COLOR: u8 = BLACK.0;

#[derive(Copy)]
#[derive(Clone)]
pub struct TextLayerChar {
    c: char,
    color: u8,
    bkg_color: u8,
    swap: bool,
    blink: bool,
    shadowed: bool
}

struct TextLayer {
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
        self.char_map = [None; TEXT_COLUMNS * TEXT_ROWS]
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        return (TEXT_COLUMNS, TEXT_ROWS);
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
        (y * TEXT_COLUMNS + x) % (TEXT_COLUMNS * TEXT_ROWS)
    }

    pub fn insert_char(&mut self, index: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        
        let char: TextLayerChar = TextLayerChar {c, 
            color: color.unwrap_or(self.default_color), 
            bkg_color: bkg_color.unwrap_or(self.default_bkg_color), 
            swap, blink, shadowed};
        

        match self.last_insert_index {
            Some(index) => {
                //If pushing a char after last position,
                //scroll whole layer one line up,
                //set index at first char of last line
                if index == self.char_map.len() - 1 {
                    self.char_map.rotate_left(TEXT_COLUMNS);
                    self.insert_char(index - TEXT_COLUMNS, c, color, bkg_color, swap, blink, shadowed);
                    self.last_insert_index = Some(index - TEXT_COLUMNS);

                } else {
                    self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
                    self.last_insert_index = Some(index);
                }
            },
            None => {
                self.insert_char(0, c, color, bkg_color, swap, blink, shadowed);
                self.last_insert_index = Some(0);
            }
            
        }
    }

    pub fn push_char(&mut self, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        let index = self.last_insert_index.unwrap_or(0) + 1;
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    pub fn pop_char() {

    }

    pub fn push_string(&mut self, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        // let index = self.last_insert_index.unwrap_or(0) + 1;
        // self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
    }

    pub fn insert_char_coord(&mut self, x: usize, y: usize, c: char, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {
        let index = self.coord_to_vec_index(x, y);
        self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
        self.last_insert_index = Some(index);
    }

    pub fn insert_string(&mut self, index: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        for c in string.chars() {
            self.insert_char(index, c, color, bkg_color, swap, blink, shadowed);
            index = index + 1;
        } 

        self.last_insert_index = Some(index);
    }

    pub fn insert_string_coord(&mut self, x: usize, y: usize, string: &str, color: Option<u8>, bkg_color: Option<u8>, swap: bool, blink: bool, shadowed: bool) {

        let mut index = self.coord_to_vec_index(x, y);
        self.insert_string(index, string, color, bkg_color, swap, blink, shadowed);
    }
}