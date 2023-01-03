use app_macro_derive::AppMacro;
use virtual_frame_buffer::characters_rom::CHAR_TABLE;

const SPLASH: &str = "\u{000D} Fantasy CPC Microcomputer V(0.1)\u{000D}\u{000D} 2022 Damien Torreilles\u{000D}\u{000D}";
const SHELL_START_MESSAGE: &str = "SHELL 0.1\u{000D}Ready\u{000D}";

const DEFAULT_BKG_COLOR: u8 = TRUE_BLUE;
const DEFAULT_COLOR: u8 = YELLOW;

#[derive(AppMacro)]
pub struct Shell {
    enable_auto_escape: bool,
    name: String,
    color: u8,
    bkg_color: u8,
    //last_character_received: Option<char>,
    clear_text_layer: bool,
    command: Vec<char>,
    // command_history: Vec<String>,
    updating: bool,
    drawing: bool,
    initialized: bool
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
        // let command_history: Vec<String> = Vec::new();

        Shell {
            enable_auto_escape: false,
            name: String::from("shell"),
            color: DEFAULT_COLOR,
            bkg_color: DEFAULT_BKG_COLOR,
            //last_character_received: None,
            clear_text_layer: false,
            command: Vec::new(),
            // command_history,
            updating: false,
            drawing: false,
            initialized: false
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
            StyledChar::Default(c) => TextLayerChar {c, color: self.color, bkg_color: self.bkg_color, swap: false, blink: false, shadowed: false},
            // StyledChar::Highlight(c) => TextLayerChar {c, color: self.color, bkg_color: self.bkg_color, swap: true, blink: false, shadowed: false},
            // StyledChar::Warning(c) => TextLayerChar {c, color: self.color, bkg_color: BLACK, swap: false, blink: false, shadowed: false},
            // StyledChar::Error(c) => TextLayerChar {c, color: RED, bkg_color: BLACK, swap: false, blink: true, shadowed: false}
        }
    }

    pub fn interpret_command(&mut self, command: String) -> AppResponse{

        let mut response: AppResponse = AppResponse::new();

        if command.len() > 0 {
            println!("Command: '{}'", command);
            if command == "help" {
                response.set_message(String::from("Type [clear] to clear screen.\u{000D}Type [quit] or [exit] to exit."));
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
                response.set_message(String::from(command));
            }
        }
        response
    }

    pub fn init_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_console_mut().pos_x = 0;
        virtual_frame_buffer.get_console_mut().pos_y = 0;
        virtual_frame_buffer.get_console_mut().set_col_count(TEXT_COLUMNS);
        virtual_frame_buffer.get_console_mut().set_row_count(TEXT_ROWS);
        virtual_frame_buffer.get_console_mut().clear();
        virtual_frame_buffer.get_console_mut().push_string(SPLASH);
        virtual_frame_buffer.get_console_mut().push_string(SHELL_START_MESSAGE);
        virtual_frame_buffer.get_console_mut().push_char('>');
    }

    pub fn update_app(
        &mut self,
        inputs: &WinitInputHelper,
        _clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {

        if self.clear_text_layer {
            virtual_frame_buffer.get_console_mut().clear();
            virtual_frame_buffer.get_console_mut().push_char('>');
            self.clear_text_layer = false;
        }

        if !inputs.text().is_empty() {
            match inputs.text().get(0) {
                Some(TextChar::Char(c)) => { 
                    if *c == unicode::ESCAPE {
                        virtual_frame_buffer.get_console_mut().push_char('\u{000D}');
                        virtual_frame_buffer.get_console_mut().push_string("Type 'quit' or 'exit' to quit Fantasy CPC.");
                        virtual_frame_buffer.get_console_mut().push_char('\u{000D}');
                        virtual_frame_buffer.get_console_mut().push_char('>');
                        self.command.clear();
                    } else {
                        self.command.push(*c);
                        virtual_frame_buffer.get_console_mut().push_text_layer_char(self.get_text_layer_char_from_style(self.style_a_char(*c, Style::Default)));   
                    }
                },
                Some(TextChar::Back) => { 
                    if !self.command.is_empty() {
                        self.command.pop();
                        virtual_frame_buffer.get_console_mut().push_char(unicode::BACKSPACE);
                    }
                },
                None => ()
            }
        }

        if inputs.key_released(VirtualKeyCode::Return) {
            let response = self.interpret_command(self.command.iter().cloned().collect::<String>());
            let message_string = response.get_message().clone();
            if message_string.is_some() {
                virtual_frame_buffer.get_console_mut().push_char('\u{000D}');
                virtual_frame_buffer.get_console_mut().push_string(&message_string.unwrap());
                virtual_frame_buffer.get_console_mut().push_char('\u{000D}');
                virtual_frame_buffer.get_console_mut().push_char('>');
            } else {
                virtual_frame_buffer.get_console_mut().push_char('\u{000D}');
                virtual_frame_buffer.get_console_mut().push_char('>');
            }
            self.command.clear();
            return Some(response);
        }

        return None;
    }

    pub fn draw_app(&mut self, _inputs: &WinitInputHelper, _clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.clear(WHITE);
        virtual_frame_buffer.get_console_mut().display = true;
    }
}
