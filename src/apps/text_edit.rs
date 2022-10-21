use app_macro::*;
use app_macro_derive::AppMacro;

use crate::text_layer::TextLayerChar;
use crate::unicode;
use crate::virtual_frame_buffer::VirtualFrameBuffer;
use std::io::{self, Write};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::ControlFlow,
};

const DEFAULT_BKG_COLOR: u8 = 7;
const DEFAULT_COLOR: u8 = 0;

#[derive(AppMacro)]
pub struct TextEdit {
    name: String,
    selected_color: u8,
    selected_bkg_color: u8,
    columns: u8,
    rows: u8,
    buffer: Vec<TextLayerChar>,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool,
}

impl TextEdit {
    pub fn new() -> TextEdit {
        let buffer = Vec::new();

        TextEdit {
            name: String::from("textEdit"),
            selected_color: DEFAULT_COLOR,
            selected_bkg_color: DEFAULT_BKG_COLOR,
            columns: 0,
            rows: 0,
            buffer,
            updating: false,
            drawing: false,
            started: false,
            ended: false,
        }
    }

    pub fn update(
        &mut self,
        keybord_input: Option<KeyboardInput>,
        char_received: Option<char>,
    ) -> AppResponse {
        let mut response = AppResponse::new();

        if !self.started {
            self.start();
            self.started = true;
        }

        match char_received {
            Some(c) => match c {
                unicode::BACKSPACE => {
                    self.buffer.pop();
                }

                unicode::ENTER => {}

                unicode::ESCAPE => {}

                _ => {
                    let plop: TextLayerChar = TextLayerChar {
                        unicode: c,
                        color: self.selected_color,
                        background_color: self.selected_bkg_color,
                        blink: false,
                        flipp: false,
                    };

                    self.buffer.push(plop);
                }
            },
            None => (),
        }

        match keybord_input {
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

                                VirtualKeyCode::Escape => {
                                    //Escape
                                    response.set_message("Escape key pressed".to_string());
                                    response.event = Some(ControlFlow::ExitWithCode(0));
                                    self.end();
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

        return response;
    }

    pub fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
        virtual_frame_buffer.get_text_layer().clear();
        virtual_frame_buffer.get_text_layer().show_cursor = false;
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);

        for text_layer_char in self.buffer.chunks_exact_mut(1) {
            virtual_frame_buffer
                .get_text_layer()
                .push_character(Some(text_layer_char[0]));
        }

        virtual_frame_buffer.get_text_layer().push_char(
            '_',
            Some(self.selected_color),
            Some(self.selected_bkg_color),
            false,
        );
    }
}
