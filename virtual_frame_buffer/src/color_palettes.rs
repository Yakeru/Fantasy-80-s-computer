pub enum Color {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGrey,
    LightGrey,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Lavender,
    Pink,
    LightPeach,
    BrownishBlack,
    DarkerBlue,
    DarkerPurple,
    BlueGreen,
    DarkBrown,
    DarkerGrey,
    MediumGrey,
    LightYellow,
    DarkRed,
    DarkOrange,
    LimeGreen,
    MediumGreen,
    TrueBlue,
    Mauve,
    DarkPeach,
    Peach,
}

pub struct ColorPalette {
    default: [(u8, u8, u8); 32],
    custom: [(u8, u8, u8); 32],
    custom_active: bool,
}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        ColorPalette {
            default: [
                (0, 0, 0),
                (29, 43, 83),
                (126, 37, 83),
                (0, 135, 81),
                (171, 82, 54),
                (95, 87, 79),
                (194, 195, 199),
                (255, 241, 232),
                (255, 0, 77),
                (255, 163, 0),
                (255, 236, 39),
                (0, 228, 54),
                (41, 173, 255),
                (131, 118, 156),
                (255, 119, 168),
                (255, 204, 170),
                (41, 24, 20),
                (17, 29, 53),
                (66, 33, 54),
                (18, 83, 89),
                (116, 47, 41),
                (73, 51, 59),
                (162, 136, 121),
                (243, 239, 125),
                (190, 18, 80),
                (255, 108, 36),
                (168, 231, 46),
                (0, 181, 67),
                (6, 90, 181),
                (117, 70, 101),
                (255, 110, 89),
                (255, 157, 129),
            ],
            custom: [
                (0, 0, 0),
                (29, 43, 83),
                (126, 37, 83),
                (0, 135, 81),
                (171, 82, 54),
                (95, 87, 79),
                (194, 195, 199),
                (255, 241, 232),
                (255, 0, 77),
                (255, 163, 0),
                (255, 236, 39),
                (0, 228, 54),
                (41, 173, 255),
                (131, 118, 156),
                (255, 119, 168),
                (255, 204, 170),
                (41, 24, 20),
                (17, 29, 53),
                (66, 33, 54),
                (18, 83, 89),
                (116, 47, 41),
                (73, 51, 59),
                (162, 136, 121),
                (243, 239, 125),
                (190, 18, 80),
                (255, 108, 36),
                (168, 231, 46),
                (0, 181, 67),
                (6, 90, 181),
                (117, 70, 101),
                (255, 110, 89),
                (255, 157, 129),
            ],
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

    pub fn get_rgb_from_index(&self, color_index: u8) -> (u8, u8, u8) {
        if color_index > 31 {
            return self.default[0];
        }

        if self.custom_active {
            return self.custom[color_index as usize];
        }

        self.default[color_index as usize]
    }

    pub fn get_rgb_from_name(&self, color_name: Color) -> (u8, u8, u8) {
        match color_name {
            Color::Black => self.default[0],
            Color::DarkBlue => self.default[1],
            Color::DarkPurple => self.default[2],
            Color::DarkGreen => self.default[3],
            Color::Brown => self.default[4],
            Color::DarkGrey => self.default[5],
            Color::LightGrey => self.default[6],
            Color::White => self.default[7],
            Color::Red => self.default[8],
            Color::Orange => self.default[9],
            Color::Yellow => self.default[10],
            Color::Green => self.default[11],
            Color::Blue => self.default[12],
            Color::Lavender => self.default[13],
            Color::Pink => self.default[14],
            Color::LightPeach => self.default[15],
            Color::BrownishBlack => self.default[16],
            Color::DarkerBlue => self.default[17],
            Color::DarkerPurple => self.default[18],
            Color::BlueGreen => self.default[19],
            Color::DarkBrown => self.default[20],
            Color::DarkerGrey => self.default[21],
            Color::MediumGrey => self.default[22],
            Color::LightYellow => self.default[23],
            Color::DarkRed => self.default[24],
            Color::DarkOrange => self.default[25],
            Color::LimeGreen => self.default[26],
            Color::MediumGreen => self.default[27],
            Color::TrueBlue => self.default[28],
            Color::Mauve => self.default[29],
            Color::DarkPeach => self.default[30],
            Color::Peach => self.default[31],
        }
    }
}
