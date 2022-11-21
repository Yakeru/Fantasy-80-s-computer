use crate::config;

const TEXT_COLUMNS: usize = config::TEXT_COLUMNS;
const TEXT_ROWS: usize = config::TEXT_ROWS;

const DEFAULT_COLOR: u8 = 7;
const DEFAULT_BKG_COLOR: u8 = 0;

// u16 color_map structure :
// 0000 0000 0000 0000
//     |         |___ 8 ls bit background color index
//     |__ 8 ms bit char color index


// u8 effect_map structure
// 0000 0000
//       |||__ swap colors flag
//       ||__ blink flag
//       |__ shadowed (draws a black checkered pattern on top)


struct Console {
    default_color: u8,
    default_bkg_color: u8,
    size_x: usize,
    size_y: usize,
    pos_x: usize,
    pos_y: usize,
    buffer: Vec<(char, u16, u8)>,
    cursor_pos: usize
}

pub struct TextLayer {
    default_color: u8,
    default_bkg_color: u8,
    color_map: [Option<u16>; TEXT_COLUMNS * TEXT_ROWS],
    effect_map: [Option<u8>; TEXT_COLUMNS * TEXT_ROWS],
    char_map: [Option<char>; TEXT_COLUMNS * TEXT_ROWS]
}

impl TextLayer {
    pub fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            color_map: [None; TEXT_COLUMNS * TEXT_ROWS],
            effect_map: [None; TEXT_COLUMNS * TEXT_ROWS],
            char_map: [None; TEXT_COLUMNS * TEXT_ROWS]
        }
    }

    pub fn clear_colors(&mut self) {
        self.color_map = [None; TEXT_COLUMNS * TEXT_ROWS];
    }

    pub fn clear_effects(&mut self) {
        self.effect_map = [None; TEXT_COLUMNS * TEXT_ROWS];
    }

    pub fn clear_chars(&mut self) {
        self.char_map = [None; TEXT_COLUMNS * TEXT_ROWS];
    }

    pub fn clear(&mut self) {
        self.clear_chars();
        self.clear_colors();
        self.clear_effects();
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        return (TEXT_COLUMNS, TEXT_ROWS);
    }

    pub fn get_size(&self) -> usize {
        return TEXT_COLUMNS * TEXT_ROWS;
    }

    pub fn get_char_map(&mut self) -> &mut [Option<char>] {
        return &mut self.char_map;
    }

    pub fn get_color_map(&mut self) -> &mut [Option<u16>] {
        return &mut self.color_map;
    }

    pub fn get_effect_map(&mut self) -> &mut [Option<u8>] {
        return &mut self.effect_map;
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

    fn coord_to_vec_index(&self, x: usize, y: usize) -> usize {
        (y * TEXT_COLUMNS + x) % (TEXT_COLUMNS * TEXT_ROWS)
    }

    pub fn insert_char(&mut self, index: usize, char: char, color: Option<u16>, effect: Option<u8>) {
        
        self.char_map[index] = Some(char);

        match color {
            Some(_c) => self.color_map[index] = color,
            None => ()
        }

        match effect {
            Some(_e) => self.effect_map[index] = effect,
            None => ()
        }
    }

    pub fn insert_char_coord(&mut self, x: usize, y: usize, char: char, color: Option<u16>, effect: Option<u8>) {
        let index = self.coord_to_vec_index(x, y);
        self.insert_char(index, char, color, effect);
    }

    pub fn insert_string_coord(&mut self, x: usize, y: usize, string: &str, color: Option<u16>, effect: Option<u8>) {

        let index = self.coord_to_vec_index(x, y);
        let mut offset = 0;

        for c in string.chars() {
            self.insert_char(index + offset, c, color, effect);
            offset = offset + 1;
        } 
    }
}