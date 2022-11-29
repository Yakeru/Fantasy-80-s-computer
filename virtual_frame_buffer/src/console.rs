use crate::text_layer_char::TextLayerChar;

pub struct Console {
    pub display: bool,
    pub pos_x: usize,
    pub pos_y: usize,
    columns: usize,
    rows: usize,
    pub default_color: u8,
    pub default_bkg_color: u8,
    pub cursor: TextLayerChar,
    pub show_border: bool,
    pub show_title_bar: bool,
    content: Vec<Option<TextLayerChar>>,
    line_indexes: Vec<usize>
}

impl Console {

    pub const fn new(pos_x: usize, pos_y: usize, columns: usize, rows: usize, 
        default_color: u8, default_bkg_color: u8, cursor: TextLayerChar, 
        show_border: bool, show_title_bar: bool) -> Console {
            Console {
                display: true, pos_x, pos_y, 
                columns: if columns < 10 { 10 } else { columns }, 
                rows: if rows < 1 { 1 } else { rows }, 
                default_color, 
                default_bkg_color,
                cursor, 
                show_border, 
                show_title_bar, 
                content: Vec::new(),
                line_indexes: Vec::new()
            }
    }

    pub fn get_row_count(&self) -> usize {
        self.rows
    }

    pub fn get_col_count(&self) -> usize {
        self.columns
    }

    pub fn set_row_count(&mut self, row_count: usize) {
        self.rows = row_count;
    }

    pub fn set_col_count(&mut self, col_count: usize) {
        self.columns = col_count;
    }

    pub fn pop_char(&mut self) {
        //Pop char
        if self.content.last().is_some() && self.content.last().unwrap().is_some() {
            self.content.pop();
            return
        }
        
        //Pop all the None until next char
        while self.content.last().is_some() && self.content.last().unwrap().is_none() {
            self.content.pop();
        }
    }

    pub fn push_char(&mut self, c: char) {
        match c {
            unicode::ENTER => {
                for i in 0..(self.columns - self.content.len() % self.columns) {
                    self.content.push(None);    
                }
            },
            unicode::BACKSPACE => (),
            _ => {
                let text_layer_char = TextLayerChar {c, color: self.default_color, bkg_color: self.default_bkg_color, swap: false, blink: false, shadowed: false};
                self.content.push(Some(text_layer_char));
            }
        }

        if self.content.len() == self.columns * self.rows {
            for i in 0..self.columns {
                self.content.remove(i);
            }
        }
    }

    pub fn push_text_layer_char(&mut self, text_layer_char: TextLayerChar) {
        match text_layer_char.c {
            unicode::ENTER => {
                for i in 0..(self.columns - self.content.len() % self.columns) {
                    self.content.push(None);    
                }
            },
            unicode::BACKSPACE => (),
            _ => {
                self.content.push(Some(text_layer_char));
            }
        }

        if self.content.len() == self.columns * self.rows {
            for i in 0..self.columns {
                self.content.remove(i);
            }
        }
    }

    pub fn get_content(&self) -> &Vec<Option<TextLayerChar>> {
        &self.content
    }
}