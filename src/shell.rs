pub const CHARACTER_WIDTH: u8 = 8;
pub const CHARACTER_HEIGHT: u8 = 8;

pub struct Shell {
    buffer_size: usize,
    columns: u8,
    rowns: u8,
    buffer: Vec<char>,
    cursor_index: usize
}

impl Shell {

    pub fn new(buffer_size: usize, column_count: u8, row_count: u8) -> Shell {

        let buffer: Vec<char> = Vec::new();

        Shell {
            buffer_size: buffer_size,
            columns: column_count,
            rowns: row_count,
            buffer: buffer,
            cursor_index: 0
        }
    }

    pub fn get_buffer(&self) -> &Vec<char> {
        return &self.buffer;
    }

    pub fn get_nb_rows(&self) -> u8 {
        return self.rowns;
    }

    pub fn get_nb_columns(&self) -> u8 {
        return self.columns;
    }

    pub fn send_character_to_shell(&mut self, c: char) {
        if self.buffer.len() == self.buffer_size {
            for i in 0..(self.buffer.len() - 1) {
                self.buffer[i] = self.buffer[i+1];
                self.buffer.pop();
                self.buffer.push(c);
            }
        } else {
            self.buffer.push(c);
        }
    }

    pub fn empty_buffer(&mut self) {
        while self.buffer.len() > 0 {
            self.buffer.pop();
        }
        self.cursor_index = 0
    }
}