pub const PALETE_SIZE: usize = 32;

pub const BLACK: usize = 0;
pub const DARK_BLUE: usize = 1;
pub const DARK_PURPLE: usize = 2;
pub const DARK_GREEN: usize = 3;
pub const BROWN: usize = 4;
pub const DARK_GREY: usize = 5;
pub const LIGHT_GREY: usize = 6;
pub const WHITE: usize = 7;
pub const RED: usize = 8;
pub const ORANGE: usize = 9;
pub const YELLOW: usize = 10;
pub const GREEN: usize = 11;
pub const BLUE: usize = 12;
pub const LAVENDER: usize = 13;
pub const PINK: usize = 14;
pub const LIGHT_PEACH: usize = 15;
pub const BROWNISH_BLACK: usize = 16;
pub const DARKER_BLUE: usize = 17;
pub const DARKER_PURPLE: usize = 18;
pub const BLUE_GREEN: usize = 19;
pub const DARK_BROWN: usize = 20;
pub const DARKER_GREY: usize = 21;
pub const MEDIUM_GREY: usize = 22;
pub const LIGHT_YELLOW: usize = 23;
pub const DARK_RED: usize = 24;
pub const DARK_ORANGE: usize = 25;
pub const LIME_GREEN: usize = 26;
pub const MEDIUM_GREEN: usize = 27;
pub const TRUE_BLUE: usize = 28;
pub const MAUVE: usize = 29;
pub const DARK_PEACH: usize = 30;
pub const PEACH: usize = 31;

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
pub const YELLOW_RGB: (u8, u8, u8) = (255, 236, 39);
pub const GREEN_RGB: (u8, u8, u8) = (0, 228, 54);
pub const BLUE_RGB: (u8, u8, u8) = (41, 173, 255);
pub const LAVENDER_RGB: (u8, u8, u8) = (131, 118, 156);
pub const PINK_RGB: (u8, u8, u8) = (255, 119, 168);
pub const LIGHT_PEACH_RGB: (u8, u8, u8) = (255, 204, 170);
pub const BROWNISH_BLACK_RGB: (u8, u8, u8) = (41, 24, 20);
pub const DARKER_BLUE_RGB: (u8, u8, u8) = (17, 29, 53);
pub const DARKER_PURPLE_RGB: (u8, u8, u8) = (66, 33, 54);
pub const BLUE_GREEN_RGB: (u8, u8, u8) = (18, 83, 89);
pub const DARK_BROWN_RGB: (u8, u8, u8) = (116, 47, 41);
pub const DARKER_GREY_RGB: (u8, u8, u8) = (73, 51, 59);
pub const MEDIUM_GREY_RGB: (u8, u8, u8) = (162, 136, 121);
pub const LIGHT_YELLOW_RGB: (u8, u8, u8) = (243, 239, 125);
pub const DARK_RED_RGB: (u8, u8, u8) = (190, 18, 80);
pub const DARK_ORANGE_RGB: (u8, u8, u8) = (255, 108, 36);
pub const LIME_GREEN_RGB: (u8, u8, u8) = (168, 231, 46);
pub const MEDIUM_GREEN_RGB: (u8, u8, u8) = (0, 181, 67);
pub const TRUE_BLUE_RGB: (u8, u8, u8) = (6, 90, 181);
pub const MAUVE_RGB: (u8, u8, u8) = (117, 70, 101);
pub const DARKPEACH_RGB: (u8, u8, u8) = (255, 110, 89);
pub const PEACH_RGB: (u8, u8, u8) = (255, 157, 129);

pub static mut COLOR_PALETTE: [(u8, u8, u8); PALETE_SIZE] = [
    BLACK_RGB,
    DARK_BLUE_RGB,
    DARK_PURPLE_RGB,
    DARK_GREEN_RGB,
    BROWN_RGB,
    DARK_GREY_RGB,
    LIGHT_GREY_RGB,
    WHITE_RGB,
    RED_RGB,
    ORANGE_RGB,
    YELLOW_RGB,
    GREEN_RGB,
    BLUE_RGB,
    LAVENDER_RGB,
    PINK_RGB,
    LIGHT_PEACH_RGB,
    BROWNISH_BLACK_RGB,
    DARKER_BLUE_RGB,
    DARKER_PURPLE_RGB,
    BLUE_GREEN_RGB,
    DARK_BROWN_RGB,
    DARKER_GREY_RGB,
    MEDIUM_GREY_RGB,
    LIGHT_YELLOW_RGB,
    DARK_RED_RGB,
    DARK_ORANGE_RGB,
    LIME_GREEN_RGB,
    MEDIUM_GREEN_RGB,
    TRUE_BLUE_RGB,
    MAUVE_RGB,
    DARKPEACH_RGB,
    PEACH_RGB,
];
