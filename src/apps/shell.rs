// use crate::text_layer::TextLayerChar;
use crate::unicode;
use virtual_frame_buffer::*;
use app_macro::*;
use app_macro_derive::AppMacro;
use virtual_frame_buffer::color_palettes::*;
use winit::event::KeyboardInput;
use winit::event_loop::ControlFlow;

const SPLASH: &str = "Fantasy CPC Microcomputer V(0.1)\u{000D}\u{000D} 2022 Damien Torreilles\u{000D}\u{000D}";
const SHELL_START_MESSAGE: &str = "SHELL 0.1\u{000D}\u{000D}Ready\u{000D}";

const DEFAULT_BKG_COLOR: u8 = TRUEBLUE.0;
const DEFAULT_COLOR: u8 = YELLOW.0;

#[derive(AppMacro)]
pub struct Shell {
    name: String,
    // console: Console,
    color: u8,
    bkg_color: u8,
    last_character_received: Option<char>,
    clear_text_layer: bool,
    command: Vec<char>,
    command_history: Vec<String>,
    display_buffer: Vec<StyledChar>,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
}

#[derive(Copy, Clone)]
enum StyledChar {
    Default(char),
    Highlight(char),
    Warning(char),
    Error(char),
}

#[derive(Copy, Clone)]
enum Style {
    Default,
    Highlight,
    Warning,
    Error,
}

impl Shell {
    pub fn new() -> Shell {
        let display_buffer: Vec<StyledChar> = Vec::new();
        let command_history: Vec<String> = Vec::new();

        Shell {
            name: String::from("shell"),
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            last_character_received: None,
            clear_text_layer: false,
            command: Vec::new(),
            display_buffer,
            command_history,
            updating: false,
            drawing: false,
            started: false,
            ended: false,
        }
    }

    fn style_a_char(&self, c: char, style: Style) -> StyledChar {
        match style {
            Style::Default => StyledChar::Default(c),
            Style::Highlight => StyledChar::Highlight(c),
            Style::Warning => StyledChar::Warning(c),
            Style::Error => StyledChar::Error(c),
        }
    }

    fn get_text_layer_char_from_style(&self, style: StyledChar) -> (char, u8, u8, bool, bool, bool) {
        match style {
            StyledChar::Default(c) => (c, self.color, self.bkg_color, false, false, false),
            StyledChar::Highlight(c) => (c, self.color, self.bkg_color, true, false, false),
            StyledChar::Warning(c) => (c, YELLOW.0, BLACK.0, false, false, false),
            StyledChar::Error(c) => (c, RED.0, BLACK.0, false, true, false),
        }
    }

    fn push_string(&mut self, string: &str, style: Style) {
        for c in string.chars() {
            self.display_buffer.push(self.style_a_char(c, style));
        }
    }

    fn push_char(&mut self, c: StyledChar) {
        self.display_buffer.push(c);
    }

    pub fn interpret_command(&mut self, command: String) -> AppResponse {
        let mut response: AppResponse = AppResponse::new();

        if command.len() > 0 {
            println!("Command: '{}'", command);
            if command == "help" {
                self.push_string("Type [clear] to clear screen.\u{000D}", Style::Default);
                self.push_string("Type [quit] or [exit] to exit.\u{000D}", Style::Default);
                self.push_string(
                    "Type [ps] to list loaded processes.\u{000D}",
                    Style::Default,
                );
            } else if command == "clear" {
                self.display_buffer.clear();
                self.command.clear();
                self.clear_text_layer = true;
            } else if command == "ps" {
                // self.push_string("Name,  Updating,  Drawing\u{000D}", Style::Default);
                // self.push_string(&format!("{},  {},  {}\u{000D}", self.name, self.updating, self.drawing), Style::Default);
                // for app in self.apps {
                //     self.push_string(&format!("{},  {},  {}\u{000D}", *app.get_name() , *app.get_state().0, *app.get_state().1), Style::Default);
                // }
            } else if command == "warning" {
                self.push_string("[WARNING]!", Style::Warning);
                self.push_string("this is a warning.\u{000D}", Style::Default);
            } else if command == "error" {
                self.push_string("[ERROR]", Style::Error);
                self.push_string("this is an error.\u{000D}", Style::Default);
            } else if command == "highlight" {
                self.push_string("[ERROR]", Style::Highlight);
                self.push_string("this is a highlight.\u{000D}", Style::Default);
            } else if command == "quit" || command == "exit" {
                response.event = Some(ControlFlow::Exit);
                response.set_message(String::from("Command 'quit' or 'exit' received; stopping."));
                println!("Command 'quit' or 'exit' received; stopping");
            } else {
                self.push_string("Syntax Error\u{000D}", Style::Default);
            }
        }
        self.push_char(StyledChar::Default('>'));
        response
    }

    pub fn start(&mut self) {
        self.push_string(SPLASH, Style::Default);
        self.push_string(SHELL_START_MESSAGE, Style::Default);
        self.push_char(StyledChar::Default('>'));
        self.started = true;
    }

    pub fn update_app(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
    ) -> AppResponse {
        let mut response = AppResponse::new();
        self.last_character_received = char_received;

        if !self.started {
            self.start();
        }

        match self.last_character_received {
            Some(unicode) => {
                match unicode {
                    unicode::BACKSPACE => {
                        //Dont delete further than prompt
                        if self.command.len() == 0 {
                            self.last_character_received = None;
                        }
                        self.command.pop();
                    }

                    unicode::ENTER => {
                        let string_command: String = String::from_iter(self.command.iter());
                        let cleaned_string_command = string_command.trim().to_lowercase();
                        response = self.interpret_command(cleaned_string_command);
                        self.command.clear();
                        self.command_history.push(string_command);
                    }

                    unicode::ESCAPE => {
                        //response.event = Some(ControlFlow::Exit);
                        self.push_string("\u{000D}Type 'quit' or 'exit' to quit Fantasy CPC\u{000D}", Style::Default);
                        response.set_message(String::from(
                            "Type 'quit' or 'exit' to quit Fantasy CPC",
                        ));
                        return response;
                    }

                    _ => {
                        self.command.push(unicode);
                    }
                }
            }
            None => (),
        }

        match keybord_input {
            Some(_k) => (),
            None => (),
        }

        return response;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        if self.clear_text_layer {
            virtual_frame_buffer.get_text_layer_mut().clear();
            self.clear_text_layer = false;
            self.last_character_received = None;
        }

        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);

        match self.last_character_received {
            Some(c) => {
                let text_layer_char = self.get_text_layer_char_from_style(self.style_a_char(c, Style::Default));
                virtual_frame_buffer.get_text_layer_mut().push_char(text_layer_char.0, Some(text_layer_char.1), Some(text_layer_char.2), 
                text_layer_char.3, text_layer_char.4, text_layer_char.5);
            }

            None => ()
        }

        for c in &self.display_buffer {
            let text_layer_char = self.get_text_layer_char_from_style(*c);
            virtual_frame_buffer
                .get_text_layer_mut().push_char(text_layer_char.0, Some(text_layer_char.1), Some(text_layer_char.2), 
                    text_layer_char.3, text_layer_char.4, text_layer_char.5);
        }

        self.display_buffer.clear();
    }
}
