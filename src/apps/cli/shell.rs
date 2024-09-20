use fantasy_cpc_app_trait::{AppResponse, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{
    characters_rom::CHAR_TABLE,
    color_palettes::{BLUE, TRUE_BLUE, YELLOW},
    config::{TEXT_COLUMNS, TEXT_ROWS},
    text_layer::TextLayerChar,
    DisplayController,
};
use winit::{event::VirtualKeyCode, event_loop::ControlFlow};
use winit_input_helper::{TextChar, WinitInputHelper};

use super::terminal::Terminal;

const SPLASH: &str = "\u{000D} Fantasy CPC Microcomputer V(0.6.0)\u{000D}\u{000D} 2023 Damien Torreilles\u{000D}\u{000D}";
const SHELL_START_MESSAGE: &str = "SHELL 0.1\u{000D}Ready\u{000D}";

const DEFAULT_BKG_COLOR: usize = TRUE_BLUE;
const DEFAULT_COLOR: usize = YELLOW;

pub struct Shell {
    app_params: FantasyCppAppDefaultParams,
    color: usize,
    bkg_color: usize,
    clear_text_layer: bool,
    command: Vec<char>,
    // command_history: Vec<String>,
    terminal: Terminal,
}

#[derive(Copy, Clone)]
enum StyledChar {
    Default(char),
    //     Highlight(char),
    //     Warning(char),
    //     Error(char),
}

#[derive(Copy, Clone)]
enum Style {
    Default,
    // Highlight,
    // Warning,
    // Error,
}

impl Shell {
    pub fn new() -> Shell {
        Self {
            app_params: FantasyCppAppDefaultParams::new(String::from("shell"), false),
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            //last_character_received: None,
            clear_text_layer: false,
            command: Vec::new(),
            // command_history,
            terminal: Terminal::new(),
        }
    }

    fn style_a_char(&self, c: char, style: Style) -> StyledChar {
        match style {
            Style::Default => StyledChar::Default(c),
            // Style::Highlight => StyledChar::Highlight(c),
            // Style::Warning => StyledChar::Warning(c),
            // Style::Error => StyledChar::Error(c),
        }
    }

    fn get_text_layer_char_from_style(&self, style: StyledChar) -> TextLayerChar {
        match style {
            StyledChar::Default(c) => TextLayerChar {
                c,
                color: self.color,
                bkg_color: self.bkg_color,
                swap: false,
                blink: false,
                shadowed: false,
            },
            // StyledChar::Highlight(c) => TextLayerChar {c, color: self.color, bkg_color: self.bkg_color, swap: true, blink: false, shadowed: false},
            // StyledChar::Warning(c) => TextLayerChar {c, color: self.color, bkg_color: BLACK, swap: false, blink: false, shadowed: false},
            // StyledChar::Error(c) => TextLayerChar {c, color: RED, bkg_color: BLACK, swap: false, blink: true, shadowed: false}
        }
    }

    fn interpret_command(&mut self, command: String) -> AppResponse {
        let mut response: AppResponse = AppResponse::new();

        if !command.is_empty() {
            println!("Command: '{}'", command);
            if command == "help" {
                response.set_message(String::from(
                    "Type [clear] to clear screen.\u{000D}Type [quit] or [exit] to exit.",
                ));
            } else if command == "clear" {
                self.command.clear();
                self.clear_text_layer = true;
            } else if command == "ps" {
            } else if command == "test" {
                let mut toto = Vec::new();
                for char in CHAR_TABLE {
                    toto.push(char);
                }
                let titi = toto.iter().cloned().collect::<String>();
                response.set_message(titi);
            } else if command == "quit" || command == "exit" {
                response.event = Some(ControlFlow::Exit);
                response.set_message(String::from("Command 'quit' or 'exit' received; stopping."));
            } else {
                response.set_message(command);
            }
        }
        response
    }
}

impl FantasyCpcApp for Shell {
    fn get_app_params(&mut self) -> &mut fantasy_cpc_app_trait::FantasyCppAppDefaultParams {
        &mut self.app_params
    }

    fn init_app(&mut self, _system_clock: &Clock, display_controller: &mut DisplayController) {
        display_controller.set_brightness(255);
        display_controller.clear(BLUE);
        self.terminal.set_coordinates((0, 0));
        self.terminal.set_size((TEXT_COLUMNS, TEXT_ROWS));
        self.terminal.clear();
        self.terminal.push_string(SPLASH);
        self.terminal.push_string(SHELL_START_MESSAGE);
        self.terminal.push_char('>');
    }

    fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
    ) -> Option<AppResponse> {
        inputs?;

        if self.clear_text_layer {
            self.terminal.clear();
            self.terminal.push_char('>');
            self.clear_text_layer = false;
        }

        if !inputs.unwrap().text().is_empty() {
            match inputs.unwrap().text().first() {
                Some(TextChar::Char(c)) => {
                    if *c == unicode::ESCAPE {
                        self.terminal.push_char('\u{000D}');
                        self.terminal
                            .push_string("Type 'quit' or 'exit' to quit Fantasy CPC.");
                        self.terminal.push_char('\u{000D}');
                        self.terminal.push_char('>');
                        self.command.clear();
                    } else {
                        self.command.push(*c);
                        self.terminal.push_text_layer_char(
                            self.get_text_layer_char_from_style(
                                self.style_a_char(*c, Style::Default),
                            ),
                        );
                    }
                }
                Some(TextChar::Back) => {
                    if !self.command.is_empty() {
                        self.command.pop();
                        self.terminal.push_char(unicode::BACKSPACE);
                    }
                }
                None => (),
            }
        }

        if inputs.unwrap().key_pressed_os(VirtualKeyCode::Return) {
            let response = self.interpret_command(self.command.iter().cloned().collect::<String>());
            let message_string = response.get_message().clone();
            if let Some(content) = message_string {
                self.terminal.push_char('\u{000D}');
                self.terminal.push_string(&content);
                self.terminal.push_char('\u{000D}');
                self.terminal.push_char('>');
            } else {
                self.terminal.push_char('\u{000D}');
                self.terminal.push_char('>');
            }
            self.command.clear();
            return Some(response);
        }

        None
    }

    fn draw_app(&mut self, _clock: &Clock, display_controller: &mut DisplayController) {
        self.terminal.render(display_controller);
    }
}
