use winit::{event::VirtualKeyCode,event_loop::{ControlFlow,EventLoopProxy}};
use crate::virtual_frame_buffer::VirtualFrameBuffer;
use crate::app::*;

const SPLASH_1: &str = "************* FANTASY CPC *************";
const SPLASH_2: &str = "*              ROM v0.1               *";
const SPLASH_3: &str = "*       Damien Torreilles 2022        *";
const SPLASH_4: &str = "***************************************";
const SPLASH_5: &str = "Ready. Type 'help' for command list.";

const DEFAULT_BKG_COLOR: u8 = 28;
const DEFAULT_COLOR: u8 = 10;
const BUFFER_SIZE: usize = 100;

pub struct Cli {
    pub app: App,
    color: u8,
    bkg_color: u8,
    command: Vec<char>,
    buffer: Vec<DisplayStyle>
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

    pub fn new(pid: usize) -> Cli {

        let buffer: Vec<DisplayStyle> = Vec::new();
        let app = App::new(String::from("Yak's CPC CLI"), pid);

        Cli {
            app,
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            command: Vec::new(),
            buffer,
        }
    }

    pub fn interpret_command(&mut self, command: String) -> Option<ControlFlow> {
        if command.len() > 0 {
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
        }

        return None;
    }
}

impl Update for Cli {

    fn start(&mut self) {
        self.buffer.push(DisplayStyle::Default(String::from(SPLASH_1)));
        self.buffer.push(DisplayStyle::Default(String::from(SPLASH_2)));
        self.buffer.push(DisplayStyle::Default(String::from(SPLASH_3)));
        self.buffer.push(DisplayStyle::Default(String::from(SPLASH_4)));
        self.buffer.push(DisplayStyle::Default(String::from(SPLASH_5)));
        self.buffer.push(DisplayStyle::Default(String::from("")));
    }

    fn end(&mut self) {
        self.app.started = false;
        self.app.drawing = false;
        self.app.updating = false;
        self.app.ended = true;
    }

    fn update(&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow> {

        if !self.app.started {
            self.start();
            self.app.started = true;
        }

        match character_received {
            Some(c) => {
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
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        virtual_frame_buffer.get_text_layer().clear();
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);

        for line in self.buffer.chunks_exact_mut(1) {

            match &line[0] {
                DisplayStyle::Default(text) => {
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Command(text) => {
                    virtual_frame_buffer.get_text_layer().push_char('>', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Message(text) => {
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Highlight(text) => {
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, 3, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Warning(text) => {
                    virtual_frame_buffer.get_text_layer().push_string("[WARNING]", DEFAULT_BKG_COLOR, DEFAULT_COLOR, false);
                    virtual_frame_buffer.get_text_layer().push_char(' ', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                }
                DisplayStyle::Error(text) => {
                    virtual_frame_buffer.get_text_layer().push_string("[ERROR] ", 0, 2, false);
                    virtual_frame_buffer.get_text_layer().push_string_line(&text, 0, 2, false);
                }
            }
        }

        virtual_frame_buffer.get_text_layer().push_char('>', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
        for c in self.command.chunks_exact_mut(1) {
            virtual_frame_buffer.get_text_layer().push_char(c[0], DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
        }
        virtual_frame_buffer.get_text_layer().push_char('_', DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
    }
}

// #[derive(Debug, Clone, Copy)]
// enum CustomEvent {
//     Grr,
// }

// impl SendEvent for Cli {
//     fn send_event(plop: &EventLoopProxy<()>) {
//         plop.send_event(CustomEvent::Grr).ok();
//     }
// }