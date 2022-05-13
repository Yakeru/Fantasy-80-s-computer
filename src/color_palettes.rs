pub fn default_color_palette(color_index: &u8) -> (u8, u8, u8) {

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