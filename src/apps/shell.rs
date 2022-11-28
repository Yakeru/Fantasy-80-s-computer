// use crate::text_layer::TextLayerChar;
use crate::{unicode, genrate_random_garbage};
use virtual_frame_buffer::*;
use app_macro::*;
use app_macro_derive::AppMacro;
use virtual_frame_buffer::color_palettes::*;
use virtual_frame_buffer::console::Console;
use virtual_frame_buffer::text_layer_char::TextLayerChar;
use winit::event::KeyboardInput;
use winit::event_loop::ControlFlow;

const SPLASH: &str = "Fantasy CPC Microcomputer V(0.1)\u{000D}\u{000D}2022 Damien Torreilles\u{000D}\u{000D}";
const SHELL_START_MESSAGE: &str = "SHELL 0.1\u{000D}\u{000D}Ready\u{000D}";

const DEFAULT_BKG_COLOR: u8 = TRUEBLUE;
const DEFAULT_COLOR: u8 = YELLOW;

#[derive(AppMacro)]
pub struct Shell {
    name: String,
    color: u8,
    bkg_color: u8,
    last_character_received: Option<char>,
    clear_text_layer: bool,
    command: Vec<char>,
    command_history: Vec<String>,
    console: Console,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
    garbage: bool
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
            console: {
                let cursor = TextLayerChar {c: '\u{25AE}', color: YELLOW, bkg_color: TRUEBLUE, swap: false, blink: true, shadowed: false};
                Console::new(0, 0, 30, 10, YELLOW, TRUEBLUE, cursor, false, true)
            },
            command_history,
            updating: false,
            drawing: false,
            started: false,
            ended: false,
            garbage: true
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

    fn get_text_layer_char_from_style(&self, style: StyledChar) -> TextLayerChar {
        match style {
            StyledChar::Default(c) => TextLayerChar {c, color: self.color, bkg_color: self.bkg_color, swap: false, blink: false, shadowed: false},
            StyledChar::Highlight(c) => TextLayerChar {c, color: self.color, bkg_color: self.bkg_color, swap: true, blink: false, shadowed: false},
            StyledChar::Warning(c) => TextLayerChar {c, color: self.color, bkg_color: BLACK, swap: false, blink: false, shadowed: false},
            StyledChar::Error(c) => TextLayerChar {c, color: RED, bkg_color: BLACK, swap: false, blink: true, shadowed: false}
        }
    }

    // fn push_string(&mut self, string: &str, style: Style) {
    //     for c in string.chars() {
    //         self.display_buffer.push(self.style_a_char(c, style));
    //     }
    // }

    // fn push_char(&mut self, c: StyledChar) {
    //     self.display_buffer.push(c);
    // }

    // pub fn interpret_command(&mut self, command: String) -> AppResponse {
    //     let mut response: AppResponse = AppResponse::new();

    //     if command.len() > 0 {
    //         println!("Command: '{}'", command);
    //         if command == "help" {
    //             self.push_string("Type [clear] to clear screen.\u{000D}", Style::Default);
    //             self.push_string("Type [quit] or [exit] to exit.\u{000D}", Style::Default);
    //             self.push_string(
    //                 "Type [ps] to list loaded processes.\u{000D}",
    //                 Style::Default,
    //             );
    //         } else if command == "clear" {
    //             self.display_buffer.clear();
    //             self.command.clear();
    //             self.clear_text_layer = true;
    //         } else if command == "ps" {
    //             // self.push_string("Name,  Updating,  Drawing\u{000D}", Style::Default);
    //             // self.push_string(&format!("{},  {},  {}\u{000D}", self.name, self.updating, self.drawing), Style::Default);
    //             // for app in self.apps {
    //             //     self.push_string(&format!("{},  {},  {}\u{000D}", *app.get_name() , *app.get_state().0, *app.get_state().1), Style::Default);
    //             // }
    //         } else if command == "warning" {
    //             self.push_string("[WARNING]!", Style::Warning);
    //             self.push_string("this is a warning.\u{000D}", Style::Default);
    //         } else if command == "error" {
    //             self.push_string("[ERROR]", Style::Error);
    //             self.push_string("this is an error.\u{000D}", Style::Default);
    //         } else if command == "highlight" {
    //             self.push_string("[highlighted text]", Style::Highlight);
    //             self.push_string("this is a highlight.\u{000D}", Style::Default);
    //         } else if command == "quit" || command == "exit" {
    //             response.event = Some(ControlFlow::Exit);
    //             response.set_message(String::from("Command 'quit' or 'exit' received; stopping."));
    //         } else {
    //             self.push_string("Syntax Error\u{000D}", Style::Default);
    //         }
    //     }
    //     self.push_char(StyledChar::Default('>'));
    //     response
    // }

    // pub fn start(&mut self) {
    //     // self.push_string(SPLASH, Style::Default);
    //     // self.push_string(SHELL_START_MESSAGE, Style::Default);
    //     // self.push_char(StyledChar::Default('>'));
    //     // self.started = true;
    // }

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
                        self.console.content.pop();
                    }

                    unicode::ESCAPE => {
                        response.event = Some(ControlFlow::Exit);
                        // self.push_string("\u{000D}Type 'quit' or 'exit' to quit Fantasy CPC\u{000D}", Style::Default);
                        // self.push_char(StyledChar::Default('>'));
                        // response.set_message(String::from(
                        //     "Type 'quit' or 'exit' to quit Fantasy CPC",
                        // ));
                        // return response;
                    }

                    _ => {
                        self.console.content.push(self.get_text_layer_char_from_style(self.style_a_char(unicode, Style::Default)))
                    }
                }
            }
            None => (),
        }

        match keybord_input {
            Some(k) => {
                match k.virtual_keycode {
                    Some(code) => {
                        match code {
                            winit::event::VirtualKeyCode::Left => self.console.pos_x = self.console.pos_x - 1,
                            winit::event::VirtualKeyCode::Right => self.console.pos_x = self.console.pos_x + 1,
                            winit::event::VirtualKeyCode::Up => self.console.pos_y = self.console.pos_y - 1,
                            winit::event::VirtualKeyCode::Down => self.console.pos_y = self.console.pos_y + 1,
                            _ => ()
                        }
                    },
                    None => ()
                } 
            },
            None => (),
        }

        return response;
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.clear_frame_buffer(WHITE);
        if self.garbage {
            genrate_random_garbage(virtual_frame_buffer);
            self.garbage = false;
        }
        virtual_frame_buffer.console_renderer(&self.console);
    }
}
