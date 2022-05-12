//Struct describing all the settings one character can have in text mode
//TODO implement "flipp" tells the renderer to flip the color and background of that character
//TODO implement "blink" tells the renderer to automatically flip the color and background of that character at a set interval, useful for blinking warning messages
#[derive(Copy, Clone)]
pub struct TextLayerChar {
    pub c: char,
    pub background_color: u8,
    pub color: u8,
    pub flipp: bool,
    pub blink: bool
}

//The virtual text mode buffer, width and height are expressed in characters
pub struct TextLayer {
    columns: usize,
    rows: usize,
    characters: Vec<Option<TextLayerChar>>
}

impl TextLayer {

    pub fn new(columns: usize, rows: usize) -> TextLayer {
        let fb: Vec<Option<TextLayerChar>> = Vec::new();
        TextLayer {
            columns,
            rows,
            characters: fb
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

    pub fn push_character(&mut self, tmchar: Option<TextLayerChar>) {
        if self.characters.len() == self.columns * self.rows {
            for i in 0..self.characters.len() -1 {
                self.characters[i] = self.characters[i+1];
            }
            self.characters.pop();
            self.characters.push(tmchar);
        } else {
            self.characters.push(tmchar);
        }
    }

    pub fn push_char(&mut self, c: char, color: u8, back_color: u8, blink: bool) {
        let a_char = TextLayerChar {
            c: c,
            background_color: back_color,
            color: color,
            flipp: false,
            blink: blink
        };
        let a_cell = Some(a_char);
        self.push_character(a_cell);
    }

    pub fn push_string(&mut self, string: &str, color: u8, back_color: u8, blink: bool) {
        for c in string.chars() {
            self.push_char(c, color, back_color, blink);
        }
    }

    pub fn push_string_line(&mut self, string: &str, color: u8, back_color: u8, blink: bool) {
        for c in string.chars() {
            self.push_char(c, color, back_color, blink);
        }

        let reminder = self.get_characters().len() % self.columns;
        for _i in 0..(self.columns - reminder) {
            self.push_character(None);
        }
    }

    //pops the last cell, just the last one, wether it contains a character or None.
    pub fn pop_char(&mut self) {
        self.characters.pop();
    }

    pub fn pop_all_none(&mut self) {

        let mut stop: bool = false;

        while match self.characters.last() { //Returns a Option<Option<TextLayerChar>> ... 
            Some(plop) => {
                match plop {
                    Some(t) => {
                        match t.c {
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

    pub fn clear(&mut self) {
        self.characters.clear();
    }

    pub fn scroll_up(&mut self) {
        for i in self.columns..self.characters.len() {
            self.characters[i - self.columns] = self.characters[i];
        }

        for _i in 0..self.columns {
            self.pop_char();
        }
    }
}