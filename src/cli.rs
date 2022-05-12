use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::text_layer::TextLayer;
use std::io::{self, Write};

const SPLASH_1: &str = "************* FANTASY CPC *************";
const SPLASH_2: &str = "*              ROM v0.1               *";
const SPLASH_3: &str = "*       Damien Torreilles 2022        *";
const SPLASH_4: &str = "***************************************";
const SPLASH_5: &str = "Ready. Type 'help' for command list.";

const DEFAULT_BKG_COLOR: u8 = 4;
const DEFAULT_COLOR: u8 = 5;
const BUFFER_SIZE: usize = 100;

pub struct Cli {
    color: u8,
    bkg_color: u8,
    columns: u8,
    rows: u8,
    command: Vec<char>,
    pub running: bool,
    buffer: Vec<DisplayStyle>
}

pub trait Update {
    fn update (&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow>;
}

pub trait Draw {
    fn draw (&mut self, text_layer: &mut TextLayer);
}

enum DisplayStyle {
    Default(String),
    Message(String),
    Command(String),
    Highlight(String),
    Warning(String),
    Error(String)
}

impl Cli {

    pub fn new(text_layer: &mut TextLayer) -> Cli {

        let mut buffer: Vec<DisplayStyle> = Vec::new();
        buffer.push(DisplayStyle::Default(String::from(SPLASH_1)));
        buffer.push(DisplayStyle::Default(String::from(SPLASH_2)));
        buffer.push(DisplayStyle::Default(String::from(SPLASH_3)));
        buffer.push(DisplayStyle::Default(String::from(SPLASH_4)));
        buffer.push(DisplayStyle::Default(String::from(SPLASH_5)));
        buffer.push(DisplayStyle::Default(String::from("")));

        Cli {
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            columns: text_layer.get_columns() as u8,
            rows: text_layer.get_rows() as u8,
            command: Vec::new(),
            buffer,
            running: false,
        }
    }

    pub fn interpret_command (&mut self, command: String) -> Option<ControlFlow> {

        if command == "help" {
            self.buffer.push(DisplayStyle::Message(String::from("Type [clear] to clear screen.")));
            self.buffer.push(DisplayStyle::Message(String::from("Type [quit] or [exit] to exit.")));
            self.buffer.push(DisplayStyle::Message(String::from("Type [warning], [error] or [highlight] to display an example.")));
        } else if command == "clear" {
            self.buffer.clear();
            self.command.clear();
        } else if command == "warning" {
            self.buffer.push(DisplayStyle::Warning(String::from("This is a warning message")));
        } else if command == "error" {
            self.buffer.push(DisplayStyle::Error(String::from("This is an error message")));
        } else if command == "highlight" {
            self.buffer.push(DisplayStyle::Highlight(String::from("This is a highlighted message")));
        } else if command == "message" {
            self.buffer.push(DisplayStyle::Message(String::from("This is a message")));
        } else if command == "quit" || command == "exit"{
            println!("Command 'quit' or 'exit' received; stopping");
            return Some(ControlFlow::Exit);
        } else {
            self.buffer.push(DisplayStyle::Message(String::from("SYNTAX ERROR")));
        }

        return None;
    }
}

impl Update for Cli {

    fn update (&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow> {

        match character_received {
            Some(c) => {

                print!("{} ", c as u8);
                io::stdout().flush().unwrap();

                match c as u8 {
                    8 => { //Backspace
                        self.command.pop();
                    } 
                    
                    13 => { //Enter
                        let string_command: String = String::from_iter(self.command.iter());
                        let cleaned_string_command = string_command.trim().to_lowercase();
                        println!("Command: '{}'", cleaned_string_command);
                        self.buffer.push(DisplayStyle::Command(string_command));
                        self.command.clear();
                        return self.interpret_command(cleaned_string_command);
                    }
                    
                    27 => { //Escape
                        return Some(ControlFlow::Exit);
                    }
                    
                    _ => {
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
                        // if self.color == 7 {self.color = 0} else {self.color += 1}
                        // text_layer.pop_char();
                        // text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Right => {
                        // if self.color == 0 {self.color = 7} else {self.color -= 1}
                        // text_layer.pop_char();
                        // text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Up => {
                        // if self.bkg_color == 7 {self.bkg_color = 0} else {self.bkg_color += 1}
                        // text_layer.pop_char();
                        // text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::Down => {
                        // if self.bkg_color == 0 {self.bkg_color = 7} else {self.bkg_color -= 1}
                        // text_layer.pop_char();
                        // text_layer.push_char('_', self.color, self.bkg_color, false);
                    }
        
                    VirtualKeyCode::PageUp => {
                        // text_layer.scroll_up();
        
                        // if text_layer.get_characters().len() == 0 {
                        //     text_layer.push_char('_', self.color, self.bkg_color, false);
                        // }
                    }

                    _ => () 
                }
            }
            None => ()
        }

        return None;
    }
}

impl Draw for Cli {
    fn draw (&mut self, text_layer: &mut TextLayer) {

        text_layer.clear();

        for line in self.buffer.chunks_exact_mut(1) {

            match &line[0] {
                DisplayStyle::Default(text) => {
                    text_layer.push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Command(text) => {
                    text_layer.push_char('>', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                    text_layer.push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Message(text) => {
                    text_layer.push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Highlight(text) => {
                    text_layer.push_string_line(&text, 3, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Warning(text) => {
                    text_layer.push_string("[WARNING]", DEFAULT_BKG_COLOR, DEFAULT_COLOR, false);
                    text_layer.push_char(' ', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                    text_layer.push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Error(text) => {
                    text_layer.push_string("[ERROR] ", 0, 2, false);
                    text_layer.push_string_line(&text, 0, 2, false);
                }
            }
        }

        text_layer.push_char('>', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
        for c in self.command.chunks_exact_mut(1) {
            text_layer.push_char(c[0], DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
        }
        text_layer.push_char('_', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
    }
}