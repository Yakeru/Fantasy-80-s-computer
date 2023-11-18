use crate::{characters_rom::*, color_palettes::*, config::*};

#[derive(Clone, Copy)]
pub struct TextCellStyle {
    pub color: usize,
    pub bkg_color: usize,
    pub swap_color: bool,
    pub blink: bool,
    pub shadowed: bool,
    pub flip_h: bool,
    pub flip_v: bool,
}

#[derive(Clone, Copy)]
pub struct TextCell {
    pub c: Option<char>,
    pub style: Option<TextCellStyle>,
}

const EMPTY_CELL: TextCell = TextCell {
    c: None,
    style: None,
};

pub const DEFAULT_STYLE: TextCellStyle = TextCellStyle {
    color: WHITE,
    bkg_color: BLACK,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub struct TextLayer {
    map: [[TextCell; TEXT_COLUMNS]; TEXT_ROWS],
}

pub enum Text {
    Char(char),
    String(String),
    ColoredChar(char, usize, usize),
    ColoredString(String, usize, usize),
    StyledChar(char, TextCellStyle),
    StyledString(String, TextCellStyle),
}

impl TextLayer {
    pub const fn new() -> TextLayer {
        TextLayer {
            map: [[EMPTY_CELL; TEXT_COLUMNS]; TEXT_ROWS],
        }
    }

    pub fn clear(&mut self, style: Option<TextCellStyle>) {
        self.map = [[TextCell { c: None, style }; TEXT_COLUMNS]; TEXT_ROWS];
    }

    /// Returns the dimensions in columns and rowns of the text layer map.
    pub fn get_dimensions_xy(&self) -> (usize, usize) {
        (TEXT_COLUMNS, TEXT_ROWS)
    }

    /// Returns the lenght of the char_map array.
    pub fn get_len(&self) -> usize {
        TEXT_COLUMNS * TEXT_ROWS
    }

    pub fn get_map(&self) -> &[[TextCell; TEXT_COLUMNS]; TEXT_ROWS] {
        &self.map
    }

    pub fn get_map_mut(&mut self) -> &mut [[TextCell; TEXT_COLUMNS]; TEXT_ROWS] {
        &mut self.map
    }

    fn valid_coord(&self, line: usize, column: usize) -> bool {
        line < TEXT_ROWS && column < TEXT_COLUMNS
    }

    /// Inserts a string in the map at the specified x and y position.
    pub fn set_char(&mut self, line: usize, column: usize, c: char) {
        if self.valid_coord(line, column) {
            self.map[line][column].c = Some(c);
        }
    }

    pub fn set_style(&mut self, line: usize, column: usize, style: TextCellStyle) {
        if self.valid_coord(line, column) {
            self.map[line][column].style = Some(style);
        }
    }

    pub fn set_color(&mut self, line: usize, column: usize, color: usize, bkg_color: usize) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.map[line][column].style {
                style.color = color;
                style.bkg_color = bkg_color;
            } else {
                let style: TextCellStyle = TextCellStyle {
                    color,
                    bkg_color,
                    swap_color: false,
                    blink: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                self.map[line][column].style = Some(style);
            }
        }
    }

    pub fn set_swap(&mut self, line: usize, column: usize, swap_color: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.map[line][column].style {
                style.swap_color = swap_color;
            }
        }
    }

    pub fn set_blink(&mut self, line: usize, column: usize, blink: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.map[line][column].style {
                style.blink = blink;
            }
        }
    }

    pub fn set_shadowed(&mut self, line: usize, column: usize, shadowed: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.map[line][column].style {
                style.shadowed = shadowed;
            }
        }
    }

    pub fn set_flip(&mut self, line: usize, column: usize, flip_h: bool, flip_v: bool) {
        if self.valid_coord(line, column) {
            if let Some(mut style) = self.map[line][column].style {
                style.flip_h = flip_h;
                style.flip_v = flip_v;
            }
        }
    }

    /// Inserts a string in the map at the specified x and y position.
    pub fn write(&mut self, line: usize, column: usize, text: Text) {
        match text {
            Text::Char(c) => self.set_char(line, column, c),
            Text::String(str) => {
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c)
                    }
                }
            }
            Text::StyledChar(c, style) => {
                self.set_char(line, column, c);
                self.set_style(line, column, style);
            }
            Text::StyledString(str, style) => {
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c);
                        self.set_style(line, column + char_index, style);
                    }
                }
            }
            Text::ColoredChar(c, color, bkg_color) => {
                let style = TextCellStyle {
                    color,
                    bkg_color,
                    blink: false,
                    swap_color: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                self.set_char(line, column, c);
                self.set_style(line, column, style);
            }
            Text::ColoredString(str, color, bkg_color) => {
                let style = TextCellStyle {
                    color,
                    bkg_color,
                    blink: false,
                    swap_color: false,
                    shadowed: false,
                    flip_h: false,
                    flip_v: false,
                };
                if !str.is_empty() {
                    for (char_index, c) in str.chars().enumerate() {
                        self.set_char(line, column + char_index, c);
                        self.set_style(line, column + char_index, style);
                    }
                }
            }
        }
    }
}

// const fn safe_coord(x: usize, y: usize) -> (usize, usize) {
//     (x % TEXT_COLUMNS, (y + x / TEXT_COLUMNS) % TEXT_ROWS)
// }

pub const fn text_coord_to_frame_coord(x: usize, y: usize) -> (usize, usize) {
    let horizontal_border: usize = (VIRTUAL_WIDTH - TEXT_COLUMNS * CHARACTER_WIDTH) / 2;
    let vertical_border: usize = (VIRTUAL_HEIGHT - TEXT_ROWS * CHARACTER_HEIGHT) / 2;
    let safe_x = x % TEXT_COLUMNS;
    let safe_y = y % TEXT_ROWS;
    let x_pos = horizontal_border + safe_x * CHARACTER_WIDTH;
    let y_pos = vertical_border + safe_y * CHARACTER_HEIGHT;
    (x_pos, y_pos)
}
