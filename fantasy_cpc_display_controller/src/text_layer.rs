use crate::{characters_rom::*, color_palettes::*, config::*};
pub struct TextLayer {
    pub size: (usize, usize),
    pub character_size: (usize, usize),
    pub chars: Vec<TextLayerCell>,
}

#[derive(Clone)]
pub struct TextLayerCell {
    pub c: char,
    pub color: usize,
    pub bkg_color: usize,
    pub swap_color: bool,
    pub blink: bool,
    pub shadowed: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

const DEFAULT_TEXT_CELL: TextLayerCell = TextLayerCell {
    c: ' ',
    color: WHITE,
    bkg_color: BLACK,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub trait TextLayerTrait {
    fn text_coord_to_frame_coord(x: usize, y: usize) -> (usize, usize);
    fn write(x: usize, y: usize, color: usize, bkg_color: usize, str: String);
    fn color(start_x: usize, start_y: usize, end_x: usize, end_y: usize, color: usize);
    fn bkg_color(start_x: usize, start_y: usize, end_x: usize, end_y: usize, bkg_color: usize);
    fn swap_colors(start_x: usize, start_y: usize, end_x: usize, end_y: usize, swap_colors: bool);
    fn blink(start_x: usize, start_y: usize, end_x: usize, end_y: usize, blink: bool);
    fn shadow(start_x: usize, start_y: usize, end_x: usize, end_y: usize, shadowed: bool);
    fn flip_h(start_x: usize, start_y: usize, end_x: usize, end_y: usize, flip_h: bool);
    fn flip_v(start_x: usize, start_y: usize, end_x: usize, end_y: usize, flip_v: bool);
}

impl TextLayer {
    pub const fn new(size: (usize, usize), character_size: (usize, usize)) -> TextLayer {
        TextLayer {
            size,
            character_size,
            chars: vec![DEFAULT_TEXT_CELL; size.0 * size.1],
        }
    }
}
