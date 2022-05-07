pub fn default_color_palette(color_index: &u8) -> (u8, u8, u8) {

    match color_index {
        0 => {return (0, 0, 0)},
        1 => {return (254, 254, 254)},
        2 => {return (254, 0, 0)},
        3 => {return (0, 254, 0)},
        4 => {return (0, 0, 254)},
        5 => {return (254, 254, 0)},
        6 => {return (0, 254, 254)},
        7 => {return (254, 0, 254)},
        8.. => {return (0, 0, 0)}
    }
}