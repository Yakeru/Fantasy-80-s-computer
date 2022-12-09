use app_macro::*;
use app_macro_derive::AppMacro;
use crate::unicode;
use virtual_frame_buffer::*;
use winit::event::{ElementState, VirtualKeyCode};

const DEFAULT_BKG_COLOR: u8 = 7;
const DEFAULT_COLOR: u8 = 0;

#[derive(AppMacro)]
pub struct TextEdit {
    enable_auto_escape: bool,
    name: String,
    selected_color: u8,
    selected_bkg_color: u8,
    buffer: Vec<(char, u8, u8)>,
    updating: bool,
    drawing: bool,
    initialized: bool
}

impl TextEdit {
    pub fn new() -> TextEdit {
        let buffer = Vec::new();

        TextEdit {
            enable_auto_escape: true,
            name: String::from("textEdit"),
            selected_color: DEFAULT_COLOR,
            selected_bkg_color: DEFAULT_BKG_COLOR,
            buffer,
            updating: false,
            drawing: false,
            initialized: false
        }
    }

    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {}

    pub fn update_app(
        &mut self,
        app_message: AppMessage,
        _virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {
        let response = AppResponse::new();

        match app_message.char_received {
            Some(c) => match c {
                unicode::BACKSPACE => {
                    self.buffer.pop();
                }

                unicode::ENTER => {}

                unicode::ESCAPE => {}

                _ => {
                    let plop: (char, u8, u8) = (c, self.selected_color, self.selected_bkg_color);
                    self.buffer.push(plop);
                }
            },
            None => (),
        }

        match app_message.keyboard_input {
            Some(k) => {
                if k.state == ElementState::Pressed {
                    match k.virtual_keycode {
                        Some(code) => {
                            match code {
                                VirtualKeyCode::Left => {
                                    if self.selected_color == 31 {
                                        self.selected_color = 0
                                    } else {
                                        self.selected_color += 1
                                    }
                                }

                                VirtualKeyCode::Right => {
                                    if self.selected_color == 0 {
                                        self.selected_color = 31
                                    } else {
                                        self.selected_color -= 1
                                    }
                                }

                                VirtualKeyCode::Up => {
                                    if self.selected_bkg_color == 31 {
                                        self.selected_bkg_color = 0
                                    } else {
                                        self.selected_bkg_color += 1
                                    }
                                }

                                VirtualKeyCode::Down => {
                                    if self.selected_bkg_color == 0 {
                                        self.selected_bkg_color = 31
                                    } else {
                                        self.selected_bkg_color -= 1
                                    }
                                }

                                VirtualKeyCode::PageUp => {
                                    //self.text_layer.scroll_up();
                                }

                                _ => (),
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }

        return Some(response);
    }

    pub fn draw_app(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
        virtual_frame_buffer.get_console_mut().display = false;

        let mut count = 0;
        for text_layer_char in self.buffer.chunks_exact_mut(1) {
            virtual_frame_buffer
                .get_text_layer_mut()
                .insert_char(count, text_layer_char[0].0, Some(text_layer_char[0].1), Some(text_layer_char[0].2), false, false, false);
                count = count + 1;
        }

        virtual_frame_buffer.get_text_layer_mut().insert_char(self.buffer.len(), '_', Some(self.selected_color), Some(self.selected_bkg_color), false, false, false);
    }
}
