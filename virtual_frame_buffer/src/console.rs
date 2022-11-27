use crate::text_layer_char::TextLayerChar;

pub struct Console {
    pub display: bool,
    pub pos_x: usize,
    pub pos_y: usize,
    pub columns: usize,
    pub rows: usize,
    pub default_color: u8,
    pub default_bkg_color: u8,
    pub cursor: TextLayerChar,
    pub show_border: bool,
    pub show_title_bar: bool,
    pub content: Vec<TextLayerChar>
}

impl Console {

    pub const fn new(pos_x: usize, pos_y: usize, 
        columns: usize, rows: usize, default_color: u8, default_bkg_color: u8,
        cursor: TextLayerChar, show_border: bool, show_title_bar: bool) -> Console {
            Console {display: true, pos_x, pos_y, 
                columns: if columns < 10 { 10 } else { columns }, 
                rows: if rows < 2 { 2 } else { rows }, 
                default_color, 
                default_bkg_color,
                cursor, 
                show_border, 
                show_title_bar, 
                content: Vec::new()}
    }
}