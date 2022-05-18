
use crate::color_palettes::*;
/// TODO implement "blink" tells the renderer to automatically flip the color and background of that character at a set interval, useful for blinking warning messages

/// Struct describing all the settings one character can have in text mode
#[derive(Copy, Clone)]
pub struct TextLayerChar {
    pub unicode: char,
    pub color: Option<ColorPalette>,
    pub background_color: Option<ColorPalette>,
    pub flipp: bool,
    pub blink: bool
}

/// The text layer buffer
pub struct TextLayer {
    columns: usize,
    rows: usize,
    characters: Vec<Option<TextLayerChar>>,
    pub show_cursor: bool
}

impl TextLayer {

    pub fn new(columns: usize, rows: usize) -> TextLayer {
        let mut fb: Vec<Option<TextLayerChar>> = Vec::new();
        fb.push(Some(TextLayerChar {
            unicode: '\u{25AE}',
            color: None,
            background_color: None,
            flipp: false,
            blink: true
        }));
        TextLayer {
            columns,
            rows,
            characters: fb,
            show_cursor: true
        }
    }

    pub fn get_columns(&self) -> usize {
        return self.columns;
    }

    pub fn get_rows(&self) -> usize {
        return self.rows;
    }

    pub fn get_characters(&self) -> &Vec<Option<TextLayerChar>> {
        return &self.characters;
    }

    /// Pushes a craracter struct to the text layer
    pub fn push_character(&mut self, text_layer_char: Option<TextLayerChar>) {

        if self.show_cursor {
            self.characters.pop();
        }
        
        match text_layer_char {
            Some(c) => {
                match c.unicode {
                    '\u{0008}' => { //Backspace
                        self.characters.pop();
                    } 
                    
                    '\u{000D}' => { //Enter
                        if self.characters.len() % self.columns == 0 {
                            for _i in 0..self.columns {
                                self.characters.push(None);
                            }
                        }
                        while self.characters.len() % self.columns > 0 {
                            self.characters.push(None);
                        }
                    }
                    
                    _ => {
                        self.characters.push(text_layer_char);
                    }
                }
            }
            None => {
                self.characters.push(None);
            }
        }

        if self.show_cursor {
            self.characters.push(Some(TextLayerChar {
                unicode: '\u{25AE}',
                color: None,
                background_color: None,
                flipp: false,
                blink: true
            }));
        }
    }

    /// Pushes a char to the text layer, must specify the color
    pub fn push_char(&mut self, c: char, color: ColorPalette, back_color: ColorPalette, blink: bool) {
        let a_char = TextLayerChar {
            unicode: c,
            background_color: Some(back_color),
            color: Some(color),
            flipp: false,
            blink: blink
        };
        self.push_character(Some(a_char));
    }

    /// Pushes all the charaters from the &str to the vector representing the text buffer
    pub fn push_string(&mut self, string: &str, color: ColorPalette, back_color: ColorPalette, blink: bool) {
        for c in string.chars() {
            self.push_char(c, color, back_color, blink);
        }
    }

    /// Pushes all the charaters from the &str to the vector representing the text buffer
    /// and fills the remaining characters in the row with None 
    // pub fn push_string_line(&mut self, string: &str, color: ColorPalette, back_color: ColorPalette, blink: bool) {
    //     //How many characters are missing to fill the line
    //     let reminder = (self.get_characters().len() + self.columns - string.chars().count()) % self.columns;
        
    //     for c in string.chars() {
    //         self.push_char(c, color, back_color, blink);
    //     }

    //     println!("Total length: {}", self.get_characters().len());
    //     println!("Reminder: {}", reminder);

    //     for _i in 0..(reminder) {
    //         //self.push_character(None); 
    //         self.push_char('#', ColorPalette::Black, ColorPalette::Blue, false); 
    //     }
    // }

    /// Pops the last cell of the character leyer vector, wether it contains a character or None.
    pub fn pop_char(&mut self) {
        self.characters.pop();
    }

    /// Pops the last cell, and then continues poping all the None until it reaches a non None character.
    pub fn pop_all_none(&mut self) {

        let mut stop: bool = false;

        while match self.characters.last() { //Returns a Option<Option<TextLayerChar>> ... 
            Some(plop) => {
                match plop {
                    Some(t) => {
                        match t.unicode {
                            '\n' => {stop = true; true}
                            _ => {false}
                        }    
                    }
                    None => {true}
                }
            }
            None => {false}
        } {
            self.characters.pop();
            if stop {return}
        }
    }

    /// Clears the entire vector
    pub fn clear(&mut self) {
        self.characters.clear();
        if self.show_cursor {
            self.characters.push(Some(TextLayerChar {
                unicode: '\u{25AE}',
                color: None,
                background_color: None,
                flipp: false,
                blink: true
            }));
        }
    }

    /// Moves the entire content of the vector one line up
    /// it pops characters at the beginning of the vector to make
    /// the rest scroll up
    pub fn scroll_up(&mut self) {
        for i in self.columns..self.characters.len() {
            self.characters[i - self.columns] = self.characters[i];
        }

        for _i in 0..self.columns {
            self.pop_char();
        }
    }
}