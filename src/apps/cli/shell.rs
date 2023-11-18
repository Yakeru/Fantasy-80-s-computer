use fantasy_cpc_app_trait::{AppMessage, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::{
    characters_rom::CHAR_TABLE,
    color_palettes::{BLACK, BLUE, DARK_ORANGE, RED, TRUE_BLUE, YELLOW},
    config::{TEXT_COLUMNS, TEXT_ROWS},
    text_layer::TextCellStyle,
    DisplayController,
};
use unicode::ENTER;
use winit::event::VirtualKeyCode;
use winit_input_helper::{TextChar, WinitInputHelper};

use super::terminal::Terminal;

const SPLASH: &str = "\u{000D} Fantasy CPC Microcomputer V(0.7.0)\u{000D}\u{000D} 2023 Damien Torreilles\u{000D}\u{000D}";
const SHELL_START_MESSAGE: &str = "SHELL 0.2\u{000D}Ready\u{000D}";

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

pub const DEFAULT_TERM_STYLE: TextCellStyle = TextCellStyle {
    color: YELLOW,
    bkg_color: TRUE_BLUE,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub const HIGHLIGHT_TERM_STYLE: TextCellStyle = TextCellStyle {
    color: YELLOW,
    bkg_color: TRUE_BLUE,
    swap_color: true,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub const WARNING_TERM_STYLE: TextCellStyle = TextCellStyle {
    color: DARK_ORANGE,
    bkg_color: TRUE_BLUE,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub const ERROR_TERM_STYLE: TextCellStyle = TextCellStyle {
    color: RED,
    bkg_color: TRUE_BLUE,
    swap_color: false,
    blink: false,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

pub const CRITICAL_TERM_STYLE: TextCellStyle = TextCellStyle {
    color: RED,
    bkg_color: BLACK,
    swap_color: false,
    blink: true,
    shadowed: false,
    flip_h: false,
    flip_v: false,
};

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
            terminal: Terminal::new((0, 0), (TEXT_COLUMNS, TEXT_ROWS)),
        }
    }

    // If it's a command recognized by the Shell it will run it internally and eventually
    // dispay stuff in the terminal.
    // If the command is not recognized by the shell it will transfer it to the main loop as
    // a system message. If the main loop recognizes the message (quit, reboot, app name, ... ) it will execute it.
    fn interpret_command(&mut self, command: String) -> Option<AppMessage> {
        if !command.is_empty() {
            println!("Command: '{}'", command);
            if command.to_lowercase() == "help" {
                self.terminal.push_string(
                    "Type [clear] to clear screen.\u{000D}Type [quit] or [exit] to exit.\u{000D}",
                    None,
                );
            } else if command.to_lowercase() == "clear" {
                self.command.clear();
                self.clear_text_layer = true;
            } else if command.to_lowercase() == "test" {
                let mut all_chars_vec = Vec::new();
                for char in CHAR_TABLE {
                    all_chars_vec.push(char);
                }
                let mut all_chars = all_chars_vec.iter().cloned().collect::<String>();
                all_chars.insert(all_chars.len(), '\u{000D}');
                self.terminal.push_string(&all_chars, None);
                self.terminal
                    .push_string("Highlighted message\u{000D}", Some(HIGHLIGHT_TERM_STYLE));
                self.terminal
                    .push_string("Warning !\u{000D}", Some(WARNING_TERM_STYLE));
                self.terminal
                    .push_string("ERROR !!\u{000D}", Some(ERROR_TERM_STYLE));
                self.terminal
                    .push_string("CRITICAL !!!\u{000D}", Some(CRITICAL_TERM_STYLE));
            } else {
                return Some(AppMessage::System(command));
            }
        }

        None
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
        self.terminal.push_string(SPLASH, None);
        self.terminal.push_string(SHELL_START_MESSAGE, None);
        self.terminal.push_char('>', None);
    }

    fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        messages: Option<Vec<AppMessage>>,
        _clock: &Clock,
    ) -> Option<Vec<AppMessage>> {
        inputs?;

        // Clear shell and terminal if last command was a clear
        if self.clear_text_layer {
            self.terminal.clear();
            self.terminal.push_char('>', None);
            self.clear_text_layer = false;
        }

        // Display all the messages sent to the shell
        if let Some(messages) = messages {
            for message in messages {
                match message {
                    fantasy_cpc_app_trait::AppMessage::Standard(text) => {
                        self.terminal.push_string(&text, Some(DEFAULT_TERM_STYLE))
                    }
                    fantasy_cpc_app_trait::AppMessage::Highlight(text) => {
                        self.terminal.push_string(&text, Some(HIGHLIGHT_TERM_STYLE))
                    }
                    fantasy_cpc_app_trait::AppMessage::Warning(text) => {
                        self.terminal.push_string(&text, Some(WARNING_TERM_STYLE))
                    }
                    fantasy_cpc_app_trait::AppMessage::Error(text) => {
                        self.terminal.push_string(&text, Some(ERROR_TERM_STYLE))
                    }
                    fantasy_cpc_app_trait::AppMessage::Critical(text) => {
                        self.terminal.push_string(&text, Some(CRITICAL_TERM_STYLE))
                    }
                    _ => (),
                }
                self.terminal.push_char(ENTER, None);
            }
        }

        // Parse the last inputs
        // If it's the escape key, we display a message
        // If it's a char, it's added to the command
        // if it's backspace, we pop a char from the command
        if !inputs.unwrap().text().is_empty() {
            match inputs.unwrap().text().get(0) {
                Some(TextChar::Char(c)) => {
                    if *c == unicode::ESCAPE && self.command.len() == 0 {
                        self.terminal.push_char(ENTER, None);
                        self.terminal
                            .push_string("Type 'quit' or 'exit' to quit Fantasy CPC.", None);
                        self.terminal.push_char(ENTER, None);
                        self.terminal.push_char('>', None);
                        self.command.clear();
                    } else {
                        self.command.push(*c);
                        self.terminal.push_char(*c, None);
                    }
                }
                Some(TextChar::Back) => {
                    if !self.command.is_empty() {
                        self.command.pop();
                        self.terminal.push_char(unicode::BACKSPACE, None);
                    }
                }
                None => (),
            }
        }

        // If it's enter, we send the command to the interpret_command method
        if inputs.unwrap().key_pressed_os(VirtualKeyCode::Return) {
            self.terminal.push_char(ENTER, None);

            let response = self.interpret_command(self.command.iter().cloned().collect::<String>());

            self.terminal.push_char('>', None);
            self.command.clear();

            if response.is_some() {
                return Some(vec![response.unwrap()]);
            }
        }

        None
    }

    fn draw_app(&mut self, _clock: &Clock, dc: &mut DisplayController) {
        self.terminal.render(dc.get_txt_mut());
    }
}
