pub const BLACK: u8 = 0;
pub const DARK_BLUE: u8 = 1;
pub const DARK_PURPLE: u8 = 2;
pub const DARK_GREEN: u8 = 3;
pub const BROWN: u8 = 4;
pub const DARK_GREY: u8 = 5;
pub const LIGHT_GREY: u8 = 6;
pub const WHITE: u8 = 7;
pub const RED: u8 = 8;
pub const ORANGE: u8 = 9;
pub const YELLOW: u8 = 10;
pub const GREEN: u8 = 11;
pub const BLUE: u8 = 12;
pub const LAVENDER: u8 = 13;
pub const PINK: u8 = 14;
pub const LIGHT_PEACH: u8 = 15;
pub const BROWNISH_BLACK: u8 = 16;
pub const DARKER_BLUE: u8 = 17;
pub const DARKER_PURPLE: u8 = 18;
pub const BLUE_GREEN: u8 = 19;
pub const DARK_BROWN: u8 = 20;
pub const DARKER_GREY: u8 = 21;
pub const MEDIUM_GREY: u8 = 22;
pub const LIGHT_YELLOW: u8 = 23;
pub const DARK_RED: u8 = 24;
pub const DARK_ORANGE: u8 = 25;
pub const LIME_GREEN: u8 = 26;
pub const MEDIUM_GREEN: u8 = 27;
pub const TRUE_BLUE: u8 = 28;
pub const MAUVE: u8 = 29;
pub const DARK_PEACH: u8 = 30;
pub const PEACH: u8 = 31;

pub const BLACK_RGB: (u8, u8, u8) = (0, 0, 0);
pub const DARK_BLUE_RGB: (u8, u8, u8) = (29, 43, 83);
pub const DARK_PURPLE_RGB: (u8, u8, u8) = (126, 37, 83);
pub const DARK_GREEN_RGB: (u8, u8, u8) = (0, 135, 81);
pub const BROWN_RGB: (u8, u8, u8) = (171, 82, 54);
pub const DARK_GREY_RGB: (u8, u8, u8) = (95, 87, 79);
pub const LIGHT_GREY_RGB: (u8, u8, u8) = (194, 195, 199);
pub const WHITE_RGB: (u8, u8, u8) = (255, 241, 232);
pub const RED_RGB: (u8, u8, u8) = (255, 0, 77);
pub const ORANGE_RGB: (u8, u8, u8) = (255, 163, 0);
pub const YELLOW_RGB: (u8, u8, u8) =  (255, 236, 39);
pub const GREEN_RGB: (u8, u8, u8) =  (0, 228, 54);
pub const BLUE_RGB: (u8, u8, u8) =  (41, 173, 255);
pub const LAVENDER_RGB: (u8, u8, u8) =  (131, 118, 156);
pub const PINK_RGB: (u8, u8, u8) =  (255, 119, 168);
pub const LIGHT_PEACH_RGB: (u8, u8, u8) =  (255, 204, 170);
pub const BROWNISH_BLACK_RGB: (u8, u8, u8) =  (41, 24, 20);
pub const DARKER_BLUE_RGB: (u8, u8, u8) =  (17, 29, 53);
pub const DARKER_PURPLE_RGB: (u8, u8, u8) =  (66, 33, 54);
pub const BLUE_GREEN_RGB: (u8, u8, u8) =  (18, 83, 89);
pub const DARK_BROWN_RGB: (u8, u8, u8) =  (116, 47, 41);
pub const DARKER_GREY_RGB: (u8, u8, u8) =  (73, 51, 59);
pub const MEDIUM_GREY_RGB: (u8, u8, u8) =  (162, 136, 121);
pub const LIGHT_YELLOW_RGB: (u8, u8, u8) =  (243, 239, 125);
pub const DARK_RED_RGB: (u8, u8, u8) =  (190, 18, 80);
pub const DARK_ORANGE_RGB: (u8, u8, u8) =  (255, 108, 36);
pub const LIME_GREEN_RGB: (u8, u8, u8) =  (168, 231, 46);
pub const MEDIUM_GREEN_RGB: (u8, u8, u8) =  (0, 181, 67);
pub const TRUE_BLUE_RGB: (u8, u8, u8) =  (6, 90, 181);
pub const MAUVE_RGB: (u8, u8, u8) =  (117, 70, 101);
pub const DARKPEACH_RGB: (u8, u8, u8) =  (255, 110, 89);
pub const PEACH_RGB: (u8, u8, u8) =  (255, 157, 129);

pub const DEFAULT_COLOR_PALETTE: [(u8, u8, u8); 32] = [BLACK_RGB, DARK_BLUE_RGB, DARK_PURPLE_RGB, DARK_GREEN_RGB, BROWN_RGB, DARK_GREY_RGB, LIGHT_GREY_RGB, WHITE_RGB, 
RED_RGB, ORANGE_RGB, YELLOW_RGB, GREEN_RGB, BLUE_RGB, LAVENDER_RGB, PINK_RGB, LIGHT_PEACH_RGB, 
BROWNISH_BLACK_RGB, DARKER_BLUE_RGB, DARKER_PURPLE_RGB, BLUE_GREEN_RGB, DARK_BROWN_RGB, DARKER_GREY_RGB, MEDIUM_GREY_RGB, LIGHT_YELLOW_RGB, 
DARK_RED_RGB, DARK_ORANGE_RGB, LIME_GREEN_RGB, MEDIUM_GREEN_RGB, TRUE_BLUE_RGB, MAUVE_RGB, DARKPEACH_RGB, PEACH_RGB];

pub struct ColorPalette {
    custom: [(u8, u8, u8); 32],
    toggle_custom: bool,
}

impl ColorPalette {
    pub fn toggle_custom(&mut self) {
        self.toggle_custom = !self.toggle_custom
    }

    pub fn set_custom_palette_color(&mut self, index: usize, rgb_color: (u8, u8, u8)) {
        self.custom[index] = rgb_color
    }

    pub fn set_custom_palette(&mut self, rgb_palette: [(u8, u8, u8); 32]) {
        self.custom = rgb_palette
    }

    pub fn get_rgb(&self, index: u8) -> (u8, u8, u8) {
        if self.toggle_custom {
            self.custom[index as usize]
        } else {
            DEFAULT_COLOR_PALETTE[index as usize]
        }
    }
}

pub const DEFAULT_PALETTE: ColorPalette = ColorPalette{custom: DEFAULT_COLOR_PALETTE, toggle_custom: false};