use crate::config;

const TEXT_COLUMNS: usize = config::TEXT_COLUMNS;
const TEXT_ROWS: usize = config::TEXT_ROWS;

const DEFAULT_COLOR: u8 = 7;
const DEFAULT_BKG_COLOR: u8 = 0;

// u32 color_map structure :
// 0000 0000 0000 0000
//       |||   |   |___ 4 bit background color index
//       |||   |__ 4 bit char color index
//       |||__ swap colors flag
//       ||__ blink flag
//       |__ shadowed (draws a black checkered pattern on top)

/// The text layer buffer
pub struct TextLayer {
    default_color: u8,
    default_bkg_color: u8,
    color_map: [Option<u16>; TEXT_COLUMNS * TEXT_ROWS],
    char_map: [Option<char>; TEXT_COLUMNS * TEXT_ROWS],
}

impl TextLayer {
    pub fn new() -> TextLayer {
        TextLayer {
            default_color: DEFAULT_COLOR,
            default_bkg_color: DEFAULT_BKG_COLOR,
            color_map: [None; TEXT_COLUMNS * TEXT_ROWS],
            char_map: [None; TEXT_COLUMNS * TEXT_ROWS]
        }
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

    pub fn insert_char_coord(&mut self, x: usize, y: usize, char: char, color: Option<u16>) {
        let index = self.coord_to_vec_index(x, y);
        self.char_map[index] = Some(char);

        match color {
            Some(_c) => self.color_map[index] = color,
            None => ()
        }
    }

    pub fn insert_char(&mut self, index: usize, char: char, color: Option<u16>) {
        
        self.char_map[index] = Some(char);

        match color {
            Some(_c) => self.color_map[index] = color,
            None => ()
        }
    }

    pub fn insert_string_coord(&mut self, x: usize, y: usize, string: &str, color: Option<u16>) {

        let index = self.coord_to_vec_index(x, y);
        let mut offset = 0;

        for c in string.chars() {
            self.insert_char(index + offset, c, color);
            offset = offset + 1;
        } 
    }
}