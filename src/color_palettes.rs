// #[derive(Copy, Clone)]
// pub enum Color {
//     Black(u8,u8,u8),
//     DarkBlue(u8,u8,u8),
//     DarkPurple(u8,u8,u8),
//     DarkGreen(u8,u8,u8),
//     Brown(u8,u8,u8),
//     DarkGrey(u8,u8,u8),
//     LightGrey(u8,u8,u8),
//     White(u8,u8,u8),
//     Red(u8,u8,u8),
//     Orange(u8,u8,u8),
//     Yellow(u8,u8,u8),
//     Green(u8,u8,u8),
//     Blue(u8,u8,u8),
//     Lavender(u8,u8,u8),
//     Pink(u8,u8,u8),
//     LightPeach(u8,u8,u8),
//     BrownishBlack(u8,u8,u8),
//     DarkerBlue(u8,u8,u8),
//     DarkerPurple(u8,u8,u8),
//     BlueGreen(u8,u8,u8),
//     DarkBrown(u8,u8,u8),
//     DarkerGrey(u8,u8,u8),
//     MediumGrey(u8,u8,u8),
//     LightYellow(u8,u8,u8),
//     DarkRed(u8,u8,u8),
//     DarkOrange(u8,u8,u8),
//     LimeGreen(u8,u8,u8),
//     MediumGreen(u8,u8,u8),
//     TrueBlue(u8,u8,u8),
//     Mauve(u8,u8,u8),
//     DarkPeach(u8,u8,u8),
//     Peach(u8,u8,u8)
// }

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

// const default_color_palette: [Color;32] = [
//     Color::Black(0, 0, 0),  //black
//     Color::DarkBlue(29, 43, 83),  //dark-blue
//     Color::DarkerPurple(126, 37, 83),  //dark-purple
//     Color::DarkGreen(0, 135, 81),  //dark-green
//     Color::Brown(0, 0, 0),  //brown
//     Color::DarkGrey(0, 0, 0),  //dark-grey
//     Color::LightGrey(0, 0, 0),  //light-grey
//     Color::White(0, 0, 0),  //white
//     Color::Red(0, 0, 0), //red
//     Color::Orange(0, 0, 0),  //orange
//     Color::Yellow(0, 0, 0),  //yellow
//     Color::Green(0, 0, 0),  //green
//     Color::Blue(0, 0, 0),  //blue
//     Color::Lavender(0, 0, 0),  //lavender
//     Color::Pink(0, 0, 0),  //pink
//     Color::LightPeach(0, 0, 0),  //light-peach
//     Color::BrownishBlack(0, 0, 0),  //brownish-black
//     Color::DarkerBlue(0, 0, 0),  //darker-blue
//     Color::DarkerPurple(0, 0, 0),  //darker-purple
//     Color::BlueGreen(0, 0, 0),  //blue-green
//     Color::DarkBrown(0, 0, 0),  //dark-brown
//     Color::DarkerGrey(0, 0, 0),  //darker-grey
//     Color::MediumGrey(0, 0, 0),  //medium-grey
//     Color::LightYellow(0, 0, 0),  //light-yellow
//     Color::DarkRed(0, 0, 0),  //dark-red
//     Color::DarkOrange(0, 0, 0),  //dark-orange
//     Color::LimeGreen(0, 0, 0),  //lime-green
//     Color::MediumGreen(0, 0, 0),  //medium-green
//     Color::TrueBlue(0, 0, 0),  //true-blue
//     Color::Mauve(0, 0, 0),  //mauve
//     Color::DarkPeach(0, 0, 0),  //dark-peach
//     Color::Peach(0, 0, 0),  //peach 
// ];

//const custom_color_palette: [Colors;3] = [Colors::Black(0,0,0), Colors::Black(0,0,0), Colors::Black(0,0,0)];


// pub fn get_color_index(color_name: &ColorPalette) -> u8 {
//     match color_name {
//         ColorPalette::Black  => {0} //black
//         ColorPalette::DarkBlue  => {1} //dark-blue
//         ColorPalette::DarkerPurple  => {2} //dark-purple
//         ColorPalette::DarkGreen  => {3} //dark-green
//         ColorPalette::Brown  => {4} //brown
//         ColorPalette::DarkGrey  => {5} //dark-grey
//         ColorPalette::LightGrey  => {6} //light-grey
//         ColorPalette::White  => {7} //white
//         ColorPalette::Red => {8} //red
//         ColorPalette::Orange  => {9} //orange
//         ColorPalette::Yellow  => {10} //yellow
//         ColorPalette::Green  => {11} //green
//         ColorPalette::Blue  => {12} //blue
//         ColorPalette::Lavender  => {13} //lavender
//         ColorPalette::Pink  => {14} //pink
//         ColorPalette::LightPeach  => {15} //light-peach
//         ColorPalette::BrownishBlack  => {16} //brownish-black
//         ColorPalette::DarkerBlue  => {17} //darker-blue
//         ColorPalette::DarkerPurple  => {18} //darker-purple
//         ColorPalette::BlueGreen  => {19} //blue-green
//         ColorPalette::DarkBrown  => {21} //dark-brown
//         ColorPalette::DarkerGrey  => {21} //darker-grey
//         ColorPalette::MediumGrey  => {22} //medium-grey
//         ColorPalette::LightYellow  => {23} //light-yellow
//         ColorPalette::DarkRed  => {24} //dark-red
//         ColorPalette::DarkOrange  => {25} //dark-orange
//         ColorPalette::LimeGreen  => {26} //lime-green
//         ColorPalette::MediumGreen  => {27} //medium-green
//         ColorPalette::TrueBlue  => {28} //true-blue
//         ColorPalette::Mauve  => {29} //mauve
//         ColorPalette::DarkPeach  => {30} //dark-peach
//         ColorPalette::Peach  => {31} //peach 
//         _ => {0}
//     }
// }