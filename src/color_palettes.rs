#[derive(Copy, Clone)]
pub enum ColorPalette {
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
    Custom0,
    Custom1,
    Custom2,
    Custom3,
    Custom4,
    Custom5,
    Custom6,
    Custom7,
}

pub fn get_rbg(color_name: &ColorPalette) -> (u8, u8, u8) {
    match color_name {
        ColorPalette::Black  => {return (0, 0, 0)} //black
        ColorPalette::DarkBlue  => {return (29, 43, 83)} //dark-blue
        ColorPalette::DarkerPurple  => {return (126, 37, 83)} //dark-purple
        ColorPalette::DarkGreen  => {return (0, 135, 81)} //dark-green
        ColorPalette::Brown  => {return (171, 82, 54)} //brown
        ColorPalette::DarkGrey  => {return (95, 87, 79)} //dark-grey
        ColorPalette::LightGrey  => {return (194, 195, 199)} //light-grey
        ColorPalette::White  => {return (255, 241, 232)} //white
        ColorPalette::Red => {return (255, 0, 77)} //red
        ColorPalette::Orange  => {return (255, 163, 0)} //orange
        ColorPalette::Yellow  => {return (255, 236, 39)} //yellow
        ColorPalette::Green  => {return (0, 228, 54)} //green
        ColorPalette::Blue  => {return (41, 173, 255)} //blue
        ColorPalette::Lavender  => {return (131, 118, 156)} //lavender
        ColorPalette::Pink  => {return (255, 119, 168)} //pink
        ColorPalette::LightPeach  => {return (255, 204, 170)} //light-peach
        ColorPalette::BrownishBlack  => {return (41,24,20)} //brownish-black
        ColorPalette::DarkerBlue  => {return (17,29,53 )} //darker-blue
        ColorPalette::DarkerPurple  => {return (66,33,54)} //darker-purple
        ColorPalette::BlueGreen  => {return (18,83,89)} //blue-green
        ColorPalette::DarkBrown  => {return (116,47,41)} //dark-brown
        ColorPalette::DarkerGrey  => {return (73,51,59)} //darker-grey
        ColorPalette::MediumGrey  => {return (162,136,121)} //medium-grey
        ColorPalette::LightYellow  => {return (243,239,125)} //light-yellow
        ColorPalette::DarkRed  => {return (190,18,80)} //dark-red
        ColorPalette::DarkOrange  => {return (255,108,36)} //dark-orange
        ColorPalette::LimeGreen  => {return (168,231,46)} //lime-green
        ColorPalette::MediumGreen  => {return (0,181,67)} //medium-green
        ColorPalette::TrueBlue  => {return (6,90,181)} //true-blue
        ColorPalette::Mauve  => {return (117,70,101)} //mauve
        ColorPalette::DarkPeach  => {return (255,110,89)} //dark-peach
        ColorPalette::Peach  => {return (255,157,129)} //peach 
        _ => {return (0, 0, 0)}
    }
}

pub fn get_rgb(color_index: &u8) -> (u8, u8, u8) {

    match color_index {
        0 => {return (0, 0, 0)} //black
        1 => {return (29, 43, 83)} //dark-blue
        2 => {return (126, 37, 83)} //dark-purple
        3 => {return (0, 135, 81)} //dark-green
        4 => {return (171, 82, 54)} //brown
        5 => {return (95, 87, 79)} //dark-grey
        6 => {return (194, 195, 199)} //light-grey
        7 => {return (255, 241, 232)} //white
        8 => {return (255, 0, 77)} //red
        9 => {return (255, 163, 0)} //orange
        10 => {return (255, 236, 39)} //yellow
        11 => {return (0, 228, 54)} //green
        12 => {return (41, 173, 255)} //blue
        13 => {return (131, 118, 156)} //lavender
        14 => {return (255, 119, 168)} //pink
        15 => {return (255, 204, 170)} //light-peach
        16 => {return (41,24,20)} //brownish-black
        17 => {return (17,29,53 )} //darker-blue
        18 => {return (66,33,54)} //darker-purple
        19 => {return (18,83,89)} //blue-green
        20 => {return (116,47,41)} //dark-brown
        21 => {return (73,51,59)} //darker-grey
        22 => {return (162,136,121)} //medium-grey
        23 => {return (243,239,125)} //light-yellow
        24 => {return (190,18,80)} //dark-red
        25 => {return (255,108,36)} //dark-orange
        26 => {return (168,231,46)} //lime-green
        27 => {return (0,181,67)} //medium-green
        28 => {return (6,90,181)} //true-blue
        29 => {return (117,70,101)} //mauve
        30 => {return (255,110,89)} //dark-peach
        31 => {return (255,157,129)} //peach 
        _ => {return (0, 0, 0)}
    }
}

pub fn get_index(color_name: &ColorPalette) -> u8 {
    match color_name {
        ColorPalette::Black  => {0} //black
        ColorPalette::DarkBlue  => {1} //dark-blue
        ColorPalette::DarkerPurple  => {2} //dark-purple
        ColorPalette::DarkGreen  => {3} //dark-green
        ColorPalette::Brown  => {4} //brown
        ColorPalette::DarkGrey  => {5} //dark-grey
        ColorPalette::LightGrey  => {6} //light-grey
        ColorPalette::White  => {7} //white
        ColorPalette::Red => {8} //red
        ColorPalette::Orange  => {9} //orange
        ColorPalette::Yellow  => {10} //yellow
        ColorPalette::Green  => {11} //green
        ColorPalette::Blue  => {12} //blue
        ColorPalette::Lavender  => {13} //lavender
        ColorPalette::Pink  => {14} //pink
        ColorPalette::LightPeach  => {15} //light-peach
        ColorPalette::BrownishBlack  => {16} //brownish-black
        ColorPalette::DarkerBlue  => {17} //darker-blue
        ColorPalette::DarkerPurple  => {18} //darker-purple
        ColorPalette::BlueGreen  => {19} //blue-green
        ColorPalette::DarkBrown  => {20} //dark-brown
        ColorPalette::DarkerGrey  => {21} //darker-grey
        ColorPalette::MediumGrey  => {22} //medium-grey
        ColorPalette::LightYellow  => {23} //light-yellow
        ColorPalette::DarkRed  => {24} //dark-red
        ColorPalette::DarkOrange  => {25} //dark-orange
        ColorPalette::LimeGreen  => {26} //lime-green
        ColorPalette::MediumGreen  => {27} //medium-green
        ColorPalette::TrueBlue  => {28} //true-blue
        ColorPalette::Mauve  => {29} //mauve
        ColorPalette::DarkPeach  => {30} //dark-peach
        ColorPalette::Peach  => {31} //peach 
        _ => {0}
    }
}