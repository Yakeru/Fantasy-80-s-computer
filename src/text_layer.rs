const TEXT_COLUMNS: usize = 40;
const TEXT_ROWS: usize = 30;
const DEFAULT_COLOR: u8 = 10;
const DEFAULT_BKG_COLOR: u8 = 28;
const DEFAULT_CURSOR: char = '\u{25AE}';

/// Struct describing all the settings one character can have in text mode
#[derive(Copy, Clone)]
pub struct TextLayerChar {
    pub unicode: char,
    pub color: u8,
    pub background_color: u8,
    pub flipp: bool,
    pub blink: bool
}

/// The text layer buffer
pub struct TextLayer {
    color: u8,
    bkg_color: u8,
    cursor: char,
    characters: Vec<Option<TextLayerChar>>,
    pub show_cursor: bool
}

impl TextLayer {

    pub fn new() -> TextLayer {
        let mut fb: Vec<Option<TextLayerChar>> = Vec::new();
        fb.push(Some(TextLayerChar {
            unicode: DEFAULT_CURSOR,
            color: DEFAULT_COLOR,
            background_color: DEFAULT_BKG_COLOR,
            flipp: false,
            blink: true
        }));
        TextLayer {
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            cursor: DEFAULT_CURSOR,
            characters: fb,
            show_cursor: true
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        return (TEXT_COLUMNS, TEXT_ROWS);
    }

    pub fn get_characters(&self) -> &Vec<Option<TextLayerChar>> {
        return &self.characters;
    }

    pub fn set_default_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn set_default_bkg_color(&mut self, bkg_color: u8) {
        self.bkg_color = bkg_color;
    }

    pub fn get_default_bkg_color(&mut self) -> u8 {
        self.bkg_color
    }

    pub fn set_cursor(&mut self, cursor: char) {
        self.cursor = cursor;
    }

    /// Pushes a character struct to the text layer
    pub fn push_character(&mut self, text_layer_char: Option<TextLayerChar>) {

        if self.show_cursor {
            self.characters.pop();
        }

        if self.characters.len() >= TEXT_COLUMNS * TEXT_ROWS {
            self.scroll_up();
        }
        
        match text_layer_char {
            Some(c) => {
                match c.unicode {
                    '\u{0008}' => { //Backspace
                        self.characters.pop();
                    } 
                    
                    '\u{000D}' => { //Enter
                        if self.characters.len() % TEXT_COLUMNS == 0 {
                            for _i in 0..TEXT_COLUMNS {
                                self.characters.push(None);
                            }
                        }
                        while self.characters.len() % TEXT_COLUMNS > 0 {
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
                unicode: self.cursor,
                color: self.color,
                background_color: self.bkg_color,
                flipp: false,
                blink: true
            }));
        }
    }

    /// Pushes a char to the text layer, must specify the color
    pub fn push_char(&mut self, c: char, color: Option<u8>, back_color: Option<u8>, blink: bool) {
        let a_char = TextLayerChar {
            unicode: c,
            color: if color.is_some() {color.unwrap()} else {self.color},
            background_color: if back_color.is_some() {back_color.unwrap()} else {self.bkg_color},
            flipp: false,
            blink: blink
        };
        self.push_character(Some(a_char));
    }

    /// Pushes all the charaters from the &str to the vector representing the text buffer
    pub fn push_string(&mut self, string: &str, color: Option<u8>, back_color: Option<u8>, blink: bool) {
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

    /// Pops the last cell of the character layer vector, wether it contains a character or None.
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
                            '\u{000D}' => {stop = true; true}
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
                unicode: self.cursor,
                color: self.color,
                background_color: self.bkg_color,
                flipp: false,
                blink: true
            }));
        }
    }

    /// Moves the entire content of the vector one line up
    /// it pops characters at the beginning of the vector to make
    /// the rest scroll up
    pub fn scroll_up(&mut self) {
        for i in TEXT_COLUMNS..self.characters.len() {
            self.characters[i - TEXT_COLUMNS] = self.characters[i];
        }

        for _i in 0..TEXT_COLUMNS {
            self.pop_char();
        }
    }
}