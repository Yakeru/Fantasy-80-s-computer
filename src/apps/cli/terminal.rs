use display_controller::DisplayController;

use crate::{
    color_palettes::{TRUE_BLUE, YELLOW},
    config::{TEXT_COLUMNS, TEXT_ROWS},
    text_layer::TextLayerChar,
};

/// The terminal is the text window in which the Shell is displayed
pub struct Terminal {
    screen_coordinates: (usize, usize),
    screen_size: (usize, usize),
    max_buffer_size: usize,
    pub default_color: usize,
    pub default_bkg_color: usize,
    pub cursor: char,
    pub show_border: bool,
    pub show_title_bar: bool,
    buffer: Vec<TextLayerChar>,
    formatted_buffer: Vec<TextLayerChar>,
}

impl Terminal {
    pub const fn new() -> Terminal {
        Terminal {
            screen_coordinates: (0, 0),
            screen_size: (TEXT_COLUMNS, TEXT_ROWS),
            max_buffer_size: 1000,
            default_color: YELLOW,
            default_bkg_color: TRUE_BLUE,
            cursor: '\u{25AE}', // filled square
            show_border: false,
            show_title_bar: false,
            buffer: Vec::new(),
            formatted_buffer: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.formatted_buffer.clear();
    }

    /// Size in columns (x) and rows (y), used by format_buffer() and
    /// the text layer renderer to format and display the console on screen
    pub fn _get_size(&self) -> (usize, usize) {
        self.screen_size
    }

    /// Top-Left corner, used by the text layer renderer to show the console at the right place on the screen
    pub fn _get_coordinates(&self) -> (usize, usize) {
        self.screen_coordinates
    }

    pub fn set_size(&mut self, size: (usize, usize)) {
        let col_count = size.0.clamp(10, TEXT_COLUMNS);
        let row_count = size.1.clamp(3, TEXT_ROWS);
        self.screen_size = (col_count, row_count)
    }

    pub fn set_coordinates(&mut self, xy_coord: (usize, usize)) {
        let x = xy_coord.0.clamp(0, TEXT_COLUMNS - self.screen_size.0);
        let y = xy_coord.1.clamp(0, TEXT_ROWS - self.screen_size.1);
        self.screen_coordinates = (x, y);
    }

    pub fn pop_char(&mut self) {
        if self.buffer.last().is_some() {
            self.buffer.pop();
        }
        self.format_buffer();
    }

    /// Add a char to the consoles's buffer
    /// will convert it to a TextLayerChar with the console's default
    /// attributes and then call push_text_layer_char()
    pub fn push_char(&mut self, c: char) {
        let text_layer_char = TextLayerChar {
            c,
            color: self.default_color,
            bkg_color: self.default_bkg_color,
            swap: false,
            blink: false,
            shadowed: false,
        };
        self.push_text_layer_char(text_layer_char);
    }

    /// Add the whole content of a &str to the consoles's buffer
    /// call's push_char() for each char in the string
    pub fn push_string(&mut self, string: &str) {
        for char in string.chars() {
            self.push_char(char);
        }
    }

    /// Pushes a TextLayerChar into the console's buffer (Vec<TextLayerChar>)
    /// if the character received is BACKSPACE, will pop the last character instead.
    pub fn push_text_layer_char(&mut self, text_layer_char: TextLayerChar) {
        match text_layer_char.c {
            unicode::BACKSPACE => {
                self.pop_char();
            }
            _ => {
                self.buffer.push(text_layer_char);
            }
        }

        if self.buffer.len() > self.max_buffer_size {
            self.buffer.remove(0);
        }

        self.format_buffer();
    }

    /// Returns the raw Vec<TextLayerChar> of characters
    /// contained in the console's buffer
    fn _get_buffer(&self) -> &Vec<TextLayerChar> {
        &self.buffer
    }

    /// Returns the content of the console buffer formated to be
    /// displayed in a text grid the size of the console.
    /// For example: if it encounters a ENTER character, get_buffer() simply returns
    /// the ENTER char, but get_formatted_buffer() fills the rest of the line with empty chars
    /// to automatically move to the next line.
    /// If you want to apply your own formatting, use get_buffer() instead.
    fn get_formatted_buffer(&self) -> &Vec<TextLayerChar> {
        &self.formatted_buffer
    }

    fn get_empty_cell(&self) -> TextLayerChar {
        TextLayerChar {
            c: ' ',
            color: self.default_color,
            bkg_color: self.default_bkg_color,
            swap: false,
            blink: false,
            shadowed: false,
        }
    }

    pub fn get_cursor(&self) -> TextLayerChar {
        TextLayerChar {
            c: self.cursor,
            color: self.default_color,
            bkg_color: self.default_bkg_color,
            swap: false,
            blink: true,
            shadowed: false,
        }
    }

    /// Formats the buffer content to display it properly in the console depending
    /// on it's size
    /// for example: if it encounters a ENTER character, fills the line with empty chars
    /// to automatically move to the next line.
    /// If you want to apply your own formatting, use get_buffer() instead.
    fn format_buffer(&mut self) {
        self.formatted_buffer.clear();

        for console_char in &self.buffer {
            match console_char.c {
                unicode::ENTER => {
                    for _i in
                        0..(self.screen_size.0 - self.formatted_buffer.len() % self.screen_size.0)
                    {
                        self.formatted_buffer.push(self.get_empty_cell())
                    }
                }
                _ => self.formatted_buffer.push(*console_char),
            }
        }
        self.formatted_buffer.push(self.get_cursor());

        // pop first line if rendered buffer is bigger than screen_size
        while self.formatted_buffer.len() > (self.screen_size.0 * self.screen_size.1) {
            self.formatted_buffer.drain(0..self.screen_size.0);
        }

        // fill the rest with empty cells
        while self.formatted_buffer.len() < (self.screen_size.0 * self.screen_size.1) {
            self.formatted_buffer.push(self.get_empty_cell());
        }
    }

    ///
    pub fn render(&mut self, dc: &mut DisplayController) {
        for (index, tlchar) in self.get_formatted_buffer().iter().enumerate() {
            dc.get_text_layer_mut()
                .insert_text_layer_char(index, *tlchar);
        }
    }
}
