use crate::{DisplayController, config::VIRTUAL_WIDTH, text_layer::{character_rom_trait::FantasyCpc8by8CharacterRomTrait, default_character_rom::FantasyCpcAmstradCharacterSet, text_layer::{TextLayerChar, text_index_to_frame_coord}}};

impl DisplayController {
    pub fn text_layer_renderer(&mut self) {
        for char_counter in 0..self.text_layer.get_len() {
            let frame_coord = text_index_to_frame_coord(char_counter);

            let text_layer_char = self.text_layer.get_char_map()[char_counter];

            if let Some(char_struct) = text_layer_char {
                self.text_layer_char_renderer(&char_struct, frame_coord.0, frame_coord.1);
            }
        }
    }

    fn text_layer_char_renderer(
        &mut self,
        text_layer_char: &TextLayerChar,
        frame_x_pos: usize,
        frame_y_pos: usize,
    ) {
        let char = text_layer_char.c;
        let char_color = text_layer_char.color;
        let bck_color = text_layer_char.bkg_color;
        let blink = text_layer_char.blink;
        let swap = text_layer_char.swap;
        let shadowed = text_layer_char.shadowed;

        //set color, swap or not
        let text_color = if swap || (blink && self.clock.half_second_latch) {
            bck_color
        } else {
            char_color
        };
        let text_bkg_color = if swap || (blink && self.clock.half_second_latch) {
            char_color
        } else {
            bck_color
        };

        //Get char picture from  "character rom"
        let pic = FantasyCpcAmstradCharacterSet::get_char(char);

        //Draw picture pixel by pixel in frame buffer
        for (row_count, _) in pic.iter().enumerate() {
            let row = pic[row_count];
            let mut mask: u8 = 128;

            for col_count in 0..8 {
                let virtual_frame_buffer_pos =
                    frame_x_pos + col_count + (frame_y_pos + row_count) * VIRTUAL_WIDTH;

                if shadowed {
                    let shadow_mask: u8 = if row_count % 2 == 0 {
                        0b10101010
                    } else {
                        0b01010101
                    };
                    match shadow_mask & mask {
                        0 => self.frame[virtual_frame_buffer_pos] = 0,
                        _ => match row & mask {
                            0 => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                            _ => self.frame[virtual_frame_buffer_pos] = text_color,
                        },
                    }
                } else {
                    match row & mask {
                        0 => self.frame[virtual_frame_buffer_pos] = text_bkg_color,
                        _ => self.frame[virtual_frame_buffer_pos] = text_color,
                    }
                }

                mask >>= 1;
            }
        }
    }
}