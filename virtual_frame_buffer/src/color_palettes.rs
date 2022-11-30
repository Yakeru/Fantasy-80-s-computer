pub const BLACK:            u8 = 0;
pub const DARKBLUE:         u8 = 1;
pub const DARKPURPLE:       u8 = 2;
pub const DARKGREEN:        u8 = 3;
pub const BROWN:            u8 = 4;
pub const DARKGREY:         u8 = 5;
pub const LIGHTGREY:        u8 = 6;
pub const WHITE:            u8 = 7;
pub const RED:              u8 = 8;
pub const ORANGE:           u8 = 9;
pub const YELLOW:           u8 = 10;
pub const GREEN:            u8 = 11;
pub const BLUE:             u8 = 12;
pub const LAVENDER:         u8 = 13;
pub const PINK:             u8 = 14;
pub const LIGHTPEACH:       u8 = 15;
pub const BROWNISHBLACK:    u8 = 16;
pub const DARKERBLUE:       u8 = 17;
pub const DARKERPURPLE:     u8 = 18;
pub const BLUEGREEN:        u8 = 19;
pub const DARKBROWN:        u8 = 20;
pub const DARKERGREY:       u8 = 21;
pub const MEDIUMGREY:       u8 = 22;
pub const LIGHTYELLOW:      u8 = 23;
pub const DARKRED:          u8 = 24;
pub const DARKORANGE:       u8 = 25;
pub const LIMEGREEN:        u8 = 26;
pub const MEDIUMGREEN:      u8 = 27;
pub const TRUEBLUE:         u8 = 28;
pub const MAUVE:            u8 = 29;
pub const DARKPEACH:        u8 = 30;
pub const PEACH:            u8 = 31;

pub const BLACK_RGB:            (u8, u8, u8) = (0, 0, 0);
pub const DARKBLUE_RGB:         (u8, u8, u8) = (29, 43, 83);
pub const DARKPURPLE_RGB:       (u8, u8, u8) = (126, 37, 83);
pub const DARKGREEN_RGB:        (u8, u8, u8) = (0, 135, 81);
pub const BROWN_RGB:            (u8, u8, u8) = (171, 82, 54);
pub const DARKGREY_RGB:         (u8, u8, u8) = (95, 87, 79);
pub const LIGHTGREY_RGB:        (u8, u8, u8) = (194, 195, 199);
pub const WHITE_RGB:            (u8, u8, u8) = (255, 241, 232);
pub const RED_RGB:              (u8, u8, u8) = (255, 0, 77);
pub const ORANGE_RGB:           (u8, u8, u8) = (255, 163, 0);
pub const YELLOW_RGB:           (u8, u8, u8) =  (255, 236, 39);
pub const GREEN_RGB:            (u8, u8, u8) =  (0, 228, 54);
pub const BLUE_RGB:             (u8, u8, u8) =  (41, 173, 255);
pub const LAVENDER_RGB:         (u8, u8, u8) =  (131, 118, 156);
pub const PINK_RGB:             (u8, u8, u8) =  (255, 119, 168);
pub const LIGHTPEACH_RGB:       (u8, u8, u8) =  (255, 204, 170);
pub const BROWNISHBLACK_RGB:    (u8, u8, u8) =  (41, 24, 20);
pub const DARKERBLUE_RGB:       (u8, u8, u8) =  (17, 29, 53);
pub const DARKERPURPLE_RGB:     (u8, u8, u8) =  (66, 33, 54);
pub const BLUEGREEN_RGB:        (u8, u8, u8) =  (18, 83, 89);
pub const DARKBROWN_RGB:        (u8, u8, u8) =  (116, 47, 41);
pub const DARKERGREY_RGB:       (u8, u8, u8) =  (73, 51, 59);
pub const MEDIUMGREY_RGB:       (u8, u8, u8) =  (162, 136, 121);
pub const LIGHTYELLOW_RGB:      (u8, u8, u8) =  (243, 239, 125);
pub const DARKRED_RGB:          (u8, u8, u8) =  (190, 18, 80);
pub const DARKORANGE_RGB:       (u8, u8, u8) =  (255, 108, 36);
pub const LIMEGREEN_RGB:        (u8, u8, u8) =  (168, 231, 46);
pub const MEDIUMGREEN_RGB:      (u8, u8, u8) =  (0, 181, 67);
pub const TRUEBLUE_RGB:         (u8, u8, u8) =  (6, 90, 181);
pub const MAUVE_RGB:            (u8, u8, u8) =  (117, 70, 101);
pub const DARKPEACH_RGB:        (u8, u8, u8) =  (255, 110, 89);
pub const PEACH_RGB:            (u8, u8, u8) =  (255, 157, 129);

pub const DEFAULT_COLOR_PALETTE: [(u8, u8, u8); 32] = [BLACK_RGB, DARKBLUE_RGB, DARKPURPLE_RGB, DARKGREEN_RGB, BROWN_RGB, DARKGREY_RGB, LIGHTGREY_RGB, WHITE_RGB, 
RED_RGB, ORANGE_RGB, YELLOW_RGB, GREEN_RGB, BLUE_RGB, LAVENDER_RGB, PINK_RGB, LIGHTPEACH_RGB, 
BROWNISHBLACK_RGB, DARKERBLUE_RGB, DARKERPURPLE_RGB, BLUEGREEN_RGB, DARKBROWN_RGB, DARKERGREY_RGB, MEDIUMGREY_RGB, LIGHTYELLOW_RGB, 
DARKRED_RGB, DARKORANGE_RGB, LIMEGREEN_RGB, MEDIUMGREEN_RGB, TRUEBLUE_RGB, MAUVE_RGB, DARKPEACH_RGB, PEACH_RGB];

pub struct ColorPalette {
    custom: [(u8, u8, u8); 32],
    custom_active: bool,
}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        ColorPalette {
            custom: DEFAULT_COLOR_PALETTE,
            custom_active: false,
        }
    }

    pub fn use_custom_palette(&mut self) {
        self.custom_active = true
    }

    pub fn use_default_palette(&mut self) {
        self.custom_active = false
    }

    pub fn set_custom_palette_color(&mut self, index: usize, rgb_color: (u8, u8, u8)) {
        self.custom[index] = rgb_color
    }

    pub fn set_custom_palette(&mut self, rgb_palette: [(u8, u8, u8); 32]) {
        self.custom = rgb_palette
    }

    pub fn get_rgb(&self, index: u8) -> (u8, u8, u8) {
        if self.custom_active {
            self.custom[index as usize]
        } else {
            DEFAULT_COLOR_PALETTE[index as usize]
        }
    }
}
