#[derive(Clone, Copy)]
pub struct TextLayerChar {
    pub c: char,
    pub color: u8,
    pub bkg_color: u8,
    pub swap: bool,
    pub blink: bool,
    pub shadowed: bool
}