use crate::virtual_frame_buffer::VirtualFrameBuffer;
use crate::characters::rom;

//Struct describing all the settings one character can have in text mode
//"flipp" tells the renderer to flip the color and background of that character
//"blink" tells the renderer to automatically flip the color and background of that character at a set interval, useful for blinking warning messages
pub struct TextModeChar {
    pub c: char,
    pub background_color: u8,
    pub color: u8,
    pub flipp: bool,
    pub blink: bool
}

//The virtual text mode buffer, width and height are expressed in characters
pub struct VirtualTextLayerFrameBuffer {
    character_columns: u32,
    character_rows: u32,
    characters: Vec<TextModeChar>
}

impl VirtualTextLayerFrameBuffer {

    pub fn new(character_columns: u32, character_rows: u32) -> VirtualTextLayerFrameBuffer {

        let fb: Vec<TextModeChar> = Vec::new();

        VirtualTextLayerFrameBuffer {
            character_columns,
            character_rows,
            characters: fb
        }
    }

    pub fn get_characters(&self) -> &Vec<TextModeChar> {
        return &self.characters;
    }

    pub fn push_character(&mut self, tmchar: TextModeChar) {
        self.characters.push(tmchar);
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

    pub fn render(&self, text_layer: &VirtualTextLayerFrameBuffer, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        let horizontal_border: u32 = (virtual_frame_buffer.get_width() as u32 - self.character_columns as u32 * 8) / 2;
        let vertical_border: u32 = (virtual_frame_buffer.get_height() - self.character_rows as u32 * 8) / 2;
    
        let mut x_pos = horizontal_border;
        let mut y_pos = vertical_border;
    
        let mut text_row_count = 0;
        let mut text_col_count = 0;
    
        for character in text_layer.get_characters() {
    
            let pic = rom(&character.c);
    
            for row_count in 0..8 {
    
                let row = pic[row_count];
                let row_in_binary = &format!("{:0>8b}", row);
                let mut character_sprite_col_count = 0;
    
                for c in row_in_binary.chars() {
                    let virtual_frame_buffer_pos = x_pos as usize + character_sprite_col_count + (y_pos as usize + row_count ) * virtual_frame_buffer.get_width() as usize;
                    match c {
                        '0' => virtual_frame_buffer.get_frame()[virtual_frame_buffer_pos] = if character.flipp {character.color} else {character.background_color},
                        '1' => virtual_frame_buffer.get_frame()[virtual_frame_buffer_pos] = if character.flipp {character.background_color} else {character.color},
                        _ => ()
                    }
                    character_sprite_col_count += 1;
                }
            }
    
            text_col_count += 1;
            x_pos += 8;
    
            if text_col_count == text_layer.character_columns {
                text_col_count = 0;
                text_row_count += 1;
                x_pos = horizontal_border;
                y_pos += 8;
            } 
    
            if text_row_count == text_layer.character_rows {
                text_col_count = 0;
                text_row_count = 0;
                x_pos = horizontal_border;
                y_pos = vertical_border;
            }
        }
    }
}