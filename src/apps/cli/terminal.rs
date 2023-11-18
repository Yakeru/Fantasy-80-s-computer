use fantasy_cpc_display_controller::text_layer::{TextCell, TextCellStyle, TextLayer};
use unicode::BACKSPACE;

use crate::{
    color_palettes::{TRUE_BLUE, YELLOW},
    config::{TEXT_COLUMNS, TEXT_ROWS},
};

/// The terminal is the text window in which the Shell is displayed
pub struct Terminal {
    screen_coordinates: (usize, usize),
    size: (usize, usize),
    max_buffer_size: usize,
    pub default_color: usize,
    pub default_bkg_color: usize,
    pub show_border: bool,
    pub title: String,
    pub show_title_bar: bool,
    buffer: Vec<TextCell>,
    formatted_buffer: Vec<TextCell>,
}

const EMPTY_CELL: TextCell = TextCell {
    c: None,
    style: Some(TextCellStyle {
        color: YELLOW,
        bkg_color: TRUE_BLUE,
        swap_color: false,
        blink: false,
        shadowed: false,
        flip_h: false,
        flip_v: false,
    }),
};

const CURSOR: TextCell = TextCell {
    c: Some('█'),
    style: Some(TextCellStyle {
        color: YELLOW,
        bkg_color: TRUE_BLUE,
        swap_color: false,
        blink: true,
        shadowed: false,
        flip_h: false,
        flip_v: false,
    }),
};

const DEFAULT_STYLE: TextCellStyle = TextCellStyle {
    color: YELLOW,
    bkg_color: TRUE_BLUE,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

impl Terminal {
    pub fn new(screen_coordinates: (usize, usize), size: (usize, usize)) -> Terminal {
        Terminal {
            screen_coordinates,
            size,
            max_buffer_size: 1000,
            default_color: YELLOW,
            default_bkg_color: TRUE_BLUE,
            show_border: false,
            title: "Term0".to_string(),
            show_title_bar: false,
            buffer: Vec::new(),
            formatted_buffer: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Size in columns (x) and rows (y), used by format_buffer() and
    /// the text layer renderer to format and display the console on screen
    pub fn _get_size(&self) -> (usize, usize) {
        self.size
    }

    /// Top-Left corner, used by the text layer renderer to show the console at the right place on the screen
    pub fn _get_coordinates(&self) -> (usize, usize) {
        self.screen_coordinates
    }

    pub fn set_size(&mut self, size: (usize, usize)) {
        let col_count = size.0.clamp(10, TEXT_COLUMNS);
        let row_count = size.1.clamp(3, TEXT_ROWS);
        self.size = (col_count, row_count)
    }

    pub fn set_coordinates(&mut self, xy_coord: (usize, usize)) {
        let x = xy_coord.0.clamp(0, TEXT_COLUMNS - self.size.0);
        let y = xy_coord.1.clamp(0, TEXT_ROWS - self.size.1);
        self.screen_coordinates = (x, y);
    }

    fn pop_char(&mut self) {
        self.buffer.pop();
        self.format_buffer();
    }

    /// Add a char to the consoles's buffer
    /// will convert it to a TextLayerCell with the console's default
    /// attributes and then call push_text_layer_char()
    pub fn push_char(&mut self, c: char, style: Option<TextCellStyle>) {
        match c {
            BACKSPACE => {
                self.pop_char();
            }
            _ => match style {
                Some(style) => self.buffer.push(TextCell {
                    c: Some(c),
                    style: Some(style),
                }),
                None => self.buffer.push(TextCell {
                    c: Some(c),
                    style: Some(DEFAULT_STYLE),
                }),
            },
        }
        self.format_buffer();
    }

    /// Add the whole content of a &str to the consoles's buffer
    /// call's push_char() for each char in the string
    pub fn push_string(&mut self, string: &str, style: Option<TextCellStyle>) {
        for c in string.chars() {
            self.push_char(c, style);
        }
    }

    /// Formats the buffer content to display it properly in the console depending
    /// on it's size
    /// for example: if it encounters a ENTER character, fills the line with empty chars
    /// to automatically move to the next line.
    /// If you want to apply your own formatting, use get_buffer() instead.
    fn format_buffer(&mut self) {
        self.formatted_buffer.clear();

        for styled_char in &self.buffer {
            match styled_char.c.unwrap() {
                unicode::ENTER => {
                    for _i in 0..(self.size.0 - self.formatted_buffer.len() % self.size.0) {
                        self.formatted_buffer.push(EMPTY_CELL)
                    }
                }
                _ => self.formatted_buffer.push(*styled_char),
            }
        }
        self.formatted_buffer.push(CURSOR);

        // pop first line if rendered buffer is bigger than screen_size
        while self.formatted_buffer.len() > (self.size.0 * self.size.1) {
            self.formatted_buffer.drain(0..self.size.0);
        }

        // fill the rest with empty cells
        while self.formatted_buffer.len() < (self.size.0 * self.size.1) {
            self.formatted_buffer.push(EMPTY_CELL);
        }
    }

    ///
    pub fn render(&mut self, txt: &mut TextLayer) {
        let mut buffer_index: usize = 0;
        let screen = txt.get_map_mut();

        for line in self.screen_coordinates.1..self.size.1 {
            for column in self.screen_coordinates.0..self.size.0 {
                screen[line][column] = self.formatted_buffer[buffer_index];
                buffer_index += 1;
            }
        }
    }
}
