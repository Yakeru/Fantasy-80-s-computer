use rand::Rng;
use crate::{DisplayController, color_palettes::PALETE_SIZE, text_layer::{character_rom_trait::FantasyCpc8by8CharacterRomTrait, default_character_rom::FantasyCpcAmstradCharacterSet, text_layer::TextLayerChar}};

impl DisplayController {
    pub fn genrate_random_garbage(&mut self) {
        let mut random = rand::thread_rng();

        let rnd_clear_color: usize = random.gen_range(0..32);
        self.clear(rnd_clear_color);
        self.get_text_layer_mut().clear();

        let char_map = self.get_text_layer_mut().get_char_map_mut();
        for index in 0..char_map.len() {
            let mut color: usize = random.gen_range(0..(PALETE_SIZE + 10)); //To get a bit more black
            color = if color > PALETE_SIZE - 1 { 0 } else { color };

            let mut bkg_color: usize = random.gen_range(0..(PALETE_SIZE + 10));
            bkg_color = if bkg_color > PALETE_SIZE - 1 {
                0
            } else {
                bkg_color
            };

            let mut char_index = random.gen_range(0..100);
            char_index = if char_index > FantasyCpcAmstradCharacterSet::get_char_table().len() - 1 {
                0
            } else {
                char_index
            };
            let c: char = FantasyCpcAmstradCharacterSet::get_char_table().chars().nth(char_index).expect("oups");

            let effect: u8 = random.gen_range(0..10);
            let swap: bool = effect & 0b00000001 > 0;
            let blink: bool = effect & 0b00000010 > 0;
            let shadowed: bool = effect & 0b00000100 > 0;

            let text_layer_char: TextLayerChar = TextLayerChar {
                c,
                color,
                bkg_color,
                swap,
                blink,
                shadowed,
            };
            char_map[index] = Some(text_layer_char);
        }
    }
}