use crate::virtual_frame_buffer::VirtualFrameBuffer;
use crate::characters_rom::rom;

//Struct describing all the settings one character can have in text mode
//TODO implement "flipp" tells the renderer to flip the color and background of that character
//TODO implement "blink" tells the renderer to automatically flip the color and background of that character at a set interval, useful for blinking warning messages
#[derive(Copy, Clone)]
pub struct TextLayerChar {
    pub c: char,
    pub background_color: u8,
    pub color: u8,
    pub flipp: bool,
    pub blink: bool
}

//The virtual text mode buffer, width and height are expressed in characters
pub struct TextLayer {
    columns: u32,
    rows: u32,
    characters: Vec<Option<TextLayerChar>>
}

impl TextLayer {

    pub fn new(columns: u32, rows: u32) -> TextLayer {
        let fb: Vec<Option<TextLayerChar>> = Vec::new();
        TextLayer {
            columns,
            rows,
            characters: fb
        }
    }

    pub fn get_columns(&self) -> u32 {
        return self.columns;
    }

    pub fn get_rows(&self) -> u32 {
        return self.rows;
    }

    pub fn get_characters(&self) -> &Vec<Option<TextLayerChar>> {
        return &self.characters;
    }

    pub fn push_character(&mut self, tmchar: Option<TextLayerChar>) {
        if self.characters.len() == self.columns as usize * self.rows as usize {
            for i in 0..self.characters.len() -1 {
                self.characters[i] = self.characters[i+1];
            }
            self.characters.pop();
            self.characters.push(tmchar);
        } else {
            self.characters.push(tmchar);
        }
    }

    pub fn push_char(&mut self, c: char, color: u8, back_color: u8, blink: bool) {
        let a_char = TextLayerChar {
            c: c,
            background_color: back_color,
            color: color,
            flipp: false,
            blink: blink
        };
        let a_cell = Some(a_char);
        self.push_character(a_cell);
    }

    pub fn push_string(&mut self, string: &str, color: u8, back_color: u8, blink: bool) {
        for c in string.chars() {
            self.push_char(c, color, back_color, blink);
        }
    }

    pub fn push_string_line(&mut self, string: &str, color: u8, back_color: u8, blink: bool) {
        for c in string.chars() {
            self.push_char(c, color, back_color, blink);
        }

        let reminder = self.get_characters().len() % self.columns as usize;
        for _i in 0..(self.columns as usize - reminder) {
            self.push_character(None);
        }
    }

    //pops the last cell, just the last one, wether it contains a character or None.
    pub fn pop_char(&mut self) {
        self.characters.pop();
    }

    pub fn pop_all_none(&mut self) {

        let mut stop: bool = false;

        while match self.characters.last() { //Returns a Option<Option<TextLayerChar>> ... 
            Some(plop) => {
                match plop {
                    Some(t) => {
                        match t.c {
                            '\n' => {stop = true; true}
                            _ => {false}
                        }    
                    }
                    None => {true}
                }
            }
            None => {false}
        } {
            self.characters.pop();
            if stop {return}
        }
    }

    pub fn clear(&mut self) {
        self.characters.clear();
    }

    pub fn scroll_up(&mut self) {
        for i in self.columns as usize..self.characters.len() as usize {
            self.characters[i - self.columns as usize] = self.characters[i];
        }

        for _i in 0..self.columns as usize {
            self.pop_char();
        }
    }
}

pub struct TextLayerRenderer {
    character_columns: u32, 
    character_rows: u32,
    output_frame_px_width: u32,
    output_frame_px_height: u32,
}

impl TextLayerRenderer {

    pub fn new(character_columns: u32, character_rows: u32, output_frame_px_width: u32, output_frame_px_height: u32) -> TextLayerRenderer {
        TextLayerRenderer {
            character_columns,
            character_rows,
            output_frame_px_width,
            output_frame_px_height
        }
    }

    pub fn render(&self, text_layer: &TextLayer, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        let horizontal_border: u32 = (virtual_frame_buffer.get_width() as u32 - self.character_columns as u32 * 8) / 2;
        let vertical_border: u32 = (virtual_frame_buffer.get_height() - self.character_rows as u32 * 8) / 2;
        let mut x_pos = horizontal_border;
        let mut y_pos = vertical_border;
        let mut text_row_count = 0;
        let mut text_col_count = 0;
    
        for character in text_layer.get_characters() {

            if character.is_some() {
                let text_mode_char = character.unwrap();
                    let pic = rom(&text_mode_char.c);
            
                    for row_count in 0..8 {
            
                        let row = pic[row_count];
                        let row_in_binary = &format!("{:0>8b}", row);
                        let mut character_sprite_col_count = 0;
            
                        for c in row_in_binary.chars() {
                            let virtual_frame_buffer_pos = x_pos as usize + character_sprite_col_count + (y_pos as usize + row_count ) * virtual_frame_buffer.get_width() as usize;
                            
                            match c {
                                '0' => virtual_frame_buffer.get_frame()[virtual_frame_buffer_pos] = if text_mode_char.flipp {text_mode_char.color} else {text_mode_char.background_color},
                                '1' => virtual_frame_buffer.get_frame()[virtual_frame_buffer_pos] = if text_mode_char.flipp {text_mode_char.background_color} else {text_mode_char.color},
                                _ => ()
                            }
                            character_sprite_col_count += 1;
                        }
                    }
            }
            
            text_col_count += 1;
            x_pos += 8;
    
            if text_col_count == text_layer.columns {
                text_col_count = 0;
                text_row_count += 1;
                x_pos = horizontal_border;
                y_pos += 8;
            } 
    
            if text_row_count == text_layer.rows {
                text_col_count = 0;
                text_row_count = 0;
                x_pos = horizontal_border;
                y_pos = vertical_border;
            }
        }
    }
}