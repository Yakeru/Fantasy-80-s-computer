use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::text_layer::TextLayer;
use std::io::{self, Write};

const DEFAULT_BKG_COLOR: u8 = 0;
const DEFAULT_COLOR: u8 = 1;

pub struct TextEdit {
    color: u8,
    bkg_color: u8,
    columns: u8,
    rows: u8,
    command: Vec<char>,
    pub running: bool
}

impl TextEdit {

    pub fn new(text_layer: &mut TextLayer) -> TextEdit {

        // text_layer.clear();
        // text_layer.push_char('_', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false); //re insert cursor

        TextEdit {
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            columns: text_layer.get_columns() as u8,
            rows: text_layer.get_rows() as u8,
            command: Vec::new(),
            running: false
        }
    }

    pub fn draw (&mut self, text_layer: &mut TextLayer) {
    }

    pub fn update (&mut self, text_layer: &mut TextLayer, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow> {

        match character_received {
            Some(c) => {

                print!("{} ", c as u8);
                io::stdout().flush().unwrap();

                match c as u8 {
                    8 => { //Backspace
                        
                        self.command.pop();
                        
                        text_layer.pop_char(); //delete cursor
                        text_layer.pop_char(); //delete a char
                        text_layer.pop_all_none();
                        text_layer.push_char('_', self.color, self.bkg_color, false); //re insert cursor
                        
                    } 
                    
                    13 => { //Enter
                        
                        text_layer.pop_char(); //delete cursor
                        text_layer.push_char('\n', self.color, self.bkg_color, false); //re insert cursor
                        //push enough None characters to fill line and go to next
                        let reminder = text_layer.get_characters().len() % self.columns as usize;
                        for _i in 0..(self.columns as usize - reminder) {
                            text_layer.push_character(None);
                        }

                        //re insert cursor
                        text_layer.push_char('_', self.color, self.bkg_color, false);
                        
                        //Interpret line content as command
                        let mut string_command: String = String::from_iter(self.command.iter());
                        string_command = string_command.trim().to_lowercase();
                        println!("Command: '{}'", string_command.trim().to_lowercase());
                        self.command.clear();

                        if string_command == "help" {
                            text_layer.pop_char();
                            text_layer.push_string("Help is on the way !                    ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string("'clear'", 2, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string(" clears the screen.              ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string("'quit'", 2, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string(" or ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string("'exit'", 2, DEFAULT_BKG_COLOR, false);
                            text_layer.push_string(" shuts down computer.   ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            text_layer.push_char('_', self.color, self.bkg_color, false);
                        }

                        if string_command == "clear" {
                            text_layer.clear();
                            text_layer.push_char('_', self.color, self.bkg_color, false);
                        }

                        if string_command == "quit" || string_command == "exit"{
                            println!("Command 'quit' or 'exit' received; stopping");
                            return Some(ControlFlow::Exit);
                        }
                    }
                    
                    27 => { //Escape
                        ()
                    }
                    
                    _ => {

                        if text_layer.get_characters().len() >= self.columns as usize * (self.rows as usize - 1) {
                            text_layer.scroll_up();
                        }
                        text_layer.pop_char(); //delete cursor
                        text_layer.push_char(c, self.color, self.bkg_color, false); //push new char
                        text_layer.push_char('_', self.color, self.bkg_color, false); //re insert cursor
                        self.command.push(c);
                    }
                }

            }
            None => ()
        }

        match key_released {
            Some(k) => {
                match k {
                    VirtualKeyCode::Left => {
                        if self.color == 7 {self.color = 0} else {self.color += 1}
                        text_layer.pop_char();
                        text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Right => {
                        if self.color == 0 {self.color = 7} else {self.color -= 1}
                        text_layer.pop_char();
                        text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Up => {
                        if self.bkg_color == 7 {self.bkg_color = 0} else {self.bkg_color += 1}
                        text_layer.pop_char();
                        text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Down => {
                        if self.bkg_color == 0 {self.bkg_color = 7} else {self.bkg_color -= 1}
                        text_layer.pop_char();
                        text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::PageUp => {
                        text_layer.scroll_up();
        
                        if text_layer.get_characters().len() == 0 {
                            text_layer.push_char('_', self.color, self.bkg_color, false);
                        }
                    }

                    _ => () 
                }
            }
            None => ()
        }

        return None;

    }
}