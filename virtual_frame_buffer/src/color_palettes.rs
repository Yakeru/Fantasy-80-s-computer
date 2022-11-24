pub const BLACK:            (u8, (u8, u8, u8)) = (0, (0, 0, 0));
pub const DARKBLUE:         (u8, (u8, u8, u8)) = (1, (29, 43, 83));
pub const DARKPURPLE:       (u8, (u8, u8, u8)) = (2, (126, 37, 83));
pub const DARKGREEN:        (u8, (u8, u8, u8)) = (3, (0, 135, 81));
pub const BROWN:            (u8, (u8, u8, u8)) = (4, (171, 82, 54));
pub const DARKGREY:         (u8, (u8, u8, u8)) = (5, (95, 87, 79));
pub const LIGHTGREY:        (u8, (u8, u8, u8)) = (6, (194, 195, 199));
pub const WHITE:            (u8, (u8, u8, u8)) = (7, (255, 241, 232));
pub const RED:              (u8, (u8, u8, u8)) = (8, (255, 0, 77));
pub const ORANGE:           (u8, (u8, u8, u8)) = (9, (255, 163, 0));
pub const YELLOW:           (u8, (u8, u8, u8)) = (10, (255, 236, 39));
pub const GREEN:            (u8, (u8, u8, u8)) = (11, (0, 228, 54));
pub const BLUE:             (u8, (u8, u8, u8)) = (12, (41, 173, 255));
pub const LAVENDER:         (u8, (u8, u8, u8)) = (13, (131, 118, 156));
pub const PINK:             (u8, (u8, u8, u8)) = (14, (255, 119, 168));
pub const LIGHTPEACH:       (u8, (u8, u8, u8)) = (15, (255, 204, 170));
pub const BROWNISHBLACK:    (u8, (u8, u8, u8)) = (16, (41, 24, 20));
pub const DARKERBLUE:       (u8, (u8, u8, u8)) = (17, (17, 29, 53));
pub const DARKERPURPLE:     (u8, (u8, u8, u8)) = (18, (66, 33, 54));
pub const BLUEGREEN:        (u8, (u8, u8, u8)) = (19, (18, 83, 89));
pub const DARKBROWN:        (u8, (u8, u8, u8)) = (20, (116, 47, 41));
pub const DARKERGREY:       (u8, (u8, u8, u8)) = (21, (73, 51, 59));
pub const MEDIUMGREY:       (u8, (u8, u8, u8)) = (22, (162, 136, 121));
pub const LIGHTYELLOW:      (u8, (u8, u8, u8)) = (23, (243, 239, 125));
pub const DARKRED:          (u8, (u8, u8, u8)) = (24, (190, 18, 80));
pub const DARKORANGE:       (u8, (u8, u8, u8)) = (25, (255, 108, 36));
pub const LIMEGREEN:        (u8, (u8, u8, u8)) = (26, (168, 231, 46));
pub const MEDIUMGREEN:      (u8, (u8, u8, u8)) = (27, (0, 181, 67));
pub const TRUEBLUE:         (u8, (u8, u8, u8)) = (28, (6, 90, 181));
pub const MAUVE:            (u8, (u8, u8, u8)) = (29, (117, 70, 101));
pub const DARKPEACH:        (u8, (u8, u8, u8)) = (30, (255, 110, 89));
pub const PEACH:            (u8, (u8, u8, u8)) = (31, (255, 157, 129));

pub const DEFAULT_COLOR_PALETTE: [(u8, u8, u8); 32] = [BLACK.1, DARKBLUE.1, DARKPURPLE.1, DARKGREEN.1, BROWN.1, DARKGREY.1, LIGHTGREY.1, WHITE.1, 
RED.1, ORANGE.1, YELLOW.1, GREEN.1, BLUE.1, LAVENDER.1, PINK.1, LIGHTPEACH.1, 
BROWNISHBLACK.1, DARKERBLUE.1, DARKERPURPLE.1, BLUEGREEN.1, DARKBROWN.1, DARKERGREY.1, MEDIUMGREY.1, LIGHTYELLOW.1, 
DARKRED.1, DARKORANGE.1, LIMEGREEN.1, MEDIUMGREEN.1, TRUEBLUE.1, MAUVE.1, DARKPEACH.1, PEACH.1];

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
