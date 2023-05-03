use display_controller::color_palettes::LIGHT_GREY;

pub const STONE_64X64: [u8; 4096] = [6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,5,5,5,5,5,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,6,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,7,0,0,0,7,7,5,5,5,0,0,0,0,6,6,6,6,6,6,5,5,5,5,5,5,6,5,5,5,5,5,5,0,0,0,0,0,0,0,6,6,6,6,6,6,6,5,5,5,5,6,0,0,0,0,6,0,0,6,5,0,0,0,7,7,6,6,6,7,7,7,6,5,5,5,5,5,6,6,6,6,6,5,0,0,0,0,0,7,7,7,7,7,7,7,6,5,5,5,5,5,5,0,0,6,6,5,0,0,5,5,6,5,5,5,5,5,0,0,0,5,5,5,6,6,7,7,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,0,0,0,0,6,6,7,7,7,7,7,5,5,5,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,6,6,6,5,6,0,0,0,0,6,6,0,0,0,0,0,0,0,0,6,6,6,6,0,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,0,0,0,6,5,6,6,6,6,6,6,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,0,0,0,0,5,5,5,5,5,0,0,0,0,0,0,6,6,6,6,6,6,6,5,6,6,5,5,6,6,6,0,0,0,5,5,5,5,5,5,6,0,0,0,5,6,6,6,6,6,7,7,7,7,7,7,7,7,6,6,6,6,6,6,5,5,0,0,0,5,6,6,0,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,5,5,5,6,0,5,0,0,0,0,0,0,5,5,0,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,5,6,6,6,6,0,0,0,0,0,0,0,0,7,7,7,7,7,7,6,6,6,6,6,6,5,5,5,5,6,6,0,0,7,7,6,6,5,5,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,6,6,6,6,6,6,6,6,6,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,7,7,6,6,6,6,6,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,6,5,5,5,5,5,5,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,6,5,5,5,5,6,5,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,0,0,0,5,5,5,5,5,5,5,5,5,5,0,0,5,5,5,5,7,7,7,7,7,7,5,5,5,5,5,5,5,0,0,5,5,5,0,0,0,0,5,5,7,7,7,5,5,5,5,6,6,6,6,6,0,0,5,5,5,5,5,5,5,5,5,5,5,7,7,7,7,7,6,6,6,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,5,0,0,0,0,0,5,5,5,6,6,6,6,6,7,7,7,7,5,5,5,0,0,5,5,5,0,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,0,0,0,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,5,5,5,5,0,0,0,0,0,6,6,6,6,6,0,0,0,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,5,7,7,7,7,7,7,7,6,6,6,6,5,5,0,0,0,0,0,0,0,0,0,0,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,0,0,0,6,6,6,6,6,6,6,6,6,5,5,0,7,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,0,5,7,7,7,6,6,6,0,0,5,6,6,6,6,6,6,5,5,5,7,7,7,7,7,7,6,5,5,0,0,6,6,6,6,6,6,6,6,5,5,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,5,6,6,6,6,6,6,6,6,0,0,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,5,0,0,0,0,0,6,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,0,0,6,6,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,5,5,5,5,5,0,5,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,5,5,5,5,6,6,6,6,0,0,5,6,6,6,6,6,5,5,6,6,6,6,6,6,0,0,0,0,0,0,6,6,6,5,6,0,0,0,0,6,6,6,6,6,6,6,6,6,6,0,0,0,0,0,0,0,6,5,5,5,5,5,7,7,7,7,7,6,6,6,6,6,6,0,0,0,5,5,5,5,5,5,5,5,0,0,0,0,5,5,5,5,0,0,0,5,5,5,5,5,5,0,0,0,0,6,6,6,6,0,0,0,0,0,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,5,6,6,6,7,7,7,5,0,5,5,6,6,6,6,0,0,0,0,0,0,7,7,7,5,5,5,0,0,0,0,0,0,5,5,5,6,6,5,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,6,6,6,6,6,6,6,5,7,7,6,6,6,6,6,5,6,6,6,6,6,6,7,7,7,7,5,5,5,5,5,5,6,6,6,6,6,5,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,5,6,6,6,6,6,6,6,7,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6];
pub const STONE_TORCH_64X64: [u8; 4096] = [6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,5,5,5,5,5,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,6,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,7,0,0,0,7,7,5,5,5,0,0,0,0,6,6,6,6,6,6,5,5,5,5,5,5,6,5,5,5,5,5,5,0,0,0,0,0,0,0,6,6,6,6,6,6,6,5,5,5,5,6,0,0,0,0,6,0,0,6,5,0,0,0,7,7,6,6,6,7,7,7,6,5,5,5,5,5,6,6,6,6,6,5,0,0,0,0,0,7,7,7,7,7,7,7,6,5,5,5,5,5,5,0,0,6,6,5,0,0,5,5,6,5,5,5,5,5,0,0,0,5,5,5,6,6,7,7,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,0,0,0,0,6,6,7,7,7,7,7,5,5,5,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,6,6,6,5,6,0,0,0,0,6,6,0,0,0,0,0,0,0,0,6,6,6,6,0,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,0,0,0,6,5,6,6,6,6,6,6,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,0,0,0,0,0,5,5,5,5,5,0,0,0,0,0,0,6,6,6,6,6,6,6,5,6,6,5,5,6,6,6,0,0,0,5,5,5,5,5,5,6,0,0,0,5,6,6,6,6,6,7,7,7,7,7,7,7,7,6,6,6,6,6,6,5,5,10,0,0,5,6,6,0,0,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,6,5,5,5,6,0,5,0,0,0,0,0,0,5,5,0,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,10,10,10,5,6,6,6,6,0,0,0,0,0,0,0,0,7,7,7,7,7,7,6,6,6,6,6,6,5,5,5,5,6,6,0,0,7,7,6,6,5,5,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,10,10,10,10,10,6,6,6,6,6,6,6,6,6,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,7,7,6,6,6,6,6,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,10,10,9,10,10,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,10,9,9,9,10,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,10,9,9,9,10,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,4,4,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,4,4,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,6,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,7,7,7,7,5,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,6,6,5,5,5,5,6,5,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,0,0,0,5,5,5,5,5,5,5,5,5,5,0,0,5,5,5,4,4,4,4,4,7,7,5,5,5,5,5,5,5,0,0,5,5,5,0,0,0,0,5,5,7,7,7,5,5,5,5,6,6,6,6,6,0,0,5,5,5,5,5,5,5,5,5,5,5,7,7,7,7,7,6,6,6,5,5,5,6,6,6,4,4,4,4,4,6,6,6,6,6,6,6,0,0,5,0,0,0,0,0,5,5,5,6,6,6,6,6,7,7,7,7,5,5,5,0,0,5,5,5,0,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,4,4,4,4,6,6,6,6,6,6,6,6,0,0,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,4,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,4,4,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,4,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,0,0,0,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,0,6,0,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,5,5,5,5,0,0,0,0,0,6,6,6,6,6,0,0,0,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,0,0,0,6,0,6,0,6,6,6,6,6,6,6,6,5,7,7,7,7,7,7,7,6,6,6,6,5,5,0,0,0,0,0,0,0,0,0,0,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,0,0,0,6,0,6,6,6,6,6,6,6,5,5,0,7,6,6,6,6,6,6,6,6,6,6,6,6,5,5,5,5,5,5,5,5,0,5,7,7,7,6,6,6,0,0,5,6,6,6,6,6,6,5,5,5,7,7,7,7,7,7,6,5,5,0,0,0,6,0,6,6,6,6,6,5,5,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,5,6,6,6,6,6,6,6,6,0,0,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,0,5,0,0,0,0,0,6,6,6,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,5,5,0,0,0,6,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,6,6,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,5,0,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,0,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,0,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,0,6,0,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,0,6,0,6,0,6,5,5,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,0,6,0,6,0,6,0,6,0,6,0,6,0,6,6,5,5,6,6,6,6,6,5,5,5,5,5,0,5,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,5,5,5,5,5,5,6,6,6,6,0,0,5,6,6,6,6,6,5,5,6,6,6,6,6,6,0,0,0,0,0,0,6,6,6,5,6,0,0,0,0,6,6,6,6,6,6,6,6,6,6,0,0,0,0,0,0,0,6,5,5,5,5,5,7,7,7,7,7,6,6,6,6,6,6,0,0,0,5,5,5,5,5,5,5,5,0,0,0,0,5,5,5,5,0,0,0,5,5,5,5,5,5,0,0,0,0,6,6,6,6,0,0,0,0,0,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,5,6,6,6,7,7,7,5,0,5,5,6,6,6,6,0,0,0,0,0,0,7,7,7,5,5,5,0,0,0,0,0,0,5,5,5,6,6,5,0,0,0,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,6,6,6,6,6,6,6,5,7,7,6,6,6,6,6,5,6,6,6,6,6,6,7,7,7,7,5,5,5,5,5,5,6,6,6,6,6,5,0,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,5,5,6,6,6,6,6,6,6,7,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,5,6,6,6,6,6,6,6,6,6,6
];
pub struct Texture {
    width: usize,
    height: usize,
    color: u8,
    columns: Vec<Vec<u8>>
}

impl Texture {
    pub fn new(data: &[u8], width: usize, height: usize) -> Self {
        
        let mut columns: Vec<Vec<u8>> = Vec::new();

        for texture_row in data.chunks_exact(width) {
            for pixel in texture_row.chunks_exact(1).enumerate() {
                match columns.get_mut(pixel.0) {
                    Some(column) => {
                        column.push(pixel.1[0]);
                    },
                    None => {
                        let mut column: Vec<u8> = Vec::new();
                        column.push(pixel.1[0]);
                        columns.push(column);
                    }
                }
            }
        }

        Texture {
            width,
            height,
            color: LIGHT_GREY,
            columns
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_column(&self, index: usize) -> &Vec<u8> {
        &self.columns[index]
    }
}