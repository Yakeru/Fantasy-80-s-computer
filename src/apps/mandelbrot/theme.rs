use display_controller::color_palettes::*;

use super::config::EMPTY_RATIO_TRIGGER;

pub struct ColorTheme {
    pub palette1: Vec<u8>,
    pub palette2: Vec<u8>,
    pub palette_swap: bool,
    pub empty_color: u8,
    pub empty_ratio: f64,
    pub fuzzyness: f64,
}

impl ColorTheme {
    pub fn get_palette_1(&mut self) -> &mut Vec<u8> {
        &mut self.palette1
    }

    pub fn get_palette_2(&mut self) -> &mut Vec<u8> {
        &mut self.palette2
    }
}

pub fn get_themes() -> Vec<ColorTheme> {
    let warm = ColorTheme {
        palette1: vec![
            BROWNISH_BLACK,
            DARK_BROWN,
            BROWN,
            DARK_RED,
            RED,
            DARK_ORANGE,
            ORANGE,
            YELLOW,
            LIGHT_PEACH,
            WHITE,
        ],
        palette2: Vec::new(),
        palette_swap: false,
        empty_color: BLACK,
        empty_ratio: EMPTY_RATIO_TRIGGER,
        fuzzyness: 250.0,
    };

    // Cool theme
    let cool = ColorTheme {
        palette1: [
            DARK_PURPLE,
            DARKER_PURPLE,
            DARKER_BLUE,
            DARK_BLUE,
            TRUE_BLUE,
            BLUE,
            WHITE,
            LAVENDER,
            MAUVE,
        ]
        .to_vec(),
        palette2: Vec::new(),
        palette_swap: false,
        empty_color: WHITE,
        empty_ratio: EMPTY_RATIO_TRIGGER,
        fuzzyness: 0.0,
    };

    // Tree theme
    let tree = ColorTheme {
        palette1: [
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_GREEN,
            MEDIUM_GREEN,
            GREEN,
            LIME_GREEN,
        ]
        .to_vec(),
        palette2: [
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_BROWN,
            BROWN,
            MEDIUM_GREEN,
            GREEN,
            LIME_GREEN,
            GREEN,
        ]
        .to_vec(),
        palette_swap: true,
        empty_color: TRUE_BLUE,
        empty_ratio: EMPTY_RATIO_TRIGGER * 2.0,
        fuzzyness: 0.0,
    };

    // Canyon theme
    let canyon = ColorTheme {
        palette1: [
            DARK_BROWN,
            BROWN,
            DARK_ORANGE,
            ORANGE,
            DARK_ORANGE,
            BROWN,
            DARK_BROWN,
            BROWN,
            DARK_ORANGE,
            ORANGE,
            DARK_ORANGE,
            BROWN,
            DARK_BROWN,
            BLACK,
        ]
        .to_vec(),
        palette2: Vec::new(),
        palette_swap: false,
        empty_color: TRUE_BLUE,
        empty_ratio: EMPTY_RATIO_TRIGGER,
        fuzzyness: 0.0,
    };

    // Burton theme
    let burton = ColorTheme {
        palette1: [
            BLACK, BLACK, BLACK, BLACK, RED, BLACK, BLACK, BLACK, BLACK, WHITE,
        ]
        .to_vec(),
        palette2: [
            BLACK, BLACK, BLACK, BLACK, DARK_RED, BLACK, BLACK, BLACK, BLACK, LIGHT_GREY,
        ]
        .to_vec(),
        palette_swap: true,
        empty_color: BLACK,
        empty_ratio: EMPTY_RATIO_TRIGGER * 2.0,
        fuzzyness: 0.0,
    };

    //B&W
    let bw = ColorTheme {
        palette1: [
            BLACK,
            DARKER_GREY,
            DARK_GREY,
            MEDIUM_GREY,
            LIGHT_GREY,
            WHITE,
            LIGHT_GREY,
            MEDIUM_GREY,
            DARK_GREY,
            DARKER_GREY,
        ]
        .to_vec(),
        palette2: [
            BLACK, BLACK, BLACK, BLACK, DARK_RED, BLACK, BLACK, BLACK, BLACK, LIGHT_GREY,
        ]
        .to_vec(),
        palette_swap: false,
        empty_color: DARK_PURPLE,
        empty_ratio: EMPTY_RATIO_TRIGGER,
        fuzzyness: 0.0,
    };

    //Candy
    let candy = ColorTheme {
        palette1: [RED, WHITE, GREEN, WHITE, TRUE_BLUE, WHITE].to_vec(),
        palette2: Vec::new(),
        palette_swap: false,
        empty_color: BLACK,
        empty_ratio: EMPTY_RATIO_TRIGGER,
        fuzzyness: 0.0,
    };

    vec![warm, cool, tree, canyon, burton, bw, candy]
}
