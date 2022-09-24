use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::text_layer::TextLayerChar;
use std::io::{self, Write};
use crate::process::*;
use crate::virtual_frame_buffer::VirtualFrameBuffer;
use crate::unicode;
use f8b_app_macro::F8bAppMacro;
use f8b_app_macro_derive::F8bAppMacro;

const DEFAULT_BKG_COLOR: u8 = 7;
const DEFAULT_COLOR: u8 = 0;

#[derive(F8bAppMacro)]
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
            ended: false
        }
    }

    pub fn update(&mut self, character_received: Option<char>, key_pressed_os: Option<VirtualKeyCode>, key_released: Option<VirtualKeyCode>) -> ProcessResponse {

        let mut response = ProcessResponse::new();

        if !self.started {
            self.start();
            self.started = true;
        }

        match character_received {
            Some(c) => {
                match c {
                    unicode::BACKSPACE => {
                        self.buffer.pop();
                    } 
                    
                    unicode::ENTER => {
                    }
                    
                    unicode::ESCAPE => {
                    }
                    
                    _ => {
                        let plop: TextLayerChar = TextLayerChar {
                            unicode: c,
                            color: self.selected_color,
                            background_color: self.selected_bkg_color,
                            blink: false,
                            flipp: false
                        };
                        
                        self.buffer.push(plop);
                    }
                }

            }
            None => ()
        }

        match key_released {
            Some(k) => {
                match k {
                    VirtualKeyCode::Left => {
                        if self.selected_color == 31 {self.selected_color = 0} else {self.selected_color += 1}
                    }
        
                    VirtualKeyCode::Right => {
                        if self.selected_color == 0 {self.selected_color = 31} else {self.selected_color -= 1}
                    }
        
                    VirtualKeyCode::Up => {
                        if self.selected_bkg_color == 31 {self.selected_bkg_color = 0} else {self.selected_bkg_color += 1}
                    }
        
                    VirtualKeyCode::Down => {
                        if self.selected_bkg_color == 0 {self.selected_bkg_color = 31} else {self.selected_bkg_color -= 1}
                    }
        
                    VirtualKeyCode::PageUp => {
                        //self.text_layer.scroll_up();
                    }

                    _ => () 
                }
            }
            None => ()
        }

        return response;
    }

    pub fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {

        virtual_frame_buffer.get_text_layer().clear();
        virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);

        for text_layer_char in self.buffer.chunks_exact_mut(1) {
            virtual_frame_buffer.get_text_layer().push_character(Some(text_layer_char[0]));
        }

        virtual_frame_buffer.get_text_layer().push_char('_', Some(self.selected_color), Some(self.selected_bkg_color), false);
    }
}