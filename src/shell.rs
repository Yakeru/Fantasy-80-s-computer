use winit::{event::VirtualKeyCode,event_loop::{ControlFlow,EventLoopProxy}};
use crate::virtual_frame_buffer::VirtualFrameBuffer;
use crate::process::*;
use crate::text_edit::*;
use crate::sprite_editor::*;
use crate::text_layer::TextLayerChar;
use crate::color_palettes::*;

const SHELL_START_MESSAGE: &str = "Shell ready.\u{000D}Type [help] for command list.\u{000D}";

const DEFAULT_BKG_COLOR: ColorPalette = ColorPalette::TrueBlue;
const DEFAULT_COLOR: ColorPalette = ColorPalette::Yellow;

pub struct Shell {
    name: String,
    color: ColorPalette,
    bkg_color: ColorPalette,
    last_character_received: Option<char>,
    clear_text_layer: bool,
    command: Vec<char>,
    display_buffer: Vec<char>,
    history_buffer: Vec<char>,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    apps: Vec<Box<dyn Process>>
}

enum Style {
    Default,
    Highlight,
    Warning,
    Error
}

impl Shell {

    pub fn new() -> Shell {

        let display_buffer: Vec<char> = Vec::new();
        let history_buffer: Vec<char> = Vec::new();
        let mut apps: Vec<Box<dyn Process>> = Vec::new();
        
        Shell {
            name: String::from("shell"),
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            last_character_received: None,
            clear_text_layer: false,
            command: Vec::new(),
            display_buffer,
            history_buffer,
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            apps
        }
    }

    fn push_string(&mut self, string: &str) {
        for c in string.chars() {
            self.display_buffer.push(c);
        }
    }

    pub fn interpret_command(&mut self, command: String)  -> ProcessResponse {

        let mut response: ProcessResponse = ProcessResponse::new();

        if command.len() > 0 {
            println!("Command: '{}'", command);
            if command == "help" {
                self.push_string("Type [clear] to clear screen.\u{000D}");
                self.push_string("Type [quit] or [exit] to exit.\u{000D}");
                self.push_string("Type [ps] to list loaded processes.\u{000D}");
            } else if command == "clear" {
                self.display_buffer.clear();
                self.command.clear();
                self.clear_text_layer = true;
            } else if command == "ps" {
                self.push_string("Name,  Updating,  Drawing\u{000D}");
                self.push_string(&format!("{},  {},  {}\u{000D}", self.name, self.updating, self.drawing));
                // for app in self.apps {
                //     self.push_string(&format!("{},  {},  {}\n", app.get_name() , app.get_state().0, app.get_state().1));
                // }  
            } else if command == "quit" || command == "exit"{
                response.event = Some(ControlFlow::Exit);
                response.set_message(String::from("\u{000D}Command 'quit' or 'exit' received; stopping.\u{000D}"));
                println!("Command 'quit' or 'exit' received; stopping");
            }
        }

        response
    }
}

impl Process for Shell {

    fn start(&mut self) {
        self.push_string(SHELL_START_MESSAGE);
    }

    fn end(&mut self) {
        self.started = false;
        self.drawing = false;
        self.updating = false;
        self.ended = true;
    }

    fn update(&mut self, character_received: Option<char>, key_pressed_os: Option<VirtualKeyCode>, key_released: Option<VirtualKeyCode>) -> ProcessResponse {

        let mut response = ProcessResponse::new();
        self.last_character_received = character_received;

        if !self.started {
            self.start();
            self.started = true;
        }

        match character_received {
            Some(unicode) => {
                match unicode {
                    '\u{0008}' => { //Backspace
                        self.command.pop();
                    } 
                    
                    '\u{000D}' => { //Enter
                        let string_command: String = String::from_iter(self.command.iter());
                        let cleaned_string_command = string_command.trim().to_lowercase();
                        response = self.interpret_command(cleaned_string_command);
                        self.command.clear();
                    }
                    
                    '\u{001B}' => { //Escape
                        response.event = Some(ControlFlow::Exit);
                        response.set_message(String::from("Command 'quit' or 'exit' received; stopping"));
                        println!("Command 'quit' or 'exit' received; stopping");
                        return response;
                    }
                    
                    _ => {
                        self.command.push(unicode);
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

        return response;
    }

    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        match self.last_character_received {
            Some(c) => {
                virtual_frame_buffer.get_text_layer().push_char(c, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
            }

            None => ()
        }

        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        for c in &self.display_buffer {
            virtual_frame_buffer.get_text_layer().push_char(*c, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
        }
        self.display_buffer.clear();

        if self.clear_text_layer {
            virtual_frame_buffer.get_text_layer().clear();
            self.clear_text_layer = false;
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_state(&mut self, updating: bool, drawing: bool) {
        self.updating = updating;
        self.drawing = drawing;

        if drawing {self.updating = true}
        if !updating {self.drawing = false}
    }

    fn get_state(&self) -> (bool, bool) {
        return (self.updating, self.drawing)
    }
}